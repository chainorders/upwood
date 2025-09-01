# WARP.md

This file provides guidance to WARP (warp.dev) when working with the legacy frontend DApp workspace.

## Workspace Overview

Legacy React-based decentralized application interface primarily used for contract administration, testing, and debugging. This interface serves as an administrative tool for developers and administrators for direct smart contract interaction and system testing.

## Development Environment

### Prerequisites
- Uses VS Code dev container (`.devcontainer/frontend-dapp/`)
- Node.js with Vite development server
- Material-UI (MUI) for UI components
- React JSON Schema Form for dynamic contract interaction

### Container Setup
```bash
# Open in VS Code dev container
# Dev Containers: Reopen in Container → Select "frontend-dapp"
# Container auto-installs dependencies and shows available scripts
```

## Core Development Commands

### Development Server
```bash
yarn dev                        # Start development server with HMR (vite --host)
yarn preview                    # Preview production build locally
```

### Building
```bash
yarn build                      # Production build (tsc && vite build)
```

### Code Quality
```bash
yarn lint                       # Run ESLint checks
yarn lint:fix                   # Auto-fix ESLint issues
yarn format                     # Format code with Prettier
```

### Deployment
```bash
yarn predeploy                  # Build before deployment
yarn deploy                     # Deploy to GitHub Pages (gh-pages -d dist)
```

## Project Architecture

### Technology Stack
- **React 18** - Modern React with TypeScript
- **Vite** - Fast build tool and development server
- **Material-UI (MUI)** - Material Design component library
- **React JSON Schema Form** - Dynamic form generation for contract interaction

### Key Libraries
- **@concordium/browser-wallet-api-helpers** - Concordium wallet integration
- **@concordium/web-sdk** - Blockchain interactions
- **@rjsf/core & @rjsf/mui** - JSON Schema Form with Material-UI integration
- **react-router-dom** - Client-side routing
- **localforage** - Local storage management

## Primary Use Cases

### Contract Administration
- Direct smart contract interaction
- Contract deployment testing
- Administrative function calls
- Contract state inspection

### Development & Testing
- Contract development testing interface
- Blockchain transaction debugging
- Wallet integration testing
- Contract parameter validation

### System Testing
- End-to-end testing of smart contract flows
- Integration testing with Concordium blockchain
- User acceptance testing for contract functionality

## Development Patterns

### Component Structure
```typescript
// React component with MUI
import { Button, Paper, Typography } from '@mui/material';

interface ComponentProps {
  // prop definitions
}

export const AdminComponent: React.FC<ComponentProps> = ({ props }) => {
  return (
    <Paper>
      <Typography variant="h4">Admin Interface</Typography>
      {/* Component logic */}
    </Paper>
  );
};
```

### Contract Interaction Pattern
```typescript
// Direct contract interaction
import { detectConcordiumProvider } from '@concordium/browser-wallet-api-helpers';

const provider = await detectConcordiumProvider();
const result = await provider.sendTransaction(
  account,
  contractAddress,
  receiveName,
  parameter
);
```

### JSON Schema Forms
```typescript
// Dynamic form generation for contract parameters
import Form from '@rjsf/mui';

const schema = {
  type: "object",
  properties: {
    // contract parameter schema
  }
};

<Form
  schema={schema}
  onSubmit={handleContractCall}
  validator={validator}
/>
```

## Administrative Features

### Contract Management
- Deploy new contract instances
- Update contract parameters
- Manage contract permissions
- Monitor contract events

### User Management
- Identity registry administration
- Whitelist/blacklist management
- Agent management
- Permission assignment

### Transaction Management
- Transaction status monitoring
- Failed transaction debugging
- Gas estimation testing
- Transaction history review

## Wallet Integration

### Concordium Wallet Connection
```typescript
// Connect to wallet for admin operations
const provider = await detectConcordiumProvider();
await provider.connect();

// Get admin account
const account = await provider.getMostRecentlySelectedAccount();

// Sign administrative transactions
const signature = await provider.signMessage(account, message);
```

### Administrative Transactions
```typescript
// Execute admin functions
const txHash = await provider.sendTransaction(
  adminAccount,
  contractAddress,
  'adminFunction',
  parameters,
  maxEnergy
);
```

## Testing & Debugging

### Contract Testing Interface
- Form-based parameter input
- Schema validation for contract parameters
- Transaction result display
- Error message debugging

### Blockchain Debugging
- Event log viewing
- Transaction status checking
- Contract state inspection
- Network connectivity testing

## Environment Configuration

### Development Settings
- **Port**: Vite dev server runs on default port (usually 5173)
- **Network**: Configurable for testnet/mainnet
- **Wallet**: Requires Concordium browser wallet extension

### Local Storage
- Uses `localforage` for persistent admin settings
- Stores contract addresses and configuration
- Maintains user preferences and session data

## Build Configuration

### Vite Configuration
- **Dev Server**: Hot module replacement enabled
- **Production**: Optimized for GitHub Pages deployment
- **Config File**: `vite.config.ts`

### TypeScript Configuration
- Strict mode enabled for type safety
- Material-UI types included
- React JSON Schema Form types configured

## Common Administrative Tasks

### Adding New Contract Interface
1. Create new contract interaction component
2. Define JSON schema for contract parameters
3. Add contract address configuration
4. Implement transaction handlers
5. Add navigation route

### Testing Contract Deployment
1. Use deployment forms to test contract initialization
2. Verify contract addresses and parameters
3. Test administrative functions
4. Validate contract events and state changes

### Debugging Contract Issues
1. Use transaction status tools
2. Inspect contract events in detail
3. Test parameter validation
4. Check wallet connection and permissions

## Deployment Strategy

### GitHub Pages Deployment
- Built files deployed to `gh-pages` branch
- Automatic deployment via `yarn deploy`
- Accessible for testing and admin use
- Static hosting suitable for admin interface

### Development Workflow
1. Develop and test locally with `yarn dev`
2. Build and verify with `yarn build`
3. Preview with `yarn preview`
4. Deploy to GitHub Pages with `yarn deploy`

## Material-UI Theming

### Component Usage
- Consistent Material Design components
- Form controls for contract interaction
- Navigation and layout components
- Data display components for results

### Styling Approach
- MUI theming for consistent appearance
- Emotion-based styling system
- Responsive design for various screen sizes
- Dark/light theme support (if configured)

## Prettier Configuration
```json
{
  "tabWidth": 2,
  "useTabs": true
}
```

## Important Notes

- **Legacy Status**: This is the legacy admin interface - new features should be added to `frontend-app/`
- **Admin Purpose**: Primarily for developers and administrators, not end-users
- **Testing Tool**: Main use case is contract testing and administrative functions
- **Direct Integration**: Directly interacts with smart contracts without backend API layer
