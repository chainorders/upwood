# Security Mint Fund

This smart contract manages investments by exchanging a specified currency token for a security token. Key features include:

- Defining agents with specific roles to control funding operations.
- Creating and removing funds that define conversion rates between currency and security tokens.
- Updating the fund’s state (Open, Success, or Fail) to handle the logic of investment, cancellation, or claim.
- Collecting and tracking user investments, minting tokens when investments are made.
- Returning or transferring invested funds based on the fund's final state.

Investors either receive their security tokens upon success or have their tokens burned with a refund on failure. Agents control fund life cycle and are authorized via checks in each function.

## Functions

- init: Initializes the contract, setting up the initial agents and currency token.
- add_agent: Adds a new agent with specific roles, authorized by the contract owner.
- remove_agent: Removes an agent, authorized by the contract owner.
- add_fund: Creates a new fund, defining the conversion rate between currency and security tokens. Requires `AddFund` agent role.
- remove_fund: Removes an existing fund, provided there are no investments. Requires `RemoveFund` agent role.
- update_fund_state: Updates a fund’s state to Open, Success, or Fail. Requires `UpdateFundState` agent role.
- transfer_invest: Transfers currency tokens to the contract to begin an investment.
- invest: Mints locked security tokens after receiving currency, updating the investor's balance.
- claim_investment: Finalizes or cancels an investment based on the fund’s state, transferring or returning funds accordingly.
- has_agent: Checks if an address has a particular role.

## Contract States

The contract operates with the following states for each fund:

- **Open**: The fund is active and accepting investments.
- **Success**: The fund has reached its goal, and investors can claim their security tokens.
- **Fail**: The fund has failed, and investors can cancel their investments to receive their currency tokens back.

## Key Functionality

### Initialization (`init`)

The `init` function initializes the contract with the initial parameters, including the currency token and the initial agents with their roles. The contract creator is automatically assigned the `owner` role.

### Agent Management (`add_agent`, `remove_agent`)

Agents are addresses with specific roles that allow them to perform certain actions, such as adding or removing funds, or updating the fund state. The `add_agent` and `remove_agent` functions are used to manage these agents, and are restricted to the contract owner.

### Fund Management (`add_fund`, `remove_fund`)

Funds define the terms of the investment, including the conversion rate between the currency token and the security token. The `add_fund` function creates a new fund, while the `remove_fund` function removes an existing fund, provided there are no investments.

### Investment Flow (`transfer_invest`, `invest`, `claim_investment`)

The investment flow involves three key functions:

1. `transfer_invest`: Transfers currency tokens to the contract, initiating the investment process.
2. `invest`: Mints locked security tokens to the investor, based on the fund's conversion rate.
3. `claim_investment`: Allows investors to claim their security tokens if the fund is successful, or cancel their investments and receive their currency tokens back if the fund fails.

### State Transitions (`update_fund_state`)

The `update_fund_state` function allows authorized agents to update the state of a fund, which determines the outcome of the investment. The state can be transitioned from `Open` to `Success` or `Fail`.

## Error Handling

The contract defines a custom `Error` enum to handle various error conditions, such as unauthorized access, invalid fund state, and token transfer failures.
