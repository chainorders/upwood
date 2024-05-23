use super::{cis2_test_contract::ICis2Contract, test_contract_client::GenericReceive};
use concordium_cis2::TransferParams;
use concordium_protocols::{
    concordium_cis2_ext::{IsTokenAmount, IsTokenId},
    concordium_cis2_security::{
        BurnParams, FreezeParams, FrozenParams, FrozenResponse, IsPausedResponse, PauseParams,
        RecoverParam,
    },
};
use concordium_std::Address;

pub trait ICis2SecurityTestContract<T: IsTokenId, A: IsTokenAmount, TEvent>:
    ICis2Contract<T, A, TEvent> {
    fn add_agent(&self) -> GenericReceive<Address, (), TEvent> {
        GenericReceive::<Address, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "addAgent",
            self.max_energy(),
        )
    }

    fn agents(&self) -> GenericReceive<(), Vec<Address>, TEvent> {
        GenericReceive::<(), Vec<Address>, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "agents",
            self.max_energy(),
        )
    }

    fn remove_agent(&self) -> GenericReceive<Address, (), TEvent> {
        GenericReceive::<Address, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "removeAgent",
            self.max_energy(),
        )
    }

    fn is_agent(&self) -> GenericReceive<Address, bool, TEvent> {
        GenericReceive::<Address, bool, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "isAgent",
            self.max_energy(),
        )
    }

    fn forced_transfer(&self) -> GenericReceive<TransferParams<T, A>, (), TEvent> {
        GenericReceive::<TransferParams<T, A>, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "forcedTransfer",
            self.max_energy(),
        )
    }

    fn pause(&self) -> GenericReceive<PauseParams<T>, (), TEvent> {
        GenericReceive::<PauseParams<T>, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "pause",
            self.max_energy(),
        )
    }

    fn un_pause(&self) -> GenericReceive<PauseParams<T>, (), TEvent> {
        GenericReceive::<PauseParams<T>, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "unPause",
            self.max_energy(),
        )
    }

    fn is_paused(&self) -> GenericReceive<PauseParams<T>, IsPausedResponse, TEvent> {
        GenericReceive::<PauseParams<T>, IsPausedResponse, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "isPaused",
            self.max_energy(),
        )
    }

    fn burn(&self) -> GenericReceive<BurnParams<T, A>, (), TEvent> {
        GenericReceive::<BurnParams<T, A>, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "burn",
            self.max_energy(),
        )
    }

    fn freeze(&self) -> GenericReceive<FreezeParams<T, A>, (), TEvent> {
        GenericReceive::<FreezeParams<T, A>, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "freeze",
            self.max_energy(),
        )
    }

    fn un_freeze(&self) -> GenericReceive<FreezeParams<T, A>, (), TEvent> {
        GenericReceive::<FreezeParams<T, A>, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "unFreeze",
            self.max_energy(),
        )
    }

    fn balance_of_frozen(&self) -> GenericReceive<FrozenParams<T>, FrozenResponse<A>, TEvent> {
        GenericReceive::<FrozenParams<T>, FrozenResponse<A>, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "balanceOfFrozen",
            self.max_energy(),
        )
    }

    fn balance_of_un_frozen(&self) -> GenericReceive<FrozenParams<T>, FrozenResponse<A>, TEvent> {
        GenericReceive::<FrozenParams<T>, FrozenResponse<A>, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "balanceOfUnFrozen",
            self.max_energy(),
        )
    }

    fn recover(&self) -> GenericReceive<RecoverParam, (), TEvent> {
        GenericReceive::<RecoverParam, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "recover",
            self.max_energy(),
        )
    }

    fn recovery_address(&self) -> GenericReceive<Address, Option<Address>, TEvent> {
        GenericReceive::<Address, Option<Address>, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "recoveryAddress",
            self.max_energy(),
        )
    }
}
