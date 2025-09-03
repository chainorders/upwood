# Security SFT Multi Blockchain Component

## File Locations

### Smart Contract

- Contract: `contracts/security-sft-multi/src/lib.rs`
- State: `contracts/security-sft-multi/src/state.rs`
- Types: `contracts/security-sft-multi/src/types.rs`

### Event Processor (Update Existing)

- Processor: `backend/events_listener/src/processors/security_sft_multi.rs`

### Database Layer (Processor-Managed)

- Database Models: `backend/shared/src/db/security_sft_multi.rs`
- Schema: `backend/shared/src/schema.rs`

---

# PART I: SMART CONTRACT ENHANCEMENTS

## Enhanced State Structure

```rust path=null start=null
struct State {
    // Existing state...
    metadata: StateMap<TokenId, MetadataUrl>,
    balances: StateMap<(TokenId, Address), Amount>,
    operators: StateSet<(Address, Address)>,
    
    // New trading fee state
    trading_fee_config: Option<TradingFeeConfig>,
    contract_trading_fees: StateMap<ContractAddress, TradingFeeConfig>,
}

struct TradingFeeConfig {
    treasury_account: AccountAddress,
    fee_percentage: u16, // basis points (100 = 1%)
    minimum_fee: Amount,
    maximum_fee: Amount,
}
```

## Contract Functions

### update_trading_fee_config

**Agent Role:** Admin

**Input Parameters:**

```rust path=null start=null
struct UpdateTradingFeeConfigParams {
    config: Option<TradingFeeConfig>, // None to disable, Some to enable/update
}
```

### update_contract_trading_fee

**Agent Role:** Admin

**Input Parameters:**

```rust path=null start=null
struct UpdateContractTradingFeeParams {
    contract_address: ContractAddress,
    config: Option<TradingFeeConfig>, // None to remove, Some to add/update
}
```

## Enhanced Transfer Logic

```rust path=null start=null
fn calculate_trading_fee(
    &self,
    sender: &Address,
    amount: Amount,
) -> ContractResult<Option<Amount>> {
    // Check if trading fees are enabled (config exists)
    let default_config = match &self.trading_fee_config {
        Some(config) => config,
        None => return Ok(None), // Trading fees disabled
    };
    
    // Check if sender is a contract
    if let Address::Contract(contract_addr) = sender {
        let config = self.contract_trading_fees
            .get(contract_addr)
            .unwrap_or(default_config);
            
        let fee_amount = amount * config.fee_percentage / 10000;
        let clamped_fee = fee_amount
            .max(config.minimum_fee)
            .min(config.maximum_fee)
            .min(amount); // Fee cannot exceed transfer amount
            
        Ok(Some(clamped_fee))
    } else {
        Ok(None) // No fee for direct wallet transfers
    }
}
```

## Events

```rust path=null start=null
struct TradingFeeCollectedEvent {
    token_id: TokenId,
    from: Address,
    to: Address,
    transfer_amount: Amount,
    fee_amount: Amount,
    treasury_account: AccountAddress,
}

struct TradingFeeConfigUpdatedEvent {
    config: Option<TradingFeeConfig>, // None means disabled
}

struct ContractTradingFeeUpdatedEvent {
    contract_address: ContractAddress,
    config: Option<TradingFeeConfig>, // None means removed
}
```

---

# PART II: EVENT PROCESSOR (ENHANCED)

```rust path=null start=null
pub fn process_events(
    conn: &mut DbConn,
    _block_height: Decimal,
    block_time: NaiveDateTime,
    _txn_index: Decimal,
    _txn_sender: &str,
    _txn_instigator: &str,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let parsed_event = event.parse::<Event>().expect("Failed to parse event");
        
        match parsed_event {
            // Existing security-sft-multi event handlers...
            
            // New trading fee event handlers
            Event::TradingFeeCollected(e) => {
                let new_record = TradingFeeRecord::new(
                    &e.token_id,
                    &e.from,
                    &e.to,
                    e.transfer_amount,
                    e.fee_amount,
                    &e.treasury_account,
                    contract.to_decimal(),
                    block_time
                );
                TradingFeeRecord::insert(new_record, conn)?;
            }
            Event::TradingFeeConfigUpdated(e) => {
                match e.config {
                    Some(config) => {
                        TradingFeeConfig::upsert(
                            contract.to_decimal(),
                            &config.treasury_account,
                            config.fee_percentage,
                            config.minimum_fee,
                            config.maximum_fee,
                            block_time,
                            conn
                        )?;
                    }
                    None => {
                        TradingFeeConfig::delete(contract.to_decimal(), conn)?;
                    }
                }
            }
            Event::ContractTradingFeeUpdated(e) => {
                match e.config {
                    Some(config) => {
                        ContractTradingFee::upsert(
                            contract.to_decimal(),
                            e.contract_address.to_decimal(),
                            &config.treasury_account,
                            config.fee_percentage,
                            config.minimum_fee,
                            config.maximum_fee,
                            block_time,
                            conn
                        )?;
                    }
                    None => {
                        ContractTradingFee::delete(
                            contract.to_decimal(),
                            e.contract_address.to_decimal(),
                            conn
                        )?;
                    }
                }
            }
        }
    }

    Ok(())
}
```

---

# PART III: DATABASE SCHEMA

### trading_fee_records

```sql path=null start=null
CREATE TABLE trading_fee_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id TEXT NOT NULL,
    contract_address DECIMAL(78, 0) NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    transfer_amount DECIMAL(78, 0) NOT NULL,
    fee_amount DECIMAL(78, 0) NOT NULL,
    treasury_account TEXT NOT NULL,
    transaction_hash BYTEA,
    processed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_trading_fee_records_contract 
ON trading_fee_records(contract_address, processed_at DESC);

CREATE INDEX idx_trading_fee_records_token 
ON trading_fee_records(token_id, processed_at DESC);
```

### trading_fee_config

```sql path=null start=null
CREATE TABLE trading_fee_config (
    contract_address DECIMAL(78, 0) PRIMARY KEY,
    treasury_account TEXT NOT NULL,
    fee_percentage INTEGER NOT NULL,
    minimum_fee DECIMAL(78, 0) NOT NULL,
    maximum_fee DECIMAL(78, 0) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

### contract_trading_fees

```sql path=null start=null
CREATE TABLE contract_trading_fees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_contract_address DECIMAL(78, 0) NOT NULL,
    trading_contract_address DECIMAL(78, 0) NOT NULL,
    treasury_account TEXT NOT NULL,
    fee_percentage INTEGER NOT NULL,
    minimum_fee DECIMAL(78, 0) NOT NULL,
    maximum_fee DECIMAL(78, 0) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(token_contract_address, trading_contract_address)
);

CREATE INDEX idx_contract_trading_fees_lookup 
ON contract_trading_fees(token_contract_address, trading_contract_address);
```

## Diesel Models

### TradingFeeRecord Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = trading_fee_records)]
pub struct TradingFeeRecord {
    pub id: Uuid,
    pub token_id: String,
    pub contract_address: Decimal,
    pub from_address: String,
    pub to_address: String,
    pub transfer_amount: Decimal,
    pub fee_amount: Decimal,
    pub treasury_account: String,
    pub transaction_hash: Option<Vec<u8>>,
    pub processed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = trading_fee_records)]
pub struct NewTradingFeeRecord {
    pub token_id: String,
    pub contract_address: Decimal,
    pub from_address: String,
    pub to_address: String,
    pub transfer_amount: Decimal,
    pub fee_amount: Decimal,
    pub treasury_account: String,
    pub transaction_hash: Option<Vec<u8>>,
    pub processed_at: DateTime<Utc>,
}

impl TradingFeeRecord {
    pub fn new(
        token_id: &TokenId,
        from: &Address,
        to: &Address,
        transfer_amount: Amount,
        fee_amount: Amount,
        treasury_account: &AccountAddress,
        contract_address: Decimal,
        processed_at: NaiveDateTime,
    ) -> NewTradingFeeRecord {
        NewTradingFeeRecord {
            token_id: token_id.to_string(),
            contract_address,
            from_address: from.to_string(),
            to_address: to.to_string(),
            transfer_amount: transfer_amount.into(),
            fee_amount: fee_amount.into(),
            treasury_account: treasury_account.to_string(),
            transaction_hash: None,
            processed_at: DateTime::from_utc(processed_at, Utc),
        }
    }

    pub fn insert(new_record: NewTradingFeeRecord, conn: &mut DbConn) -> QueryResult<Self> {
        diesel::insert_into(trading_fee_records::table)
            .values(&new_record)
            .get_result(conn)
    }
}
```

### TradingFeeConfig Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = trading_fee_config)]
pub struct TradingFeeConfig {
    pub contract_address: Decimal,
    pub treasury_account: String,
    pub fee_percentage: i32,
    pub minimum_fee: Decimal,
    pub maximum_fee: Decimal,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl TradingFeeConfig {
    pub fn upsert(
        contract_address: Decimal,
        treasury_account: &AccountAddress,
        fee_percentage: u16,
        minimum_fee: Amount,
        maximum_fee: Amount,
        updated_at: NaiveDateTime,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        diesel::insert_into(trading_fee_config::table)
            .values((
                trading_fee_config::contract_address.eq(contract_address),
                trading_fee_config::treasury_account.eq(treasury_account.to_string()),
                trading_fee_config::fee_percentage.eq(fee_percentage as i32),
                trading_fee_config::minimum_fee.eq(Decimal::from(minimum_fee)),
                trading_fee_config::maximum_fee.eq(Decimal::from(maximum_fee)),
                trading_fee_config::updated_at.eq(DateTime::from_utc(updated_at, Utc)),
            ))
            .on_conflict(trading_fee_config::contract_address)
            .do_update()
            .set((
                trading_fee_config::treasury_account.eq(treasury_account.to_string()),
                trading_fee_config::fee_percentage.eq(fee_percentage as i32),
                trading_fee_config::minimum_fee.eq(Decimal::from(minimum_fee)),
                trading_fee_config::maximum_fee.eq(Decimal::from(maximum_fee)),
                trading_fee_config::updated_at.eq(DateTime::from_utc(updated_at, Utc)),
            ))
            .get_result(conn)
    }

    pub fn delete(contract_address: Decimal, conn: &mut DbConn) -> QueryResult<usize> {
        diesel::delete(
            trading_fee_config::table
                .filter(trading_fee_config::contract_address.eq(contract_address))
        ).execute(conn)
    }
}
```

### ContractTradingFee Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = contract_trading_fees)]
pub struct ContractTradingFee {
    pub id: Uuid,
    pub token_contract_address: Decimal,
    pub trading_contract_address: Decimal,
    pub treasury_account: String,
    pub fee_percentage: i32,
    pub minimum_fee: Decimal,
    pub maximum_fee: Decimal,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl ContractTradingFee {
    pub fn upsert(
        token_contract_address: Decimal,
        trading_contract_address: Decimal,
        treasury_account: &AccountAddress,
        fee_percentage: u16,
        minimum_fee: Amount,
        maximum_fee: Amount,
        updated_at: NaiveDateTime,
        conn: &mut DbConn,
    ) -> QueryResult<Self> {
        diesel::insert_into(contract_trading_fees::table)
            .values((
                contract_trading_fees::token_contract_address.eq(token_contract_address),
                contract_trading_fees::trading_contract_address.eq(trading_contract_address),
                contract_trading_fees::treasury_account.eq(treasury_account.to_string()),
                contract_trading_fees::fee_percentage.eq(fee_percentage as i32),
                contract_trading_fees::minimum_fee.eq(Decimal::from(minimum_fee)),
                contract_trading_fees::maximum_fee.eq(Decimal::from(maximum_fee)),
                contract_trading_fees::updated_at.eq(DateTime::from_utc(updated_at, Utc)),
            ))
            .on_conflict((contract_trading_fees::token_contract_address, contract_trading_fees::trading_contract_address))
            .do_update()
            .set((
                contract_trading_fees::treasury_account.eq(treasury_account.to_string()),
                contract_trading_fees::fee_percentage.eq(fee_percentage as i32),
                contract_trading_fees::minimum_fee.eq(Decimal::from(minimum_fee)),
                contract_trading_fees::maximum_fee.eq(Decimal::from(maximum_fee)),
                contract_trading_fees::updated_at.eq(DateTime::from_utc(updated_at, Utc)),
            ))
            .get_result(conn)
    }

    pub fn delete(
        token_contract_address: Decimal,
        trading_contract_address: Decimal,
        conn: &mut DbConn,
    ) -> QueryResult<usize> {
        diesel::delete(
            contract_trading_fees::table
                .filter(contract_trading_fees::token_contract_address.eq(token_contract_address))
                .filter(contract_trading_fees::trading_contract_address.eq(trading_contract_address))
        ).execute(conn)
    }
}
