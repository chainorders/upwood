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
    use poem_openapi::Object;
    use serde::{Deserialize, Serialize};
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
        FirstNameNotProvided,
        LastNameNotProvided,
        NationalityNotProvided,
    }
    impl From<CredentialLookupError> for VerifyPresentationError {
        fn from(_: CredentialLookupError) -> Self { VerifyPresentationError::CredentialLookup }
    }
    impl From<PresentationVerificationError> for VerifyPresentationError {
        fn from(_: PresentationVerificationError) -> Self {
            VerifyPresentationError::PresentationVerification
        }
    }

    #[derive(Object, Serialize, Deserialize)]
    pub struct VerifyPresentationResponse {
        pub first_name:  String,
        pub last_name:   String,
        pub nationality: String,
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
        let first_name = {
            revealed_id_attributes
                .iter()
                .find_map(|(tag, attr)| {
                    if "firstName".parse::<AttributeTag>().unwrap().eq(tag) {
                        match attr {
                            Web3IdAttribute::String(s) => Some(s.0.clone()),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .ok_or(VerifyPresentationError::FirstNameNotProvided)?
        };
        let last_name = {
            revealed_id_attributes
                .iter()
                .find_map(|(tag, attr)| {
                    if "lastName".parse::<AttributeTag>().unwrap().eq(tag) {
                        match attr {
                            Web3IdAttribute::String(s) => Some(s.0.clone()),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .ok_or(VerifyPresentationError::LastNameNotProvided)?
        };
        let nationality = {
            revealed_id_attributes
                .iter()
                .find_map(|(tag, attr)| {
                    if "nationality".parse::<AttributeTag>().unwrap().eq(tag) {
                        match attr {
                            Web3IdAttribute::String(s) => Some(s.0.clone()),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .ok_or(VerifyPresentationError::NationalityNotProvided)?
        };
        Ok(VerifyPresentationResponse {
            first_name,
            last_name,
            nationality,
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
    use std::collections::BTreeMap;

    use concordium_rust_sdk::common::types::{CredentialIndex, KeyIndex};
    use concordium_rust_sdk::common::Versioned;
    use concordium_rust_sdk::id::curve_arithmetic::Curve;
    use concordium_rust_sdk::id::types::{
        AccountAddress, AccountCredentialWithoutProofs, Attribute,
    };
    use concordium_rust_sdk::types::WalletAccount;
    use concordium_rust_sdk::v2::{self, AccountIdentifier, BlockIdentifier, QueryError};
    use sha2::Digest;

    pub trait Signer {
        fn wallet(&self) -> &WalletAccount;

        fn address(&self) -> AccountAddress { self.wallet().address }

        /// Signs a hash of the given bytes using the wallet account's sign key.
        fn sign(
            &self,
            bytes: &[u8],
        ) -> concordium_rust_sdk::base::contracts_common::AccountSignatures {
            let wallet_account = self.wallet();
            wallet_account.keys.sign_message(bytes)
        }
    }

    /// Errors that can occur during generating or verifying account signatures.
    #[derive(thiserror::Error, Debug)]
    pub enum SignatureError {
        #[error("Network error: {0}")]
        QueryError(#[from] QueryError),
        #[error(
            "Indices do not exist on chain for credential index `{credential_index}` and key \
             index `{key_index}`"
        )]
        MissingIndicesOnChain {
            credential_index: u8,
            key_index:        u8,
        },
        #[error("The indices in the maps do not match")]
        MismatchMapIndices,
        #[error(
            "The public key and the private key in the `account_keys` map do not match for \
             credential index `{credential_index}` and key index `{key_index}`"
        )]
        MismatchPublicPrivateKeys {
            credential_index: u8,
            key_index:        u8,
        },
        #[error("The signature is invalid")]
        InvalidSignatureHex,
    }

    /// Account signatures are constructed similarly to transaction signatures. The
    /// only difference is that account signatures use as account nonce the value 0
    /// when signing. This is to ensure that the user does not accidentally sign a
    /// valid transaction. As such account signatures can be used to sign messages
    /// in wallets that can then be verified in a smart contract or by a backend.
    ///
    /// Account signatures should be thought of as a nested map, indexed on the
    /// outer layer by credential indices, and the inner map maps key indices to
    /// [`Signature`]s.
    pub type AccountSignatures = BTreeMap<u8, CredentialSignatures>;
    pub type CredentialSignatures = BTreeMap<u8, Signature>;
    /// A single signature. Using the same binary and JSON serialization as the
    /// Haskell counterpart. In particular this means encoding the length as 2
    /// bytes, and thus the largest size is 65535 bytes.
    pub type Signature = String;

    /// Verify a given message was signed by the given account.
    /// Concretely this means:
    /// - enough credential holders signed the message.
    /// - each of the credential signatures has the required number of signatures.
    /// - all of the signatures are valid, that is, it is not sufficient that a
    ///   threshold number are valid, and some extra signatures included are
    ///   invalid.
    pub async fn verify_account_signature(
        mut client: v2::Client,
        signer: AccountAddress,
        signatures: &AccountSignatures,
        message: impl AsRef<[u8]>,
        bi: BlockIdentifier,
    ) -> Result<bool, SignatureError> {
        let message_hash = calculate_message_hash(message, signer);

        let signer_account_info = client
            .get_account_info(&AccountIdentifier::Address(signer), bi)
            .await?;

        let signer_account_credentials = signer_account_info.response.account_credentials;
        let credential_signatures_threshold = signer_account_info.response.account_threshold;

        // Ensure that all key indices in the signatures map exist on chain but not vice
        // versa. This allows us to iterate through the `signer_account_credentials` map
        // to get the public keys and thresholds from the on-chain information
        // further down while still knowing that each signature in the signatures map
        // has a corresponding entry on chain.
        check_signature_map_key_indices_on_chain(signatures, &signer_account_credentials)?;

        let mut valid_credential_signatures_count = 0u8;
        for (credential_index, credential) in signer_account_credentials {
            // Retrieve the public key and threshold from the on-chain information.
            let (keys, signatures_threshold) = match credential.value {
                AccountCredentialWithoutProofs::Initial { icdv } => {
                    (icdv.cred_account.keys, icdv.cred_account.threshold)
                }
                AccountCredentialWithoutProofs::Normal { cdv, .. } => {
                    (cdv.cred_key_info.keys, cdv.cred_key_info.threshold)
                }
            };

            let mut valid_signatures_count = 0u8;

            for (key_index, public_key) in keys {
                // If a signature exists for the given credential and key index, verify it and
                // increase the `valid_signatures_count`.
                let Some(cred_sigs) = signatures.get(&credential_index.index) else {
                    continue;
                };

                let Some(signature) = cred_sigs.get(&key_index.0) else {
                    continue;
                };
                if public_key.verify(
                    message_hash,
                    &concordium_rust_sdk::common::types::Signature {
                        sig: hex::decode(signature)
                            .map_err(|_| SignatureError::InvalidSignatureHex)?,
                    },
                ) {
                    // If the signature is valid, increase the `valid_signatures_count`.
                    valid_signatures_count += 1;
                } else {
                    // If any signature is invalid, return `false`.
                    return Ok(false);
                }
            }

            // Check if the number of valid signatures meets the required threshold
            // so that this credential counts as having a valid credential signature.
            if valid_signatures_count >= signatures_threshold.into() {
                valid_credential_signatures_count += 1;
            }
        }

        // Check if the total number of valid credential signatures meets the required
        // threshold.
        Ok(valid_credential_signatures_count >= credential_signatures_threshold.into())
    }

    /// Calculate the message hash that is signed in Concordium wallets.
    pub fn calculate_message_hash(message: impl AsRef<[u8]>, signer: AccountAddress) -> [u8; 32] {
        // A message signed in a Concordium wallet is prepended with the
        // `account` address (signer) and 8 zero bytes. Accounts in a Concordium
        // wallet can either sign a regular transaction (in that case the
        // prepend is `account` address and the nonce of the account which is by
        // design >= 1) or sign a message (in that case the prepend is `account`
        // address and 8 zero bytes). Hence, the 8 zero bytes ensure that the user
        // does not accidentally sign a transaction. The account nonce is of type
        // u64 (8 bytes).
        let mut hasher = sha2::Sha256::new();
        hasher.update(signer);
        hasher.update([0u8; 8]);
        hasher.update(message);
        hasher.finalize().into()
    }

    /// Check that all key indices in the signatures map exist on chain but not vice
    /// versa.
    fn check_signature_map_key_indices_on_chain<C: Curve, AttributeType: Attribute<C::Scalar>>(
        signatures: &AccountSignatures,
        on_chain_credentials: &BTreeMap<
            CredentialIndex,
            Versioned<AccountCredentialWithoutProofs<C, AttributeType>>,
        >,
    ) -> Result<(), SignatureError> {
        // Ensure all outer-level keys in the signatures map exist in the
        // on_chain_credentials map.
        for (outer_key, inner_map) in signatures {
            // Check if the outer_key exists in the on_chain_credentials map.
            let on_chain_cred = on_chain_credentials
                .get(&CredentialIndex { index: *outer_key })
                .ok_or(SignatureError::MissingIndicesOnChain {
                    credential_index: *outer_key,
                    // The key_index does not exist in this context, use the default value.
                    key_index:        0u8,
                })?;

            // Ensure that the inner-level keys in the signatures map exist in the
            // on_chain_credentials map.
            for inner_key in inner_map.keys() {
                let map = match &on_chain_cred.value {
                    AccountCredentialWithoutProofs::Initial { icdv } => &icdv.cred_account.keys,
                    AccountCredentialWithoutProofs::Normal { cdv, .. } => &cdv.cred_key_info.keys,
                };

                if !map.contains_key(&KeyIndex(*inner_key)) {
                    return Err(SignatureError::MissingIndicesOnChain {
                        credential_index: *outer_key,
                        key_index:        *inner_key,
                    });
                }
            }
        }
        Ok(())
    }
}

pub mod chain {
    use concordium_rust_sdk::v2::{self, BlockIdentifier};

    use super::identity::GlobalContext;

    pub async fn concordium_global_context(concordium_client: &mut v2::Client) -> GlobalContext {
        concordium_client
            .get_cryptographic_parameters(BlockIdentifier::LastFinal)
            .await
            .expect("Failed to get concordium cryptographic parameters")
            .response
    }
}
