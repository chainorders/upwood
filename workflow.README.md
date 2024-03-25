## Workflow

Below workflow details all the steps needed to demo / debug the application. Kindly note that Account Addresses, Contract Addresses & Transaction links mentioned as the ones which were used during the reproduction of the below workflow and **WILL** be different each time / on every system.

### Prerequisites

* An Account for Admin Role : `47fb97YAZtEEYNpaWz3ccrUCwqEnNfm2qQXiUGHEJ52Fiu7AVi`
* An Account for Identity Registry Agent Role : `3ab35yTaTTK8xr4jSBDBmMYAf9V8s5zVZAR86Rah1daX3uA39Q`
* An Account for Sponsor Role (Can be same as Identity Registry Agent) : `3ab35yTaTTK8xr4jSBDBmMYAf9V8s5zVZAR86Rah1daX3uA39Q`
* An Account with `IN` Nationality : `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN`
* An Account with `US` Nationality : `3mudn3pcFWnFrrJgzQ2U9W1Vuu2zNvG2BspnFZNb9BB5XoMswN`
* downloaded wallet export (from concordium browser wallet) : at some path (`/home/parv0888/Downloads/`)

### Terminal

```bash
git clone git@github.com:chainorders/concordium-rwa.git
git submodule update --init --recursive
cd concordium-rwa
yarn
cp /home/parv0888/Downloads/3ab35yTaTTK8xr4jSBDBmMYAf9V8s5zVZAR86Rah1daX3uA39Q.export ./backend/verifier_wallet.export
cp /home/parv0888/Downloads/3ab35yTaTTK8xr4jSBDBmMYAf9V8s5zVZAR86Rah1daX3uA39Q.export ./backend/sponsor_wallet.export
docker compose up -d
```

### VS Code

* Update Starting Block hash in the [backend env file](./backend/.env). *this is an optional step if not set the system would start from current consensus block*
  * `STARTING_BLOCK_HASH`=ebcb1b1a919b6ef55b707e6eb8b373f587869820f263d3ae3db9878b470bed84

* Generate Frontend Clients & Start the contract api

    ```bash
    yarn workspace backend generate:client
    yarn workspace backend debug:contracts-api
    ```

* Start frontend

    ```bash
    yarn workspace frontend dev
    ```

### [Admin UI](http://localhost:5173/contracts)

* Selected Account `47fb97YAZtEEYNpaWz3ccrUCwqEnNfm2qQXiUGHEJ52Fiu7AVi`
* Initialize Identity Registry : 8176
* Add Identity Registry Agent take account address from the [wallet file](./backend/agent_wallet.export)

### VS Code

* Update env variables in [backend env file](./backend/.env)
  * `IDENTITY_REGISTRY`

### Terminal

```bash
    yarn workspace backend debug:verifier-api
```

### [Admin UI](http://localhost:5173/contracts)

* Initialize Compliance Module : 8177
  * Name : Compliance Module (IN, US)
  * Nationality : IN, US
  * Selected Identity Registry : 8176
* Initialize Compliance Contract : 8178
  * Name : Compliance
  * Select Compliance Module : 8177
* Initialize Sponsor Contract : 8179
  * Name : Sponsor Contract
* Initialize NFT Contract : 8180
  * Name : Real Estate
  * Identity Registry : 8176
  * Compliance : 8178
  * Sponsor : 8179
* Select Wallet Account : `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` (Indian Account) - This is required because the following contracts need to hold tokens and current account is not of compliant nationality
* Initialize Fractionalizer Contract : 8185
  * Name : Real Estate Fractionalizer
  * Identity Registry : 8176
  * Compliance : 8178
  * Sponsor : 8179
* Initialize Market Contract : 8186
  * Name : Real Estate Market
  * Commission : 0
  * Token Contracts : 8180, 8185
  * Exchange Token : Unit, 7260
* `<SFT Contract>` > Add Agent
  * Account : `47fb97YAZtEEYNpaWz3ccrUCwqEnNfm2qQXiUGHEJ52Fiu7AVi` (Admin)
  * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=30b9bed1f5343327b2f1f976dae4a1517fbd3b9abcba5a7781c50ee5b174d78f)

### VS Code

* Update [frontend env variables](./frontend/.env)
  * `VITE_NFT_SFT_CONTRACT_INDEX=8185`
  * `VITE_MARKET_CONTRACT_INDEX=8186`
  * `VITE_SPONSOR_CONTRACT_INDEX=8179`
* Update [backend env variables](./backend/.env)
  * `SPONSOR_CONTRACT=<8179,0>`

### Terminal

```bash
yarn workspace backend debug:sponsor-api
```

### Market UI

* Select Wallet Account : `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` (Indian Account)

* Menu > Register
  * Generate Challenge
  * Contract : 8185
  * Popup Opens : Select Account `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` > Approve
  * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=21c5e9ae2cb59e0e84417fc8d7cf24416d8d39337f8f5d6f1bd60fdcf472f036)

* Menu > Register
  * Generate Chalenge
  * Contract : 8186
  * Popup Opens : Select Account `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` > Approve
  * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=8848a185ca55ae01ab923718c0798e0ebd1ef92e412f2611a20e8b6512951ea9)

* Menu > Register
  * Generate Chalenge
  * Popup Opens : Select Account `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` > Approve
  * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=2e7456db97afdc0870d766f7a764fd0e698fd00453590ac524cbff26815191d0)

* Select Wallet Account : `3mudn3pcFWnFrrJgzQ2U9W1Vuu2zNvG2BspnFZNb9BB5XoMswN` (US Account)

* Menu > Register
  * Generate Chalenge
  * Popup Opens : Select Account `3mudn3pcFWnFrrJgzQ2U9W1Vuu2zNvG2BspnFZNb9BB5XoMswN` > Approve
  * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=9be181cd460db0f0300faea2904f22e42f354fbcd13ae782eaa58df928777d18)

### Admin UI

* Select Account `47fb97YAZtEEYNpaWz3ccrUCwqEnNfm2qQXiUGHEJ52Fiu7AVi` (Admin)
* `<NFT Contract>` > Mint
  * Owner : Account `3mudn3pcFWnFrrJgzQ2U9W1Vuu2zNvG2BspnFZNb9BB5XoMswN`
  * Tokens
    * <https://ipfs.io/ipfs/QmSoLMeqvY7u3CTK3VZQxYDtvEabEz4tRD86kbG7CpmjeV>
    * <https://ipfs.io/ipfs/Qmb2qLX9up4dcyHSjdxLJwMUbHS2vxit9epXv6ZmmNia6B>
    * <https://ipfs.io/ipfs/QmYS8Qmtw14amcfoNHTLdTTuo9Tbcomptek8GAXhsRAc4z>
  * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=7b7aec5edcc04090a48be4738143ec2a53caaf3f99b480bc53f67e33b6515c7d)
* You should now see 3 tokens at the `tokens` section
* `<SFT Contract>` > Add Tokens
  * Tokens
    * Deposit Token Id
      * Contract : <8180,0> (NFT Contract)
      * token id : 00
      * metadata url : <https://ipfs.io/ipfs/QmdwLwXY9RD66NAJXcPAeJShBktBneBGaQoTZ4cXdVSEvZ>
    * Fractions : 1000
    * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=0c7d77bc4b0e8791d32f7162639d3677ae8fb65ddbcdb86cd859437241b4001b)
* `<SFT Contract>` > Add Tokens
  * Tokens
    * Deposit Token Id
      * Contract : <8180,0> (NFT Contract)
      * token id : 01
      * metadata url : <https://ipfs.io/ipfs/QmVBecYzhr6N1RLoDGXet6AXiU8K1LuHUqNPep3nyBhnkz>
    * Fractions : 10000
    * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=0c7d77bc4b0e8791d32f7162639d3677ae8fb65ddbcdb86cd859437241b4001b)

### Market UI

* Select Account `3mudn3pcFWnFrrJgzQ2U9W1Vuu2zNvG2BspnFZNb9BB5XoMswN` (US Account)
* Menu > Tokens
  * You should see 3 tokens
* NFT Token 1 (00)
  * Fractionalize > Sign
    * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=d872287af3e0c0fba81802c9709854234f62f5cdf1b0182dc472a2eb4ed32f81)
* Upon Refresh you should see 2 NFT tokens and 1 fractionalized tokens
* NFT Token 2 (01)
  * Sponsored Fractionalize > Submit
    * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=0d1f6d94d9af69608540c95a14a8aca59b1746b41fcbf815e9d069a7e3eadf4d)
* Upon Refresh you should see 1 NFT tokens and 2 fractionalized tokens
* NFT Token 3 (02)
  * Sell
    * data
      * Contract : 8180
      * token id 02
      * exchange rates
        * CCD: 10
        * 7260 : 1
    * `>` List Sponsored > Sign
    * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=e709a4cc95d9a85afb8d1bdc2b43f6255502448f8f31e79d9716ec44dd915a22)
* Menu > Market
  * You should now see 1 listed token
* Menu > Tokens
  * SFT Token 1 (0000000)
    * Sell
      * Contract : 8185
      * token id : 00000000
      * amount : 1000
      * exchange rates
        * ccd : 10
        * 7260 : 1
      * `>` List > Submit
        * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=0bc6a3bd95ebdb7f173c2365d37d34d096054e2bc1feb5f9202d796e79327068)
* Menu > Market
  * You should now see 2 listed token
* Select Account `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` (Indian Account)
* NFT Token Id : 02
  * Buy
    * Select Payment Token : 7260/0
    * Buy Amount : 1
    * Pay By Token Via Transfer > Submit
    * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=e08890a16958ef4049b19ac9743c8de269ae0e5d015b50e94da015ff6c255375)
* Menu > Market
  * You should now see 1 listed token
* Admin UI > NFT Contract > Balance of
  * Account : `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` > Submit
  * You should see 1
* Select account `3mudn3pcFWnFrrJgzQ2U9W1Vuu2zNvG2BspnFZNb9BB5XoMswN` (US account)
* Menu > Market
  * SFT Token 00000000
    * De List (Wait for transaction : loader missing)
* Menu > Un Listed Tokens
  * SFT Token 00000000
  * Return > Withdraw
    * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=639bcd407c3274d2c86c71c98cfc81873c3a53ae40641cb6e6d211a522273df6)
* Menu > Tokens
  * You should see 2 tokens
  * SFT Token 00000000
    * Sell
      * Contract : 8185/0
      * token id : 00000000
      * amount : 1000
      * exchange rates
        * ccd : 100
        * 7260 : 1
      * List > Submit
        * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=52bf0397a835bbb653b788d029f3c58a5e266ae207fc5bac1e1f73d5ac7f572c)
* select account `4Rw3AxTo8wsEh53cbQ3TidvTeYKioX81aDcThUy5rMDtMcMCyN` (Indian Account)
* Menu > Market
  * You should see 1 token listed
  * SFT token id 00000000
    * buy
      * CCD
      * amount : 1
      * Pay via CCD > Submit
        * [txn](https://testnet.ccdscan.io/transactions?dentity=transaction&dhash=05d6bc937029b18d2d5c7166431c5a8d4b8e7ad9a22e9dca0ba75b996c12e372)
* Menu > Tokens
  * You should see 1 NFT Token & 1 SFT Token