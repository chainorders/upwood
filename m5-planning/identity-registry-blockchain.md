# Identity Registry Blockchain Component

## File Locations

### Smart Contract

- Contract: `contracts/identity-registry/src/lib.rs`
- State: `contracts/identity-registry/src/state.rs`
- Types: `contracts/identity-registry/src/types.rs`
- Error: `contracts/identity-registry/src/error.rs`

### Event Processor

- Processor: `backend/events_listener/src/processors/identity_registry.rs`

### Database Layer (Processor-Managed)

- Database Models: `backend/shared/src/db/identity_registry.rs`
- Schema: `backend/shared/src/schema.rs`

---

# PART I: SMART CONTRACT

## State Structure

```rust path=null start=null
#[derive(Serialize, SchemaType, Clone, Debug, PartialEq, Eq)]
pub enum AddressState {
    Whitelisted,
    Blacklisted,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S=StateApi> {
    pub agents: StateSet<Address, S>,
    pub address_states: StateMap<Address, AddressState, S>,
}

// Address status logic:
// - If address not in map: Registered (default state)
// - If address in map with Whitelisted: Can receive maturity payments
// - If address in map with Blacklisted: Cannot receive any payments
```

## Contract Functions

### setAddressState

**Agent Role:** Agent (requires agent permission)

**Input Parameters:**

```rust path=null start=null
#[derive(Serialize, SchemaType)]
pub struct SetAddressStateParams {
    pub address: Address,
    pub state: AddressState,
}
```

**Implementation:**

```rust path=null start=null
#[receive(
    contract = "rwa_identity_registry",
    name = "setAddressState",
    mutable,
    enable_logger,
    parameter = "SetAddressStateParams",
    error = "Error"
)]
pub fn set_address_state(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        host.state().agents.contains(&ctx.sender()),
        Error::Unauthorized
    );
    
    let params: SetAddressStateParams = ctx.parameter_cursor().get()?;
    
    host.state_mut().address_states.insert(params.address, params.state.clone());
    
    let event = match params.state {
        AddressState::Blacklisted => Event::AddressStateChanged(AddressStateChangedEvent {
            address: params.address,
            new_state: AddressState::Blacklisted,
        }),
        AddressState::Whitelisted => Event::AddressStateChanged(AddressStateChangedEvent {
            address: params.address,
            new_state: AddressState::Whitelisted,
        }),
    };
    
    logger.log(&event)?;
    Ok(())
}
```

### removeAddress

**Agent Role:** Agent (requires agent permission)

**Input Parameters:**

```rust path=null start=null
pub type Address = concordium_std::Address;
```

**Implementation:**

```rust path=null start=null
#[receive(
    contract = "rwa_identity_registry",
    name = "removeAddress",
    mutable,
    enable_logger,
    parameter = "Address",
    error = "Error"
)]
pub fn remove_address(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
    logger: &mut Logger,
) -> ContractResult<()> {
    ensure!(
        host.state().agents.contains(&ctx.sender()),
        Error::Unauthorized
    );
    
    let address: Address = ctx.parameter_cursor().get()?;
    
    ensure!(
        host.state_mut().address_states.remove(&address).is_some(),
        Error::AddressNotFound
    );
    
    logger.log(&Event::AddressRemoved(AddressRemovedEvent { address }))?;
    Ok(())
}
```

### getAddressState

**Agent Role:** Public (read-only)

**Input Parameters:**

```rust path=null start=null
pub type Address = concordium_std::Address;
```

**Return Value:**

```rust path=null start=null
#[derive(Serialize, SchemaType)]
pub enum AddressStatus {
    Registered,    // Not in map (default state)
    Whitelisted,   // In map with Whitelisted state
    Blacklisted,   // In map with Blacklisted state
}
```

**Implementation:**

```rust path=null start=null
#[receive(
    contract = "rwa_identity_registry",
    name = "getAddressState",
    parameter = "Address",
    return_value = "AddressStatus"
)]
pub fn get_address_state(
    ctx: &ReceiveContext,
    host: &Host<State>,
) -> ContractResult<AddressStatus> {
    let address: Address = ctx.parameter_cursor().get()?;
    
    match host.state().address_states.get(&address) {
        Some(AddressState::Whitelisted) => Ok(AddressStatus::Whitelisted),
        Some(AddressState::Blacklisted) => Ok(AddressStatus::Blacklisted),
        None => Ok(AddressStatus::Registered),
    }
}
```

### isWhitelisted

**Agent Role:** Public (read-only)

**Input Parameters:**

```rust path=null start=null
pub type Address = concordium_std::Address;
```

**Return Value:** `bool`

### isBlacklisted

**Agent Role:** Public (read-only)

**Input Parameters:**

```rust path=null start=null
pub type Address = concordium_std::Address;
```

**Return Value:** `bool`

## Events

```rust path=null start=null
#[derive(Serialize, SchemaType, Debug)]
pub struct AgentUpdatedEvent {
    pub agent: Address,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct AddressStateChangedEvent {
    pub address: Address,
    pub new_state: AddressState,
}

#[derive(Serialize, SchemaType, Debug)]
pub struct AddressRemovedEvent {
    pub address: Address,
}

#[derive(Serialize, SchemaType, Debug)]
#[concordium(repr(u8))]
pub enum Event {
    AgentAdded(AgentUpdatedEvent),
    AgentRemoved(AgentUpdatedEvent),
    AddressStateChanged(AddressStateChangedEvent),
    AddressRemoved(AddressRemovedEvent),
}
```

## Error Types

```rust path=null start=null
#[derive(Serial, Reject, SchemaType)]
pub enum Error {
    ParseError,
    LogError,
    Unauthorized,
    AgentAlreadyExists,
    AgentNotFound,
    AddressNotFound,
    AddressAlreadyInState,
}
```

---

# PART II: EVENT PROCESSOR

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
    for (event_index, event) in events.iter().enumerate() {
        let parsed_event = event.parse::<Event>().expect("Failed to parse event");
        
        match parsed_event {
            Event::AgentAdded(e) => {
                Agent::new(e.agent, block_time, contract.to_decimal()).insert(conn)?;
            }
            Event::AgentRemoved(e) => {
                Agent::delete(conn, contract.to_decimal(), &e.agent)?;
            }
            Event::AddressStateChanged(e) => {
                let state_str = match e.new_state {
                    AddressState::Blacklisted => "blacklisted",
                    AddressState::Whitelisted => "whitelisted",
                };
                
                // Upsert address state (insert or update)
                let address_state = AddressStateRecord::new(
                    &e.address,
                    state_str,
                    block_time,
                    contract.to_decimal()
                );
                AddressStateRecord::upsert(address_state, conn)?;
                
                // Record history
                let history = AddressStateHistory::new_state_change(
                    &e.address,
                    state_str,
                    _block_height,
                    block_time,
                    contract.to_decimal(),
                    event.transaction_hash.clone(),
                    event_index as i32
                );
                AddressStateHistory::insert(history, conn)?;
            }
            Event::AddressRemoved(e) => {
                // Remove address state entry
                AddressStateRecord::delete(conn, contract.to_decimal(), &e.address)?;
                
                // Record history
                let history = AddressStateHistory::new_state_change(
                    &e.address,
                    "removed",
                    _block_height,
                    block_time,
                    contract.to_decimal(),
                    event.transaction_hash.clone(),
                    event_index as i32
                );
                AddressStateHistory::insert(history, conn)?;
            }
        }
    }

    Ok(())
}
```

---

# PART III: DATABASE SCHEMA

### address_states

```sql path=null start=null
CREATE TABLE address_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contract_address DECIMAL(78, 0) NOT NULL,
    wallet_address TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('whitelisted', 'blacklisted')),
    state_set_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(contract_address, wallet_address)
);

CREATE INDEX idx_address_states_lookup 
ON address_states(contract_address, wallet_address);

CREATE INDEX idx_address_states_contract_state 
ON address_states(contract_address, state, state_set_at DESC);

CREATE INDEX idx_address_states_state 
ON address_states(state, state_set_at DESC);
```

### address_state_history

```sql path=null start=null
CREATE TABLE address_state_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contract_address DECIMAL(78, 0) NOT NULL,
    wallet_address TEXT NOT NULL,
    action TEXT NOT NULL CHECK (action IN ('whitelisted', 'blacklisted', 'removed')),
    block_height DECIMAL(20, 0) NOT NULL,
    block_time TIMESTAMP WITH TIME ZONE NOT NULL,
    transaction_hash BYTEA NOT NULL,
    event_index INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(transaction_hash, event_index)
);

CREATE INDEX idx_address_state_history_wallet 
ON address_state_history(wallet_address, block_time DESC);

CREATE INDEX idx_address_state_history_contract 
ON address_state_history(contract_address, block_time DESC);
```

## Diesel Models

### AddressStateRecord Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(table_name = address_states)]
pub struct AddressStateRecord {
    pub id: Uuid,
    pub contract_address: Decimal,
    pub wallet_address: String,
    pub state: String,
    pub state_set_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = address_states)]
pub struct NewAddressStateRecord {
    pub contract_address: Decimal,
    pub wallet_address: String,
    pub state: String,
    pub state_set_at: DateTime<Utc>,
}

impl AddressStateRecord {
    pub fn new(
        wallet_address: &concordium_rust_sdk::types::Address,
        state: &str,
        state_set_at: NaiveDateTime,
        contract_address: Decimal,
    ) -> NewAddressStateRecord {
        NewAddressStateRecord {
            contract_address,
            wallet_address: wallet_address.to_string(),
            state: state.to_string(),
            state_set_at: DateTime::from_utc(state_set_at, Utc),
        }
    }

    pub fn upsert(new_state: NewAddressStateRecord, conn: &mut DbConn) -> QueryResult<Self> {
        diesel::insert_into(address_states::table)
            .values(&new_state)
            .on_conflict((address_states::contract_address, address_states::wallet_address))
            .do_update()
            .set((
                address_states::state.eq(new_state.state),
                address_states::state_set_at.eq(new_state.state_set_at),
                address_states::updated_at.eq(diesel::dsl::now)
            ))
            .get_result(conn)
    }

    pub fn delete(
        conn: &mut DbConn,
        contract_addr: Decimal,
        wallet_addr: &concordium_rust_sdk::types::Address,
    ) -> QueryResult<usize> {
        diesel::delete(
            address_states::table
                .filter(address_states::contract_address.eq(contract_addr))
                .filter(address_states::wallet_address.eq(wallet_addr.to_string()))
        ).execute(conn)
    }

    pub fn get_address_state(
        contract_addr: Decimal,
        wallet_addr: &str,
        conn: &mut DbConn,
    ) -> QueryResult<Option<Self>> {
        address_states::table
            .filter(address_states::contract_address.eq(contract_addr))
            .filter(address_states::wallet_address.eq(wallet_addr))
            .first(conn)
            .optional()
    }

    pub fn get_addresses_by_state(
        contract_addr: Decimal,
        state: &str,
        conn: &mut DbConn,
    ) -> QueryResult<Vec<Self>> {
        address_states::table
            .filter(address_states::contract_address.eq(contract_addr))
            .filter(address_states::state.eq(state))
            .order(address_states::state_set_at.desc())
            .load(conn)
    }

    pub fn is_whitelisted(
        contract_addr: Decimal,
        wallet_addr: &str,
        conn: &mut DbConn,
    ) -> QueryResult<bool> {
        Ok(Self::get_address_state(contract_addr, wallet_addr, conn)?
            .map(|record| record.state == "whitelisted")
            .unwrap_or(false))
    }

    pub fn is_blacklisted(
        contract_addr: Decimal,
        wallet_addr: &str,
        conn: &mut DbConn,
    ) -> QueryResult<bool> {
        Ok(Self::get_address_state(contract_addr, wallet_addr, conn)?
            .map(|record| record.state == "blacklisted")
            .unwrap_or(false))
    }
}
```


### AddressStateHistory Model

```rust path=null start=null
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = address_state_history)]
pub struct AddressStateHistory {
    pub id: Uuid,
    pub contract_address: Decimal,
    pub wallet_address: String,
    pub action: String,
    pub block_height: Decimal,
    pub block_time: DateTime<Utc>,
    pub transaction_hash: Vec<u8>,
    pub event_index: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = address_state_history)]
pub struct NewAddressStateHistory {
    pub contract_address: Decimal,
    pub wallet_address: String,
    pub action: String,
    pub block_height: Decimal,
    pub block_time: DateTime<Utc>,
    pub transaction_hash: Vec<u8>,
    pub event_index: i32,
}

impl AddressStateHistory {
    pub fn new_state_change(
        wallet_address: &concordium_rust_sdk::types::Address,
        action: &str,
        block_height: Decimal,
        block_time: NaiveDateTime,
        contract_address: Decimal,
        transaction_hash: Vec<u8>,
        event_index: i32,
    ) -> NewAddressStateHistory {
        NewAddressStateHistory {
            contract_address,
            wallet_address: wallet_address.to_string(),
            action: action.to_string(),
            block_height,
            block_time: DateTime::from_utc(block_time, Utc),
            transaction_hash,
            event_index,
        }
    }

    pub fn insert(new_history: NewAddressStateHistory, conn: &mut DbConn) -> QueryResult<Self> {
        diesel::insert_into(address_state_history::table)
            .values(&new_history)
            .get_result(conn)
    }

    pub fn get_address_history(
        wallet_address: &str,
        conn: &mut DbConn,
    ) -> QueryResult<Vec<Self>> {
        address_state_history::table
            .filter(address_state_history::wallet_address.eq(wallet_address))
            .order(address_state_history::block_time.desc())
            .load(conn)
    }
}
