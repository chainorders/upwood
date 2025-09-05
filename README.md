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

### Development Environment Setup (Docker Compose)

1. **Install prerequisites**
   - Docker Engine / Docker Desktop
   - Node.js (optional if you want to run frontend on host)

2. **AWS Setup** (required for full functionality)
   - Install AWS CLI: `curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && unzip awscliv2.zip && sudo ./aws/install`
   - Configure credentials: `aws configure` (use your AWS access key, secret key, and region `eu-west-2`)
   - Or use AWS SSO: `aws configure sso`
   - Verify: `aws sts get-caller-identity`

3. **One-time setup**
   - `cp .env.example .env`
   - `cp backend/upwood/.secure.env.sample backend/upwood/.secure.env` and populate with real wallet JSON strings
   - Ensure your `backend/upwood/.secure.env` contains valid Concordium wallet JSONs for the agents

4. **Start services**
   - `docker compose up -d postgres`
   - `docker compose up --build backend-api backend-listener frontend-app`

5. **Access**
   - Postgres: `localhost:5432`
   - Backend API: `http://localhost:3001` (once AWS credentials are configured)
   - Frontend: `http://localhost:5173`

**Notes:**
- AWS services (S3, SES, Cognito) are accessed directly (no mocks). Containers mount `~/.aws` and use `AWS_PROFILE=default`.
- Database migrations run automatically on backend startup.
- Frontend can also be run on host with `yarn dev`.
- If you don't have AWS credentials configured, the backend services will fail to start.

## 📦 Workspaces

### 🔗 Smart Contracts (`contracts/`)

Concordium blockchain smart contracts written in Rust that enable forest project tokenization, carbon credit management, P2P trading, investment fund management, and comprehensive identity & compliance systems.

📖 **[Full Documentation](./contracts/README.md)**

### 🔧 Backend Services (`backend/`)

Rust-based backend services that process Concordium blockchain events, provide REST APIs for frontend applications, and manage data persistence using the Poem web framework and PostgreSQL databases.

📖 **[Full Documentation](./backend/README.md)**

### 🎨 Frontend Application (`frontend-app/`)

Modern React-based user interface built with Vite and TypeScript that provides an intuitive interface for investors to browse forest projects, manage portfolios, trade carbon credits, and interact with Concordium blockchain wallets.

📖 **[Full Documentation](./frontend-app/README.md)**

### ☁️ AWS Infrastructure (`cdk-deployment/`)

AWS Cloud Development Kit (CDK) infrastructure as code using TypeScript that manages the complete cloud infrastructure including ECS services, databases, CDN, monitoring, and security configurations for production and staging environments.

📖 **[Full Documentation](./cdk-deployment/README.md)**

### 📱 Legacy DApp (`frontend-dapp/`)

Legacy React-based decentralized application interface primarily used for contract administration, testing, and debugging. This interface serves as an administrative tool for developers and administrators for direct smart contract interaction and system testing.

📖 **[Full Documentation](./frontend-dapp/README.md)**

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
# Start backend services with Docker Compose
# In repository root
cp .env.example .env
cp backend/upwood/.secure.env.sample backend/upwood/.secure.env # then populate secrets

docker compose up -d postgres
# Terminal 1
docker compose up --build backend-listener
# Terminal 2
docker compose up --build backend-api
```

### 3. Frontend Development

```bash
# Option A: Run via Docker Compose
docker compose up --build frontend-app

# Option B: Run on host
cd frontend-app
corepack enable && yarn install
VITE_API_BASE_URL=http://localhost:3001 yarn dev --host
```

### 4. Infrastructure Management

```bash
# Run CDK commands on host (no dev containers)
cd cdk-deployment
corepack enable && yarn install
# Example:
yarn cdk synth
# or deploy with appropriate AWS profile/region configured on host
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
