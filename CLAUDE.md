# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
# Install dependencies (uses pnpm)
pnpm install

# Run all apps in development mode
pnpm dev

# Build all packages
pnpm build

# Lint all packages
pnpm lint

# Type check all packages
pnpm check-types

# Format code
pnpm format
```

### Smart Contracts (packages/contract)

```bash
# Compile contracts
pnpm contract:compile
# or from package directory:
cd packages/contract && pnpm compile

# Run tests (uses Node test runner, not Mocha)
pnpm contract:test
# or:
cd packages/contract && pnpm test

# Run a single test file
cd packages/contract && npx hardhat test test/Counter.ts
```

### Frontend (apps/frontend)

```bash
# Development server
cd apps/frontend && pnpm dev

# Production build
cd apps/frontend && pnpm build

# Lint
cd apps/frontend && pnpm lint
```

## Architecture

This is a Turborepo monorepo with two workspaces:

### packages/contract
- **Hardhat 3** with `@nomicfoundation/hardhat-toolbox-viem` plugin
- Uses **viem** for contract interactions (not ethers.js)
- Tests use **Node.js test runner** (`node:test`), not Mocha
- Solidity version: 0.8.28
- Network configs: `hardhatMainnet`, `hardhatOp` (simulated), `sepolia` (requires env vars)

### apps/frontend
- **Next.js 16** with App Router
- **React 19**
- **Tailwind CSS v4** (uses `@tailwindcss/postcss`)
- TypeScript 5.9

## Key Technical Details

- **Package Manager**: pnpm 9.0.0
- **Node Version**: 22+ required (22.18.0 in CI)
- Contract tests use `network.connect()` and `viem.deployContract()` pattern
- Turborepo handles build dependencies between packages
