use concordium_smart_contract_testing::*;
use concordium_std::{Deserial, Serial};

fn to_signer(owner: &Account) -> Signer {
    Signer::with_keys(owner.keys.num_keys()).expect("Creating Signer")
}

/// Generic Init
pub struct GenericInit<TIn = (), TEvent = ()> {
    pub module_ref:    ModuleReference,
    pub contract_name: OwnedContractName,
    _phantom_in:       std::marker::PhantomData<TIn>,
    _phantom_event:    std::marker::PhantomData<TEvent>,
}

impl<TIn, TEvent> GenericInit<TIn, TEvent> {
    pub fn new(module_ref: ModuleReference, contract_name: &str) -> Self {
        Self {
            module_ref,
            contract_name: OwnedContractName::new_unchecked(contract_name.to_owned()),
            _phantom_in: std::marker::PhantomData,
            _phantom_event: std::marker::PhantomData,
        }
    }
}

impl<TIn: Serial, TEvent> GenericInit<TIn, TEvent> {
    pub fn init_payable(
        &self,
        chain: &mut Chain,
        owner: &Account,
        param: &TIn,
        amount: Amount,
    ) -> Result<ContractInitSuccess, ContractInitError> {
        let param = OwnedParameter::from_serial(param).expect("Serializing Init Params");

        chain.contract_init(
            to_signer(owner),
            owner.address,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: self.module_ref,
                amount,
                init_name: self.contract_name.to_owned(),
                param,
            },
        )
    }

    pub fn init(
        &self,
        chain: &mut Chain,
        owner: &Account,
        param: &TIn,
    ) -> Result<ContractInitSuccess, ContractInitError> {
        self.init_payable(chain, owner, param, Amount::zero())
    }
}

impl<TIn, TEvent: Deserial> GenericInit<TIn, TEvent> {
    pub fn parse_events(
        &self,
        res: ContractInitSuccess,
    ) -> Result<Vec<TEvent>, concordium_std::ParseError> {
        res.events.iter().map(|e| e.parse()).collect()
    }
}

impl<TEvent> GenericInit<(), TEvent> {
    pub fn init_payable_without_params(
        &self,
        chain: &mut Chain,
        owner: &Account,
        amount: Amount,
    ) -> Result<ContractInitSuccess, ContractInitError> {
        chain.contract_init(
            to_signer(owner),
            owner.address,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: self.module_ref,
                amount,
                init_name: self.contract_name.to_owned(),
                param: OwnedParameter::empty(),
            },
        )
    }

    pub fn init_without_params(
        &self,
        chain: &mut Chain,
        owner: &Account,
    ) -> Result<ContractInitSuccess, ContractInitError> {
        self.init_payable_without_params(chain, owner, Amount::zero())
    }
}

/// Generic Receive
pub struct GenericReceive<TIn, TOut, TEvent> {
    pub contract_name:    OwnedContractName,
    pub entrypoint_name:  OwnedEntrypointName,
    pub max_energy:       Energy,
    pub contract_address: ContractAddress,
    _phantom_in:          std::marker::PhantomData<TIn>,
    _phantom_out:         std::marker::PhantomData<TOut>,
    _phantom_event:       std::marker::PhantomData<TEvent>,
}

impl<TIn, TOut, TEvent> GenericReceive<TIn, TOut, TEvent> {
    pub fn new(
        contract_address: ContractAddress,
        contract_name: OwnedContractName,
        entrypoint_name: &str,
        max_energy: Energy,
    ) -> Self {
        Self {
            contract_address,
            contract_name,
            entrypoint_name: OwnedEntrypointName::new_unchecked(entrypoint_name.to_owned()),
            max_energy,
            _phantom_in: std::marker::PhantomData,
            _phantom_out: std::marker::PhantomData,
            _phantom_event: std::marker::PhantomData,
        }
    }

    pub fn receive_name(&self) -> OwnedReceiveName {
        OwnedReceiveName::construct_unchecked(
            self.contract_name.as_contract_name(),
            self.entrypoint_name.as_entrypoint_name(),
        )
    }
}

impl<TIn: Serial, TOut, TEvent> GenericReceive<TIn, TOut, TEvent> {
    pub fn update_payable(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &TIn,
        amount: Amount,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            to_signer(sender),
            sender.address,
            Address::Account(sender.address),
            self.max_energy,
            self.update_payload(amount, params),
        )
    }

    pub fn invoke_payable(
        &self,
        chain: &mut Chain,
        invoker: &Account,
        params: &TIn,
        amount: Amount,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_invoke(
            invoker.address,
            Address::Account(invoker.address),
            self.max_energy,
            self.update_payload(amount, params),
        )
    }

    pub fn update(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &TIn,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        self.update_payable(chain, sender, params, Amount::zero())
    }

    pub fn invoke(
        &self,
        chain: &mut Chain,
        invoker: &Account,
        params: &TIn,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        self.invoke_payable(chain, invoker, params, Amount::zero())
    }

    fn update_payload(&self, amount: Amount, params: &TIn) -> UpdateContractPayload {
        UpdateContractPayload {
            amount,
            receive_name: self.receive_name(),
            address: self.contract_address,
            message: OwnedParameter::from_serial(params).expect("Serializing Params"),
        }
    }
}

impl<TOut, TEvent> GenericReceive<(), TOut, TEvent> {
    pub fn update_payable_without_params(
        &self,
        chain: &mut Chain,
        sender: &Account,
        amount: Amount,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            to_signer(sender),
            sender.address,
            Address::Account(sender.address),
            self.max_energy,
            self.update_payload_without_params(amount),
        )
    }

    pub fn invoke_payable_without_params(
        &self,
        chain: &mut Chain,
        invoker: AccountAddress,
        sender: Address,
        amount: Amount,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_invoke(
            invoker,
            sender,
            self.max_energy,
            self.update_payload_without_params(amount),
        )
    }

    pub fn update_without_params(
        &self,
        chain: &mut Chain,
        sender: &Account,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        self.update_payable_without_params(chain, sender, Amount::zero())
    }

    pub fn invoke_without_params(
        &self,
        chain: &mut Chain,
        invoker: AccountAddress,
        sender: Address,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        self.invoke_payable_without_params(chain, invoker, sender, Amount::zero())
    }

    fn update_payload_without_params(&self, amount: Amount) -> UpdateContractPayload {
        UpdateContractPayload {
            amount,
            receive_name: self.receive_name(),
            address: self.contract_address,
            message: OwnedParameter::empty(),
        }
    }
}

impl<TIn, TOut, TEvent: Deserial> GenericReceive<TIn, TOut, TEvent> {
    pub fn parse_events(
        &self,
        res: &ContractInvokeSuccess,
    ) -> Result<Vec<TEvent>, concordium_std::ParseError> {
        res.events()
            .filter(|(contract_address, _)| self.contract_address.eq(contract_address))
            .flat_map(|e| e.1)
            .map(|e| e.parse())
            .collect()
    }
}

impl<TIn, TOut: Deserial, TEvent> GenericReceive<TIn, TOut, TEvent> {
    pub fn parse_return_value(
        &self,
        res: &ContractInvokeSuccess,
    ) -> Result<TOut, concordium_std::ParseError> {
        res.parse_return_value()
    }
}

pub trait ITestContract {
    fn contract_name() -> OwnedContractName;
    fn contract_address(&self) -> ContractAddress;
    fn max_energy(&self) -> Energy { Energy::from(30000) }
}

pub trait ITestModule {
    fn module_path(&self) -> String;

    fn module_ref(&self) -> ModuleReference {
        module_load_v1(self.module_path()).expect("Loading Module").get_module_ref()
    }

    fn deploy(
        &self,
        chain: &mut Chain,
        owner: &Account,
    ) -> Result<ModuleReference, ModuleDeployError> {
        let module = module_load_v1(self.module_path()).expect("Loading module");
        chain
            .module_deploy_v1(
                Signer::with_keys(owner.keys.num_keys()).expect("Creating deploy Signer"),
                owner.address,
                module,
            )
            .map(|r| r.module_reference)
    }
}
