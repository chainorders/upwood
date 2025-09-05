# Concordium RWA Frontend

The new unified frontend application for the Concordium RWA (Real World Asset) platform, built with React 18, Vite, TypeScript, NextUI, and Tailwind CSS.

## Features

- **Modern Stack**: React 18 + Vite + TypeScript + NextUI + Tailwind CSS
- **Comprehensive UI**: Complete registration flow, platform dashboard, and asset management
- **Component Structure**: Organized components for registration, platform features, and shared UI elements
- **Responsive Design**: Mobile-first design with Tailwind CSS
- **Type Safety**: Full TypeScript support throughout the application

## Project Structure

```
src/
├── components/
│   ├── registration/     # User registration flow components
│   ├── platform/         # Platform-specific UI components
│   └── ...              # Shared components (Navigation, Footer, etc.)
├── pages/
│   ├── platform/         # Platform pages (Assets, Projects, Settings)
│   └── ...              # Public pages (Home, About, Contact, etc.)
├── App.tsx              # Main application component
├── main.tsx             # Application entry point
└── index.css            # Global styles
```

## Development

### Prerequisites

- Node.js (>=18.0.0)
- yarn (managed via packageManager field)

### Setup

```bash
# Install dependencies (from root directory)
yarn install

# Start development server
yarn workspace frontend dev

# Build for production
yarn workspace frontend build

# Preview production build
yarn workspace frontend preview

# Run linting
yarn workspace frontend lint
```

### Development Server

The development server runs on `http://localhost:5173` with hot module replacement (HMR) enabled.

## Key Dependencies

- **@nextui-org/react**: Modern UI component library
- **framer-motion**: Animation library for React
- **lucide-react**: Icon library
- **react-router-dom**: Client-side routing
- **tailwindcss**: Utility-first CSS framework

## Integration with Backend

This frontend is designed to integrate with the Concordium RWA backend services:

- API client generation from backend OpenAPI specs
- Concordium blockchain integration via `@concordium/browser-wallet-api-helpers`
- Event-driven state management based on blockchain events

## Environment Configuration

Create a `.env` file in the root directory with the following variables:

```env
VITE_API_BASE_URL=http://localhost:3000
VITE_CONCORDIUM_NODE_URL=https://grpc.testnet.concordium.com:20000
VITE_NETWORK=testnet
```

See `.env.example` for all available configuration options.

## Component Architecture

### Registration Flow
Complete KYC/AML onboarding process with:
- Personal information collection
- Identity verification
- Document upload and verification
- Wallet setup and connection

### Platform Features
- Asset portfolio management
- Active project tracking
- Bond token trading
- Yield and maturity tracking
- Compliance and settings management

### Shared Components
- Navigation and routing
- Modal systems
- Form components
- UI utilities

## Deployment

The frontend is containerized and can be deployed using Docker or directly to cloud platforms:

```bash
# Production build (from root directory)
yarn workspace frontend build

# Serve static files from dist/
```

## Contributing

1. Follow the existing code structure and conventions
2. Use TypeScript for all new components
3. Follow the established component organization patterns
4. Ensure responsive design compatibility
5. Test integration with backend API endpoints

For more information, see the main project documentation in the root WARP.md file.
