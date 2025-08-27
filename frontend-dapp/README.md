# 📱 Legacy DApp Workspace

**Legacy React-based decentralized application interface for Upwood's Concordium RWA platform, primarily used for contract administration and testing.**

## 🌟 Overview

This workspace contains the legacy DApp interface built with React 18, Material-UI, and Vite. It serves as an administrative tool for contract interactions, testing, and debugging. This interface is primarily used by developers and administrators for direct smart contract interaction and system testing.

## ⚠️ Important Notice

**This is a legacy workspace.** New frontend development should focus on the `frontend-app` workspace, which provides the modern user interface for end users. This DApp interface is maintained for:

- Contract administration and testing
- Developer tools and debugging interfaces  
- Direct smart contract interaction
- System integration testing

## 🏗️ Directory Structure

```
frontend-dapp/
├── dist/                        # Built application files
├── public/                      # Static assets
├── src/                         # Source code
│   ├── assets/                 # Application assets
│   ├── components/             # UI components
│   │   ├── common/            # Common UI components
│   │   └── contracts/         # Contract interaction components
│   └── lib/                   # Utility libraries
│       └── generated/         # Generated contract clients
├── index.html                  # HTML template
├── tsconfig.json              # TypeScript configuration
├── vite.config.ts             # Vite build configuration
└── package.json               # Dependencies and scripts
```

## 🚀 Development Environment Setup

### Using VS Code Dev Containers

1. **Open VS Code in repository root**

   ```bash
   cd /path/to/concordium-rwa
   code .
   ```

2. **Open in Dev Container**
   - Press `F1` or `Ctrl+Shift+P`
   - Type: `Dev Containers: Reopen in Container`
   - Select: **frontend-dapp**

3. **Container Setup**
   - The container automatically installs Node.js and dependencies
   - Vite development server will be available on port 5173
   - Terminal shows available yarn scripts upon completion

## 🛠️ Available Scripts

All scripts are defined in `package.json` and can be run with `yarn <script>`:

### Development

```bash
yarn dev        # Start development server with hot reload (vite --host)
yarn preview    # Preview production build locally
```

### Building & Deployment

```bash
yarn build      # Build for production (tsc && vite build)
yarn predeploy  # Pre-deployment build step (runs yarn build)
yarn deploy     # Deploy to GitHub Pages (gh-pages -d dist)
```

### Code Quality

```bash
yarn lint       # Run ESLint checks
yarn lint:fix   # Run ESLint and fix auto-fixable issues
yarn format     # Format code with Prettier
```

## 🎯 Key Features

### 🔧 Contract Administration

- Direct smart contract interaction interface
- Contract deployment and initialization
- Administrative function access
- Contract state inspection and debugging

### 🧪 Testing Interface

- Contract function testing with form-based inputs
- Transaction result inspection
- Error handling and debugging
- Multi-contract workflow testing

### 📊 Developer Tools

- JSON-RPC interface for contract calls
- Transaction history and analysis
- Event log monitoring
- Contract schema validation

### 🔐 Wallet Integration

- Concordium Browser Wallet integration
- Administrative account management
- Multi-account testing capabilities
- Transaction signing interface

## 🏢 Technical Architecture

### Frontend Stack

- **React 18** - Modern React with hooks
- **TypeScript** - Type-safe development
- **Material-UI v5** - Component library
- **Vite** - Fast build tool and development server
- **React JSON Schema Form** - Dynamic form generation

### Key Libraries

- **@concordium/browser-wallet-api-helpers** - Concordium wallet integration
- **@concordium/web-sdk** - Concordium blockchain interactions
- **@mui/material** - Material-UI components
- **@rjsf/mui** - JSON Schema forms with Material-UI
- **react-router-dom** - Client-side routing
- **localforage** - Client-side storage

### Development Tools

- **ESLint** - Code linting with TypeScript support
- **Prettier** - Code formatting
- **TypeScript** - Static type checking

## 🔄 Contract Integration

### Generated Contract Clients

The DApp uses generated TypeScript clients from smart contracts:

```typescript
// Generated clients in src/lib/generated/
import { SecuritySftMultiClient } from './lib/generated/security-sft-multi';
import { IdentityRegistryClient } from './lib/generated/identity-registry';

// Usage
const contractClient = new SecuritySftMultiClient(wallet, contractAddress);
const result = await contractClient.mint(params);
```

### Dynamic Form Generation

Contract interactions use JSON Schema forms:

```typescript
// Dynamic forms based on contract schemas
<Form
  schema={contractSchema}
  formData={formData}
  onSubmit={handleContractCall}
  validator={validator}
/>
```

## 📋 Usage Guidelines

### For Developers

1. Use this interface for contract testing and debugging
2. Validate contract functions before integration
3. Test multi-step workflows and edge cases
4. Debug transaction failures and error conditions

### For Administrators

1. Deploy and initialize new contracts
2. Manage contract permissions and agents
3. Monitor contract state and events
4. Perform administrative operations

### Migration Notes

- **New feature development**: Use `frontend-app` workspace
- **User-facing interfaces**: Implement in `frontend-app`
- **Administrative tools**: Can remain in this workspace
- **Testing utilities**: Keep for development and QA processes

---

**📱 Legacy interface maintained for administrative and testing purposes. New development should use the `frontend-app` workspace for user-facing features.**
