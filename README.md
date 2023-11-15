# Real World Asset (RWA) for Concordium

## Accounts & Roles

### Admin

Admin account is responsible for

1. Initializing the Security Token Contract (STC)
2. Initializing Trusted Identity Registry (TIR) Contract
3. Performing Off-Chain KYC and updating `KYCRole -> Address` Mapping in `TIR Contract`
4. Adding `Trusted Agent TIR` to TIR
5. Initializing Compliance Contract (ComC)
6. Adding `Trusted Agent ComC (TaComC)` to Compliance Contract (ComC)
7. Initializing Fractionalizer token Contract (FTC)
8. Adding MLP Account to FTC to be able to set fractions for a particular Token
9. Adding trusted STC to FTC. An STC will only accept token transfers from trusted STC

### MLP

MLP Account is responsible for

1. Keeping off chain custody of the RWA
2. Initializing Token Fractionalizer
3. Adding Trusted STC to the Fractionalizer Contract
4. Minting a token representing RWA on chain
5. Minting a token representing Fractions of RWA and setting the maximum supply for the Fraction Tokens

`Admin` should add MLP account in `Mint` KYCRole after initializing the `TIR Contract`

### Trusted Agent TIR (TaTir)

Trusted Agent is responsible for

1. Updating `TERole -> TE` Mapping in `TIR Contract`

A TA should be added to the TIR by the Admin (Owner of the TIR Contract)
A TA is responsible for following regulations and adding appropriate TE needed for a particular `TERole` in TIR

### Trusted Entity Account

Owner of the TE required. This account initializes needed TE which is a CIS4 contract and is responsible for issuing VC representing licenses whenever required for a particular role.

### Trusted Agent Compliance Contract (TaComC)

Responsible for Adding and Removing Trusted Module to Compliance Contract

### RWA Receiver (RwaR)

An account which can receive a RWA.
RwaR represents a Account Address and Holder of VC's issued by Trusted TE's in TIR
The Account should perform KYC with `Admin` to be added in the `KYCRole->Address` mapping by the admin
The Account holder should also be able to provide signatures from keys required and added TE's to TIR

## Contracts

### Security Token Contract (CIS2)

**Every CIS2 token contract can contract multiple tokens of the same type**. Which means they should have the same trust requirements for Minting, Receiving & Burning. Which means the same set of Addresses in the TIR Contract for a particular role should be able to execute that role over the tokens of the same type / contract.

Every Security Token Contract will have a corresponding Trusted Registry Contract defining the roles and their trust requirements (KYC and VC's) needed.

* Who should be able to Mint?
The custodian of the RWA should be able to Mint a token representing that an RWA has been Tokenized by the Minter. The Owner of the CIS2 token contract should follow regulations to add an appropriate minter

* Who should be able to Receive?
* Who should be able to Burn?

#### Functions

Apart from the functions described in [CIS2 contract standard](https://proposals.concordium.software/CIS/cis-2.html#cis-2-concordium-token-standard-2)

1. Receive : Executed by the receiver of the token. Only after a successful call to this function the `balanceOf` will be updated denoting complete transfer. This function is needed in addition to the transfer function because in the cases where a VC is required & a TE is specified for the `Receiver role`. Only the receiver of the token can send valid signatures to validate his identity against issued credentials.
2. Burn : Removes the token balance from the Current Owner

### Trusted Identity Registry Contract (TIR)

#### State

* Mapping of `KYCRole -> Set<Address>`
  * `Address is Account Address | Contract Address`
  * `KYCRole` is an enum of `Mint`, `Receive`, `Burn` and is extendable.
    * `KYCRole` can only be updated by the owner of the contract. Hence Owner of this contract needs to track the regulations and perform appropriate KYC
* Mapping of `TERole -> Set<TE>`
  * `TE is CIS4 Contract Address`
  * `TERole` is an enum of `Mint`, `Receive`, `Burn` and is extendable
    * `TERole` can only be updated by a `Trusted Agent`
* `Set<TA>`
  * TA is a Trusted Agent & Account address
  * TA's should be part of the regulations body of the particular token and are responsible for adding TE's to a particular role
  * Owner of the contract can make himself a TA
* Mapping of `Address -> Set<Pair<Compliance Attribute, Attribute Value>>`

#### Logical Flow

  Example flow During minting of a particular token the CIS2 contract will check if the `sender` of the transaction

* **Has done KYC** by checking if the Mint KYCRole has the sender address mentioned
* **Has Required License** by checking all the TE's in the Mint TERole for public keys of the attached signatures. at least 1 TE should have an unrevoked claim for the mentioned key with signatures

and the transaction is

### Trusted Agent Contract (TE)

This is a CIS4 contract responsible for Issuing VC to Public Keys for a particular `TERole`. A TE contract is ony to be used when the regulations require a license (VC) to be needed for Minting, Holding etc for a particular token

### Compliance Contract (ComC)

A Compliance contract is only responsible for checking the global compliance of a particular transaction which involves transferring ownership of a particular token.

Compliance contract makes the Compliance Mechanism dynamic by having the Ability to Add / Remove Compliance Modules dynamically. Each compliance module is responsible for checking individual global compliance required for transferring a particular token.

### Compliance Contract Module (ComCM)

Multiple Compliance Contract Modules can be added to a `Compliance Contract (ComC)`. Each Compliance Module Contract is created to check for an individual part of global compliance while transferring the ownership of a token

### Marketplace Contract (MC)

A MC should be added as a valid receiver of the Token in STC. Since MC holds the token in custody while allowing anyone to buy the token

### Fractionalized Token Contract (FTC)

A FTC should be added as a valid receiver of the token in STC. Since FTC holds the token in custody while providing the fractionalized tokens
