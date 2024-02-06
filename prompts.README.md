# Concordium RWA ; Frontend Code Generation Prompts

Concordium is a Blockchain with built in Identity Management and Privacy features. Concordium RWA is a project to build a Decentralized Application to manage Real World Assets on the Concordium Blockchain.
Each Concordium Account address is a base58 encoded string. The Concordium Browser Wallet is a browser extension that allows users to manage their Concordium Accounts. The Concordium Browser Wallet is available for Chrome and Firefox.

This Project uses React and Typescript. With React Material UI as the UI framework.
Project uses Github Copilot for code generation.
The Contracts that the Project interacts with are defined in the [Contracts Readme](./contracts/README.md).
Projects uses multiple components with a React Router to navigate between them.
Project users react Reducer to manage state.

The Layout of the project UI is as follows:

* Header at the top with a navigation bar.
  * The heading Should be "Concordium RWA".
  * The Header Bar should also have a dropdown to select a Concordium Account Address. The dropdown should have a list of all the user accounts that the user has access to. Which can be queried from the Concordium Browser Wallet. which can be fetched using the code below. The account address should be displayed as a string. `requestAccounts` returns a `Promise<Account[]>`

  ```typescript
   detectConcordiumProvider()
  .then((provider) => {
   provider
    .requestAccounts()
    .then((accounts) => {
     console.log(accounts);
    })
    .catch((error) => {
     console.log(error);
    });
  })
  .catch((error) => {
   console.log(error);
  });
  ```

* A Navigation Bar in the Left with a link to each Contract Page and each Project Workflows. The navigation bar should always be visible.
* Sticky Footer at the bottom with the Concordium Logo and links to concordium developer documentation.
* Extreme right side should be a bar with the list of Contract Events. This bar should be visible only when the user is on a Contract Page.
* Center of the page should be the main content of the page. This should be a component that is rendered based on the URL.
