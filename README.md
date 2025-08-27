# Upwood Concordium RWA Platform

**A comprehensive blockchain-based Real World Asset management system on Concordium blockchain, specializing in forest project tokenization and environmental asset management.**

## 🌟 Project Overview

Upwood's Concordium RWA (Real World Asset) platform is designed to tokenize and manage forest projects, enabling transparent investment, trading, and yield distribution of environmental assets. The system provides a complete ecosystem for forest project bonds, carbon credit tokenization, P2P trading, and investor rewards.

### Key Features

- 🌳 **Forest Project Tokenization** - Represent forest projects as blockchain assets
- 🍃 **Carbon Credit Management** - Tokenize and trade carbon credits on-chain
- 💰 **Investment Fund Management** - Bond and tranche-based forest project funding
- 🔄 **P2P Trading Platform** - Direct trading of environmental assets
- 🏦 **Yield Distribution** - Automated reward distribution to investors
- 🔐 **Identity & Compliance** - Comprehensive access control and regulatory compliance
- ⚡ **Multi-Workspace Architecture** - Modular development environment with specialized containers

## 🏗️ Architecture Overview

The platform consists of five main workspaces, each optimized for specific development tasks:

```
concordium-rwa/
├── 📦 contracts/           # Concordium Smart Contracts (Rust)
├── 🔧 backend/            # Event Listener & API Services (Rust)
├── 🎨 frontend-app/       # User Interface (React + Vite)
├── ☁️  cdk-deployment/     # AWS Infrastructure (TypeScript + CDK)
└── 📱 frontend-dapp/      # Legacy DApp Interface (React)
```

## 🚀 Quick Start

### Repository Setup

```bash
git clone git@github.com:chainorders/concordium-rwa.git
cd concordium-rwa
git submodule update --init --recursive
```

### Development Environment Setup

1. **Install Prerequisites**
   - [Docker Desktop](https://www.docker.com/products/docker-desktop/)
   - [VS Code](https://code.visualstudio.com/)
   - [Dev Containers Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

2. **Choose Your Workspace**
   - Open VS Code in the repository root
   - Run `Dev Containers: Reopen in a Container`
   - Select your target workspace from the list

3. **Start Development**
   - Each container automatically runs `yarn run` to show available scripts
   - Follow workspace-specific instructions below

## 📦 Workspaces

### 🔗 Smart Contracts (`contracts/`)

**Purpose**: Concordium blockchain smart contracts written in Rust

**Container**: `.devcontainer/contracts/`

**Key Components**:

- 🌳 `security-sft-multi` - Forest project representation contract
- 🍃 `security-sft-single` - Carbon credit tokenization
- 🔄 `security-p2p-trading` - P2P trading marketplace
- 💰 `security-mint-fund` - Investment fund management with bond support
- 🔐 `rwa-identity-registry` - Identity & access control
- 📝 `rwa-compliance` - Regulatory compliance (deprecated)
- 🎁 `nft-multi-rewarded` - Authentication NFTs
- 💎 `offchain-rewards` - Off-chain reward claims (deprecated)
- 🏆 `security-sft-multi-yielder` - Yield distribution (deprecated)

**Setup Requirements**:

```bash
# Copy your testnet wallet to:
cp your-wallet.export .devcontainer/contracts/default_account.export
```

**Key Scripts**:

- `yarn build` - Compile all contracts
- `yarn test` - Run contract tests
- `yarn deploy` - Deploy contracts to testnet

### 🔧 Backend Services (`backend/`)

**Purpose**: Rust-based backend services for blockchain event processing and API management

**Container**: `.devcontainer/backend/`

**Key Components**:

- 📡 **Events Listener** (`events_listener/`) - Concordium blockchain event processor
- 🔗 **API Services** (`upwood/`) - RESTful API using Poem framework
- 📚 **Shared Library** (`shared/`) - Common database models and utilities
- 🧪 **Shared Tests** (`shared_tests/`) - Integration test utilities

**Services**:

- **Listener**: Processes blockchain events and updates database
- **API**: Provides REST endpoints for frontend applications
- **Verifier**: Identity verification service
- **Sponsor**: Transaction sponsorship service (deprecated)

**Key Scripts**:

- `yarn debug:listener` - Start blockchain event listener
- `yarn debug:contracts-api` - Start REST API server
- `yarn debug:verifier-api` - Start identity verification service
- `yarn generate:client` - Generate API client code

**Environment Setup**:

```bash
# Required environment variables:
CONCORDIUM_NODE_URI=https://grpc.testnet.concordium.com:20000
MONGODB_URI=mongodb://localhost:27017/concordium_rwa
WEB_SERVER_ADDR=0.0.0.0:3000
```

### 🎨 Frontend Application (`frontend-app/`)

**Purpose**: Modern React-based user interface built with Vite

**Container**: `.devcontainer/frontend-app/`

**Technology Stack**:

- ⚛️ React 18 with TypeScript
- ⚡ Vite for development and building
- 🎨 Modern UI components
- 🔗 Concordium wallet integration

**Key Features**:

- Forest project browsing and investment
- Carbon credit marketplace
- Portfolio management
- Transaction history
- Wallet integration

**Key Scripts**:

- `yarn dev` - Start development server (port 5173)
- `yarn build` - Build production assets
- `yarn preview` - Preview production build
- `yarn lint` - Code linting

### ☁️ AWS Infrastructure (`cdk-deployment/`)

**Purpose**: AWS infrastructure deployment using TypeScript and CDK

**Container**: `.devcontainer/cdk-deployment/`

**Infrastructure Components**:

- 🖥️ ECS services for backend applications
- 🗄️ RDS PostgreSQL databases
- 🌐 CloudFront CDN for frontend
- 🔒 IAM roles and security policies
- 📊 CloudWatch monitoring

**Setup Requirements**:

```bash
# Create AWS credentials file:
cp aws_credentials.csv.example .devcontainer/cdk-deployment/aws_accessKeys.csv
# Edit with your AWS credentials
```

**Key Scripts**:

- `yarn deploy` - Deploy infrastructure to AWS
- `yarn diff` - Show infrastructure changes
- `yarn destroy` - Remove deployed infrastructure
- `yarn synth` - Generate CloudFormation templates

### 📱 Legacy DApp (`frontend-dapp/`)

**Purpose**: Legacy React-based decentralized application interface

**Container**: `.devcontainer/frontend-dapp/`

**Note**: This is a legacy interface primarily used for contract administration and testing. New development should focus on `frontend-app/`.

## 🛠️ Development Workflow

### 1. Contract Development

```bash
# Open contracts workspace
code . && Dev Containers: Reopen in Container → contracts

# Develop and test contracts
yarn build
yarn test
yarn deploy
```

### 2. Backend Development

```bash
# Open backend workspace
code . && Dev Containers: Reopen in Container → backend

# Start services
yarn debug:listener    # Terminal 1: Blockchain listener
yarn debug:contracts-api  # Terminal 2: API server
```

### 3. Frontend Development

```bash
# Open frontend workspace
code . && Dev Containers: Reopen in Container → frontend-app

# Start development server
yarn dev  # Runs on http://localhost:5173
```

### 4. Infrastructure Management

```bash
# Open CDK workspace
code . && Dev Containers: Reopen in Container → cdk-deployment

# Deploy to AWS
yarn deploy
```

## 🔧 Advanced Configuration

### Apple Silicon Development

If developing on Apple Silicon (M1/M2), update the following files:

```json
// Change VARIANT to "bullseye" in:
// .devcontainer/contracts/devcontainer.json
// .devcontainer/backend/docker-compose.yml
// .devcontainer/frontend-dapp/devcontainer.json
{
  "build": {
    "args": {
      "VARIANT": "bullseye"  // Changed from "bookworm"
    }
  }
}
```

### Database Setup

The backend workspace includes PostgreSQL database setup:

```bash
# Database runs automatically in backend container
# Connection: postgresql://postgres:password@localhost:5432/concordium_rwa

# Run migrations
diesel migration run

# Reset database
diesel database reset
```

### Blockchain Configuration

```bash
# Testnet configuration
CONCORDIUM_NODE_URI=https://grpc.testnet.concordium.com:20000
NETWORK=testnet

# Mainnet configuration (production)
CONCORDIUM_NODE_URI=https://grpc.mainnet.concordium.software:20000
NETWORK=mainnet
```

## 📚 Additional Resources

- **[Contracts Documentation](./contracts/README.md)** - Detailed smart contract specifications
- **[Backend Documentation](./backend/README.md)** - API documentation and service architecture
- **[Concordium Documentation](https://docs.concordium.com/)** - Concordium blockchain documentation
- **[Forest Project Specifications](./docs/forest-projects.md)** - Business logic and project specifications

## 🤝 Contributing

1. **Choose appropriate workspace** for your contribution
2. **Follow workspace-specific coding standards**:
   - Contracts: Rust with Concordium standards
   - Backend: Rust with Poem framework
   - Frontend: React with TypeScript
   - Infrastructure: TypeScript with AWS CDK
3. **Write comprehensive tests**
4. **Update documentation** as needed

## 🚨 Troubleshooting

### Common Issues

**Container won't start**:

```bash
# Rebuild container
Dev Containers: Rebuild Container

# Clear Docker cache
docker system prune -a
```

**Database connection issues**:

```bash
# Reset database in backend container
docker-compose down -v
docker-compose up -d
```

**Wallet connection problems**:

```bash
# Ensure wallet file exists:
ls -la .devcontainer/contracts/default_account.export

# Check wallet format and permissions
```

### Getting Help

- 📧 **Internal Team**: Use internal development channels
- 🐛 **Bug Reports**: Create detailed issue reports with workspace context
- 💡 **Feature Requests**: Include business case and technical requirements

---

**Built with ❤️ for sustainable forest management and environmental impact.**
