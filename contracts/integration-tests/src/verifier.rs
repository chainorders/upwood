use crate::identity_registry::{IIdentityRegistryContract, NATIONALITY_ATTRIBUTE_TAG};
use concordium_rwa_identity_registry::{identities::RegisterIdentityParams, types::*};
use concordium_smart_contract_testing::*;

pub struct Verifier<T: IIdentityRegistryContract> {
    pub account:           Account,
    pub identity_registry: T,
}

impl<T: IIdentityRegistryContract> Verifier<T> {
    pub fn register_nationalities(
        &self,
        chain: &mut Chain,
        nationalities: Vec<(Address, String)>,
    ) -> Result<Vec<ContractInvokeSuccess>, ContractInvokeError> {
        nationalities
            .iter()
            .map(|(address, nationality)| {
                self.identity_registry.register_identity().update(
                    chain,
                    &self.account,
                    &RegisterIdentityParams {
                        address:  *address,
                        identity: Identity {
                            attributes:  vec![IdentityAttribute {
                                tag:   NATIONALITY_ATTRIBUTE_TAG,
                                value: nationality.to_owned(),
                            }],
                            credentials: vec![],
                        },
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()
    }
}
