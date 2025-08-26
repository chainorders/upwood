# Security-SFT-Multi-Yielder Contract Sequence Diagrams

## Adding Yield Configuration Process

```mermaid
sequenceDiagram
    actor Admin
    participant YC as Yielder Contract
    participant Indexer as Indexer

    Admin->>YC: upsertYield(token_contract, token_id, yields)
    Note over Admin,YC: Specifies Security Token Contract, Token ID (version), and Yield configurations
    YC->>YC: Verify Admin has AddYield role
    YC->>YC: Store yield configurations
    Note over YC: Maps token versions to yield rewards with rates
    YC-->>Indexer: YieldAdded Event
```

## Token Holder Claiming Yields Process

```mermaid
sequenceDiagram
    actor TokenHolder
    participant YC as Yielder Contract
    participant STC as Security Token Contract
    participant YTC as Yield Token Contract
    participant TR as Treasury
    participant Indexer as Indexer

    TokenHolder->>YC: yieldFor(owner, yields)
    Note over TokenHolder,YC: Request yields for tokens held from version 1 to 100
    YC->>YC: Verify caller is owner or has Operator role
    YC->>YC: Calculate yields for each version jump
    Note over YC: Applies yield rates for each version difference

    YC->>YTC: Transfer yield tokens
    Note over YC,YTC: Transfer reward tokens from treasury to token holder
    YTC->>TR: Request tokens
    TR-->>TokenHolder: Yield tokens transferred

    YC->>STC: Burn old version tokens (e.g., Token ID 1)
    STC->>STC: Burn tokens from holder
    Note over STC: Remove outdated token version

    YC->>STC: Mint new version tokens (e.g., Token ID 100)
    STC->>STC: Mint tokens to holder
    Note over STC: Create same amount of new token version

    YC-->>Indexer: YieldDistributed Event
    YC-->>TokenHolder: Yield process complete
```

These diagrams illustrate the key workflows in the security-sft-multi-yielder contract:

1. **Adding Yields**: Admins configure which yields (rewards) are available for specific token versions
2. **Claiming Yields**: Token holders receive rewards and upgrade their tokens to newer versions

The process enables token holders to receive rewards for holding security tokens over time, with the rewards calculated based on token amount and holding duration.
