# Concordium Real World Asset Contracts

## Contracts

### [Market Contract](./market/src/lib.rs) : Market Contract for trading RWA tokens

Functions:

- `addPaymentToken` - Adds a new payment token to the contract.
- `addSellTokenContract` - Adds a new sell token contract to the contract. These tokens can be exchanged with the payment tokens.
- `allowedToList` - Checks if a token is allowed to be listed / sold on the contract.
- `balanceOfDeposited` - Returns the balance of deposited tokens for a user.
- `balanceOfListed` - Returns the balance of listed tokens for a user. Balance which can be exchanged.
- `balanceOfUnlisted` - Returns the balance of unlisted tokens for a user. Balance which can be withdrawn.
- `calculateAmounts` - Calculates the amounts which are sent to the seller, buyer and contract owner as commission.
- `deList` - Removes a token from the list of listed tokens.
- `deposit` - Deposits a certain amount of tokens to the contract.
- `exchange` - Exchanges one type of token for another.
- `getListed` - Returns a list of all listed tokens.
- `list` - Lists a new token in the contract.
- `paymentTokens` - Returns a list of all payment tokens.
- `withdraw` - Withdraws a certain amount of tokens from the contract.

### [Security Token Contract](./security-nft/src/lib.rs) : `rwa_security_nft` CIS2 compatible contract for security NFTs

The `Security Token Contract` is a robust and flexible solution for managing security Non-Fungible Tokens (NFTs). It provides a comprehensive set of functionalities for managing the lifecycle of these tokens.

Key functionalities include:

- **Agent Management**: Functions like `addAgent` and `removeAgent` allow for the addition and removal of agents, who are authorized to perform certain actions within the contract. The `agents` function retrieves the list of agents.
- **Token Management**: The contract supports the creation (`mint`), destruction (`burn`), and transfer (`transfer`, `forcedTransfer`) of tokens. It also allows for the freezing (`freeze`, `unFreeze`) and pausing (`pause`, `unPause`) of token operations.
- **Balance Queries**: Functions like `balanceOf`, `balanceOfFrozen`, and `balanceOfUnFrozen` provide the ability to query the total, frozen, and unfrozen balance of tokens for a specific holder.
- **Compliance and Identity Registry**: The `compliance` and `identityRegistry` functions return the contract addresses of the associated compliance contract and identity registry, respectively. These can be updated using `setCompliance` and `setIdentityRegistry`.
- **Recovery**: The `recover` function provides a mechanism to recover a lost account.
- **Interface Support Check**: The `supports` function allows the contract to check if it implements a specific interface.
- **Token Metadata**: The `tokenMetadata` function provides detailed information about a specific token.

#### Functions Security Token Contract

- `addAgent`: Adds a new agent to the contract.
- `agents`: Retrieves the list of agents.
- `balanceOf`: Fetches the total balance of tokens for a specific holder.
- `balanceOfFrozen`: Fetches the balance of frozen tokens for a specific holder.
- `balanceOfUnFrozen`: Fetches the balance of unfrozen tokens for a specific holder.
- `burn`: Burns a specific amount of tokens from a holder's account.
- `compliance`: Returns the contract address of the associated compliance contract.
- `forcedTransfer`: Executes a transfer of a token even if the transfer is non-compliant.
- `freeze`: Freezes a specific amount of tokens for a holder.
- `identityRegistry`: Returns the contract address of the associated identity registry.
- `isAgent`: Checks if a given address is an agent.
- `isPaused`: Verifies if operations for a specific token are currently paused.
- `mint`: Creates a new token and adds it to the total supply.
- `operatorOf`: Verifies if a given address is authorized to manage tokens for a specific owner.
- `pause`: Pauses all operations for a specific token.
- `recover`: Facilitates the recovery of a lost account.
- `recoveryAddress`: Retrieves the address that is authorized to recover a specific account.
- `removeAgent`: Removes an agent from the contract.
- `setCompliance`: Sets the compliance contract address.
- `setIdentityRegistry`: Sets the identity registry contract address.
- `supports`: Checks if the contract implements a specific interface.
- `tokenMetadata`: Provides detailed information about a specific token.
- `transfer`: Executes a compliant transfer of a token from the holder to another account.
- `unFreeze`: Unfreezes a specific amount of tokens for a holder.
- `unPause`: Resumes all operations for a specific token.
- `updateOperator`: Updates the operator authorized to manage tokens for a specific owner.

#### Events Security Token Contract

- `Recovered`: This event is triggered when an account is recovered.
- `IdentityRegistryAdded`: This event is triggered when an identity registry is added.
- `ComplianceAdded`: This event is triggered when compliance is added.
- `UnPaused`: This event is triggered when a token is unpaused.
- `Paused`: This event is triggered when a token is paused.
- `TokensFrozen`: This event is triggered when tokens are frozen.
- `TokensUnFrozen`: This event is triggered when tokens are unfrozen.
- `AgentRemoved`: This event is triggered when an agent is removed.
- `AgentAdded`: This event is triggered when an agent is added.
- `Cis2`: This event is forwarded from the CIS2 contract.

### [Identity Registry Contract](./identity-registry/src/lib.rs) : `rwa_identity_registry`

The `Identity Registry Contract` is a comprehensive solution for managing identities within a contract. It provides a wide range of functionalities for managing the lifecycle of identities.

Key functionalities include:

- **Agent Management**: Functions like `addAgent` and `removeAgent` allow for the addition and removal of agents, who are authorized to perform certain actions within the contract. The `agents` function retrieves the list of agents.
- **Issuer Management**: Functions like `addIssuer` and `removeIssuer` allow for the addition and removal of issuers, who are authorized to issue identities. The `issuers` function retrieves the list of issuers.
- **Identity Management**: The contract supports the registration (`registerIdentities`), deletion (`deleteIdentities`), and updating (`updateIdentities`) of identities. It also allows for the fetching of identity details (`getIdentity`) and checking if a holder has an identity (`hasIdentity`).
- **Identity Verification**: The `isVerified` function checks if an identity is verified.
- **Interface Support Check**: The `supports` function allows the contract to check if it implements a specific interface.
- **Identity Comparison**: The `isSame` function checks if two identities are the same.

#### Functions Identity Registry Contract

- `addAgent`: Adds a new agent to the contract.
- `addIssuer`: Adds a new issuer to the contract.
- `agents`: Retrieves the list of agents.
- `deleteIdentities`: Deletes specified identities from the contract.
- `getIdentity`: Fetches the identity details for a specific holder.
- `hasIdentity`: Checks if a given holder has an identity.
- `isAgent`: Checks if a given address is an agent.
- `isIssuer`: Checks if a given address is an issuer.
- `isSame`: Checks if two identities are the same.
- `isVerified`: Checks if an identity is verified.
- `issuers`: Retrieves the list of issuers.
- `registerIdentities`: Registers new identities in the contract.
- `removeAgent`: Removes an agent from the contract.
- `removeIssuer`: Removes an issuer from the contract.
- `supports`: Checks if the contract implements a specific interface.
- `updateIdentities`: Updates the details of specified identities.

#### Events Identity Registry Contract

- `IdentityRegistered`: Triggered when a new identity is registered.
- `IdentityUpdated`: Triggered when an existing identity is updated.
- `IdentityRemoved`: Triggered when an identity is removed.
- `IssuerAdded`: Triggered when a new issuer is added.
- `IssuerRemoved`: Triggered when an issuer is removed.
- `AgentAdded`: Triggered when a new agent is added.
- `AgentRemoved`: Triggered when an agent is removed.

### [Compliance Contract](./compliance/src/compliance/mod.rs) : `rwa_compliance`

The `Compliance Contract` is designed to manage and enforce compliance rules for token transactions. It provides a set of functionalities to control the lifecycle of tokens and manage authorized agents.

Key functionalities include:

- **Agent Management**: Functions like `addAgent` and `removeAgent` allow for the addition and removal of agents, who are authorized to perform certain actions within the contract. The `agents` function retrieves the list of agents.
- **Token Management**: The contract records the amount of tokens minted (`minted`), burned (`burned`), and transferred (`transferred`) between accounts.
- **Transfer Validation**: The `can_transfer` function checks if a certain amount of tokens can be transferred from one account to another, enforcing compliance rules.
- **Interface Support Check**: The `supports` function allows the contract to check if it implements a specific interface.

#### Functions Compliance Contract

- `addAgent`: Adds a new agent to the contract.
- `agents`: Retrieves the list of agents.
- `burned`: Records the amount of tokens burned from a holder's account.
- `can_transfer`: Checks if a certain amount of tokens can be transferred from one account to another.
- `isAgent`: Checks if a given address is an agent.
- `minted`: Records the amount of tokens minted to a holder's account.
- `removeAgent`: Removes an agent from the contract.
- `supports`: Checks if the contract implements a specific interface.
- `transferred`: Records the amount of tokens transferred from one account to another.

#### Events Compliance Contract

- `AgentRemoved`: Triggered when an agent is removed.
- `AgentAdded`: Triggered when a new agent is added.

### [Sponsor Contract](./sponsor/src/lib.rs) : `rwa_sponsor` [CIS3](https://proposals.concordium.software/CIS/cis-3.html) compatible contract

The `Sponsor Contract` is a CIS-3 standard contract designed to manage sponsorship of transactions. It provides a set of functionalities to control who can sponsor transactions on behalf of another account.

Key functionalities include:

- **Sponsorship Management**: The `permit` function grants permission for an account to sponsor transactions on behalf of another account. This allows for delegated sponsorship, where an account can allow another account to sponsor transactions, as per the CIS-3 standard.
- **Interface Support Check**: The `supportsPermit` function checks if the contract supports the `permit` functionality, as defined in the CIS-3 standard. This allows for compatibility checks with the CIS-3 standard.

#### Functions Sponsor Contract

- `permit`: Grants permission for a holder to spend a certain amount of tokens on behalf of the owner.
- `supportsPermit`: Checks if the contract supports the `permit` functionality.
