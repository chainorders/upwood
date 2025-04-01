# Security-Mint-Fund Contract Sequence Diagrams

## Fund Initialization and Investment Process

```mermaid
sequenceDiagram
    actor Admin
    actor Investor
    participant FC as Fund Contract
    participant TC as Token Contract
    participant PC as Token Presale Contract
    participant CTC as Currency Token Contract
    participant Indexer as Indexer

    Admin->>FC: addFund(token, securityToken, rate)
    Note over Admin,FC: Specifies Token Contract, Token ID, Presale Token Contract, Presale Token ID, Fund Rate
    FC-->>Indexer: Fund Added Event

    Investor->>FC: transferInvest(amount, securityToken)
    FC->>CTC: Transfer Currency Tokens
    CTC-->>FC: Tokens Transferred

    FC->>FC: invest function
    Note over FC: Calculates security token amount based on rate
    FC->>FC: Store investment details

    FC->>PC: Mint Presale Tokens (frozen)
    Note over FC,PC: Mints frozen presale tokens to investor
    PC-->>Indexer: Tokens Minted

    FC-->>Indexer: Invested Event
```

This diagram illustrates the primary workflow of the security-mint-fund contract:

1. Admin initializes a fund by specifying token details and conversion rate
2. Investors can transfer currency tokens to invest in the fund
3. The fund contract calculates the equivalent token amount using the rate
4. Presale tokens are minted to the investor in a frozen state
5. All events are sent to the Indexer for off-chain processing and monitoring

Additional operations like claiming investments, cancelling investments, and updating fund state are available as shown in the contract code.

## Fund State Update Process

```mermaid
sequenceDiagram
    actor Admin
    participant FC as Fund Contract
    participant Indexer as Indexer

    Admin->>FC: updateFundState(securityToken, state)
    Note over Admin,FC: Fund state can be changed from Open to Success or Fail
    FC->>FC: Verify Admin has UpdateFundState role
    FC->>FC: Verify current fund state is Open

    alt Set to Success
        Admin->>FC: Update with Success state and funds_receiver
        FC->>FC: Set fund state to Success with receiver info
        Note over FC: Funds will be sent to receiver upon claim
    else Set to Fail
        Admin->>FC: Update with Fail state
        FC->>FC: Set fund state to Fail
        Note over FC: Investments can be canceled and funds returned
    end

    FC-->>Indexer: FundStateUpdated Event
```

This diagram shows how an admin with the UpdateFundState role can update a fund's state:

1. Admin calls updateFundState with the security token ID and new state
2. The contract verifies the admin has proper permissions
3. The contract checks the fund is in Open state (can't update from Success or Fail)
4. Fund state is updated to either Success (with funds receiver) or Fail
5. The state change event is sent to the Indexer
6. Based on the new state, investors can either claim tokens (Success) or cancel investments (Fail)

## Investment Claiming Process (Fund Success)

```mermaid
sequenceDiagram
    actor Investor
    participant FC as Fund Contract
    participant TC as Token Contract
    participant PC as Token Presale Contract
    participant CTC as Currency Token Contract
    participant TR as Treasury Account
    participant Indexer as Indexer

    Investor->>FC: claimInvestment(securityToken)
    FC->>FC: Verify fund state is Success
    FC->>FC: Get investment details

    FC->>CTC: Transfer currency tokens to treasury
    CTC-->>TR: Currency tokens transferred

    alt If security token equals fund token
        FC->>PC: Unfreeze presale tokens
        Note over FC,PC: Tokens become transferable
    else If security token differs from fund token
        FC->>PC: Burn presale tokens
        FC->>TC: Mint unfrozen security tokens
        TC-->>Investor: Security tokens received
    end

    FC-->>Indexer: InvestmentClaimed Event
    FC-->>Investor: Claim completed
```

This diagram shows the process when an investor claims their investment after fund success:

1. Investor or authorized operator calls claimInvestment with the security token details
2. The contract verifies the fund is in "Success" state
3. The invested currency tokens are transferred to the treasury account (specified during fund state update)
4. Depending on token configuration:
   - If the security token equals the fund token: presale tokens are simply unfrozen
   - Otherwise: presale tokens are burned and new unfrozen security tokens are minted to the investor
5. An InvestmentClaimed event is sent to the Indexer

## Investment Return Process (Fund Fail)

```mermaid
sequenceDiagram
    actor Investor
    participant FC as Fund Contract
    participant PC as Token Presale Contract
    participant CTC as Currency Token Contract
    participant Indexer as Indexer

    Investor->>FC: claimInvestment(securityToken)
    FC->>FC: Verify fund state is Fail
    FC->>FC: Get investment details

    FC->>CTC: Return currency tokens to investor
    CTC-->>Investor: Currency tokens returned

    FC->>PC: Burn presale tokens

    FC-->>Indexer: InvestmentCancelled Event
    FC-->>Investor: Return completed
```

This diagram shows the process when an investor claims their investment after fund failure:

1. Investor or authorized operator calls claimInvestment with the security token details
2. The contract verifies the fund is in "Fail" state
3. The invested currency tokens are returned to the original investor
4. The presale tokens are burned
5. An InvestmentCancelled event is sent to the Indexer
