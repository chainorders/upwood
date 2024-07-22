pub mod verifier_challenges {
    use crate::schema::{self, verifier_challenges::dsl::*};
    use bigdecimal::BigDecimal;
    use chrono::{NaiveDateTime, Utc};
    use concordium_rust_sdk::{id::types::AccountAddress, types::ContractAddress};
    use diesel::{
        dsl::*,
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection},
    };

    type Conn = PooledConnection<ConnectionManager<PgConnection>>;
    type Result<T> = std::result::Result<T, diesel::result::Error>;

    pub async fn find_challenge(
        conn: &mut Conn,
        for_account: &AccountAddress,
        verifier: &AccountAddress,
        identity_registry: &ContractAddress,
    ) -> Result<Option<[u8; 32]>> {
        let for_accnt_str = for_account.0.to_vec();
        let verifier_str = verifier.0.to_vec();
        let db_challenge: Option<Vec<u8>> = verifier_challenges
            .filter(
                account_address
                    .eq(for_accnt_str)
                    .and(verifier_account_address.eq(verifier_str))
                    .and(identity_registry_index.eq::<BigDecimal>(identity_registry.index.into()))
                    .and(
                        identity_registry_sub_index
                            .eq::<BigDecimal>(identity_registry.subindex.into()),
                    ),
            )
            .select(challenge)
            .get_result::<Vec<u8>>(conn)
            .optional()?;
        let ret: Option<[u8; 32]> = db_challenge
            .map(|c| c.try_into().expect("could not de serialize challenge stored in db"));

        Ok(ret)
    }

    #[derive(Insertable)]
    #[diesel(table_name = schema::verifier_challenges)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct ChallengeInsert {
        pub account_address:             Vec<u8>,
        pub verifier_account_address:    Vec<u8>,
        pub identity_registry_index:     BigDecimal,
        pub identity_registry_sub_index: BigDecimal,
        pub challenge:                   Vec<u8>,
        pub create_time:                 NaiveDateTime,
    }

    impl ChallengeInsert {
        pub fn new(
            accnt: &AccountAddress,
            verifier_accnt: &AccountAddress,
            identity_registry: &ContractAddress,
            db_challenge: [u8; 32],
        ) -> Self {
            ChallengeInsert {
                account_address:             accnt.0.into(),
                verifier_account_address:    verifier_accnt.0.into(),
                identity_registry_index:     identity_registry.index.into(),
                identity_registry_sub_index: identity_registry.subindex.into(),
                challenge:                   db_challenge.to_vec(),
                create_time:                 Utc::now().naive_utc(),
            }
        }
    }

    pub async fn insert_challenge(conn: &mut Conn, value: ChallengeInsert) -> Result<i32> {
        let res = insert_into(verifier_challenges).values(value).returning(id).get_result(conn)?;
        Ok(res)
    }
}
