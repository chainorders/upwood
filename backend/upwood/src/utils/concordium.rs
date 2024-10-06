pub mod identity {
    use concordium_rust_sdk::id::constants::ArCurve;
    use concordium_rust_sdk::id::id_proof_types::AtomicStatement;
    use concordium_rust_sdk::id::types::AttributeTag;
    use concordium_rust_sdk::web3id::did::Network;
    use concordium_rust_sdk::web3id::{
        get_public_data, CredentialLookupError, CredentialProof, PresentationVerificationError,
        Web3IdAttribute,
    };
    use concordium_rust_sdk::{cis4, constants, v2};
    use sha2::Digest;

    pub type Presentation = concordium_rust_sdk::web3id::Presentation<ArCurve, Web3IdAttribute>;
    pub type GlobalContext = concordium_rust_sdk::id::types::GlobalContext<ArCurve>;
    pub type IdStatement = Vec<AtomicStatement<ArCurve, AttributeTag, Web3IdAttribute>>;
    /// Generate a challenge for the user by hashing the user id
    pub fn generate_challenge(user_id: &str) -> [u8; constants::SHA256] {
        let mut hasher = sha2::Sha256::new();
        hasher.update(user_id);
        let result = hasher.finalize();
        result.into()
    }

    pub enum VerifyPresentationError {
        CredentialLookup,
        InActiveCredential,
        PresentationVerification,
        InvalidChallenge,
    }
    impl From<CredentialLookupError> for VerifyPresentationError {
        fn from(_: CredentialLookupError) -> Self { VerifyPresentationError::CredentialLookup }
    }
    impl From<PresentationVerificationError> for VerifyPresentationError {
        fn from(_: PresentationVerificationError) -> Self {
            VerifyPresentationError::PresentationVerification
        }
    }

    pub struct VerifyPresentationResponse {
        pub revealed_attributes: Vec<(AttributeTag, Web3IdAttribute)>,
    }

    /// # Verifies that
    /// 1. All the Web3 Id Credentials are active
    /// 2. The challenge in the presentation matches the one sent
    /// 3. The presentation is valid
    /// # Returns
    /// A list of the revealed id attributes
    pub async fn verify_presentation(
        network: Network,
        concordium_client: &mut v2::Client,
        global_context: &GlobalContext,
        presentation: &Presentation,
        sent_challenge: [u8; constants::SHA256],
    ) -> std::result::Result<VerifyPresentationResponse, VerifyPresentationError> {
        let public_data = get_public_data(
            concordium_client,
            network,
            presentation,
            v2::BlockIdentifier::LastFinal,
        )
        .await?;
        if !public_data
            .iter()
            .all(|cm| matches!(cm.status, cis4::CredentialStatus::Active))
        {
            return Err(VerifyPresentationError::InActiveCredential);
        }
        let ver_res =
            presentation.verify(global_context, public_data.iter().map(|cm| &cm.inputs))?;
        if ver_res.challenge != sent_challenge.into() {
            return Err(VerifyPresentationError::InvalidChallenge);
        }
        let revealed_id_attributes = get_revealed_id_attributes(presentation);
        Ok(VerifyPresentationResponse {
            revealed_attributes: revealed_id_attributes,
        })
    }

    /// Retrieves the revealed identity attributes from a given
    /// `VerifyPresentationResponse`.
    ///
    /// # Arguments
    ///
    /// * `response` - A reference to the `VerifyPresentationResponse` from which to
    ///   retrieve the attributes.
    ///
    /// # Returns
    ///
    /// * A Result containing a `HashMap` where the keys are the attribute names and
    ///   the values are the revealed attributes, or an `Error`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `VerifyPresentationResponse` does
    /// not contain the expected data or if there is a problem decoding the
    /// attributes.
    fn get_revealed_id_attributes(
        presentation: &Presentation,
    ) -> Vec<(AttributeTag, Web3IdAttribute)> {
        presentation
            .verifiable_credential
            .iter()
            .filter_map(|vc| match vc {
                CredentialProof::Account { proofs, .. } => Some(
                    proofs
                        .iter()
                        .filter_map(|proof| {
                            match proof {
                        (
                            AtomicStatement::RevealAttribute { statement },
                            concordium_rust_sdk::id::id_proof_types::AtomicProof::RevealAttribute {
                                attribute,
                                ..
                            },
                        ) => Some((statement.attribute_tag, attribute.clone())),
                        _ => None,
                    }
                        })
                        .collect::<Vec<_>>(),
                ),
                CredentialProof::Web3Id { .. } => None,
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

pub mod account {
    use concordium_rust_sdk::base::contracts_common::AccountSignatures;
    use concordium_rust_sdk::id::types::AccountAddress;
    use concordium_rust_sdk::types::WalletAccount;

    pub trait Signer {
        fn wallet(&self) -> &WalletAccount;

        fn address(&self) -> AccountAddress { self.wallet().address }

        /// Signs a hash of the given bytes using the wallet account's sign key.
        fn sign(&self, bytes: &[u8]) -> AccountSignatures {
            let wallet_account = self.wallet();
            wallet_account.keys.sign_message(bytes)
        }
    }
}
