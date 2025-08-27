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

Concordium blockchain smart contracts written in Rust that enable forest project tokenization, carbon credit management, P2P trading, investment fund management, and comprehensive identity & compliance systems.

📖 **[Full Documentation](./contracts/README.md)**

🐳 **Dev Container**: Open with `Dev Containers: Reopen in Container` → Select **contracts** → Container files: [`.devcontainer/contracts/`](./.devcontainer/contracts/)

### 🔧 Backend Services (`backend/`)

Rust-based backend services that process Concordium blockchain events, provide REST APIs for frontend applications, and manage data persistence using the Poem web framework and PostgreSQL databases.

📖 **[Full Documentation](./backend/README.md)**

🐳 **Dev Container**: Open with `Dev Containers: Reopen in Container` → Select **backend** → Container files: [`.devcontainer/backend/`](./.devcontainer/backend/)

### 🎨 Frontend Application (`frontend-app/`)

Modern React-based user interface built with Vite and TypeScript that provides an intuitive interface for investors to browse forest projects, manage portfolios, trade carbon credits, and interact with Concordium blockchain wallets.

📖 **[Full Documentation](./frontend-app/README.md)**

🐳 **Dev Container**: Open with `Dev Containers: Reopen in Container` → Select **frontend-app** → Container files: [`.devcontainer/frontend-app/`](./.devcontainer/frontend-app/)

### ☁️ AWS Infrastructure (`cdk-deployment/`)

AWS Cloud Development Kit (CDK) infrastructure as code using TypeScript that manages the complete cloud infrastructure including ECS services, databases, CDN, monitoring, and security configurations for production and staging environments.

📖 **[Full Documentation](./cdk-deployment/README.md)**

🐳 **Dev Container**: Open with `Dev Containers: Reopen in Container` → Select **cdk-deployment** → Container files: [`.devcontainer/cdk-deployment/`](./.devcontainer/cdk-deployment/)

### 📱 Legacy DApp (`frontend-dapp/`)

Legacy React-based decentralized application interface primarily used for contract administration, testing, and debugging. This interface serves as an administrative tool for developers and administrators for direct smart contract interaction and system testing.

📖 **[Full Documentation](./frontend-dapp/README.md)**

🐳 **Dev Container**: Open with `Dev Containers: Reopen in Container` → Select **frontend-dapp** → Container files: [`.devcontainer/frontend-dapp/`](./.devcontainer/frontend-dapp/)

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
# Rebuild backend container to reset database
Dev Containers: Rebuild Container

# Or use backend workspace database commands
# (Open backend container first)
diesel database reset
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
