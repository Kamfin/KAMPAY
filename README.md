# KAMPAY

Kampay is an AI-first, autonomous payroll protocol built on the Stellar blockchain — and at its core, it's a protocol anyone can own.

Every transaction processed through Kampay generates fee revenue. That revenue flows into a Protocol Treasury. And anyone — individuals, organisations, workers — can purchase KPAY shares and earn a direct cut of that revenue as monthly dividends. No VC gatekeeping. No minimum buy-in. Just buy shares, hold them in your Stellar wallet, and get paid every month — automatically, on-chain.

The global payroll industry processes over **$8 trillion annually**, yet the people and businesses powering that volume see none of the upside. They pay 3–8% in processing fees, wait 3–5 business days for settlements, and hand their money to intermediaries who pocket the margin.

Kampay flips that model. Payroll runs as a Soroban smart contract on Stellar — settling in 3–5 seconds for fractions of a cent. An AI Treasury Agent handles scheduling, FX hedging, and cash flow autonomously. And the fee revenue that would have gone to a processor goes instead to KPAY shareholders: the workers, employers, and community members who believe in the protocol and buy into it.

Stellar makes this possible. With native USDC support, a built-in DEX, 3–5 second finality, and transaction costs of ~$0.000001, Stellar is the only public blockchain built from the ground up for payments at global scale. Kampay doesn't work around Stellar's architecture — it's built directly on top of it.

**Core Value Propositions**

- **Streaming payroll** — workers are paid per second, not per month
- **AI Treasury Agent** — autonomous payroll scheduling, FX hedging, and cash flow forecasting
- **On-chain compliance** — tax calculations and reporting enforced by smart contracts
- **Shareholder model** — buy KPAY shares, earn dividends from protocol fee revenue
- **Global by default** — no banking infrastructure required on either side

---

## 2. The Problem Space

### 2.1 What Is Broken in Global Payroll

| Problem            | Current Reality              | Kampay Solution              |
| ------------------ | ---------------------------- | ---------------------------- |
| Settlement speed   | 3–5 business days via SWIFT  | 3–5 seconds on Stellar       |
| Transaction fees   | 3–8% via payment processors  | ~$0.000001 per operation     |
| Geographic access  | Requires local banking rails | Wallet address is enough     |
| Payroll management | Manual HR + finance ops      | AI Agent automates all of it |
| Transparency       | Black-box intermediaries     | Full on-chain audit trail    |
| Ownership          | Shareholders are VCs only    | Anyone can buy KPAY shares   |

### 2.2 The Workforce Has Changed

- 1.4 billion adults globally are unbanked — yet many work for international clients
- Gig and freelance workers now represent over **35% of the US workforce** alone
- Cross-border remote work surged **300%+** post-2020
- Workers increasingly demand real-time, streaming compensation — not monthly batches

Kampay is built for this workforce — not the one payroll processors built for in 1995.

---

## 3. Product Overview

### 3.1 Streaming Payroll Engine

Traditional payroll fires once a month. Kampay's streaming engine pays continuously — per second, per milestone, or per any custom interval — using Soroban smart contracts on Stellar.

**How It Works**

1. Employer deposits USDC or XLM into a Kampay payroll vault contract
2. For each worker, a streaming payment contract is created with: rate per second, start time, cliff/vesting rules (optional), auto-stop conditions
3. Workers can withdraw their earned balance at any time — no waiting for payday
4. If the vault runs low, the AI Treasury Agent tops it up automatically

**Key Features**

- Real-time earnings dashboard — watch balance grow live
- Configurable: stream by second, hour, milestone, or deliverable
- Multi-currency: USDC, XLM, or any Stellar-native asset
- Instant withdrawals: settled in 3–5 seconds, no bank account needed
- Programmable stop conditions: pause on dispute, stop on contract end

### 3.2 AI Treasury Agent

The AI Treasury Agent monitors, manages, and optimizes the employer's entire payroll treasury autonomously — replacing manual finance ops.

| Capability            | Description                                                           |
| --------------------- | --------------------------------------------------------------------- |
| Cash Flow Forecasting | Predicts treasury runway; alerts when vault will run dry              |
| Auto-Replenishment    | Triggers top-up from funding wallet when balance < threshold          |
| FX Hedging            | Uses Stellar DEX to swap assets at optimal rates                      |
| Anomaly Detection     | Flags duplicate payments, unusual withdrawals, contract modifications |
| Payroll Scheduling    | Optimizes batch payment timing to minimize on-chain fees              |
| Reporting & Audit     | Generates payroll summaries, tax reports, and on-chain audit logs     |

### 3.3 Compliance Engine

Compliance is built into the contract layer, not the paper layer.

- W-8/W-9 equivalent data collected at wallet onboarding
- Withholding rates configurable per jurisdiction — enforced at the smart contract level
- Automatic 1099/invoice generation for contractors at period close
- Immutable on-chain record of every payment — audit-proof by default
- KYC/AML hooks via Stellar's native trustline and auth flags

### 3.4 Worker Portal

Every worker gets a clean, mobile-first portal with live streaming balance, payment history, withdrawal controls, tax summary, and KPAY share allocation.

- No bank account required — Freighter wallet connects in one click
- Real-time earnings ticker (balance updates every second)
- One-tap withdrawal to any Stellar address or connected ramp
- Built-in currency conversion via Stellar DEX

---

## 4. The KPAY Shareholder Model

Anyone can purchase KPAY shares and earn dividends from the protocol's fee revenue — turning Kampay from a payroll tool into a community-owned financial protocol.

### 4.1 Model Overview

|                      |                                                        |
| -------------------- | ------------------------------------------------------ |
| Protocol Fee         | 0.25% of each payroll transaction                      |
| Fee Destination      | Protocol Treasury smart contract (on-chain, auditable) |
| Distribution Cadence | Monthly dividends to KPAY shareholders                 |
| Share Token          | KPAY — Stellar-native asset with on-chain governance   |
| Share Purchase       | Via Kampay app or Stellar DEX                          |
| Minimum Buy-In       | None — fractional shares supported                     |

### 4.2 Share Purchase Flow

**For Individuals**

1. Connect Freighter wallet to the Kampay app
2. Navigate to the Shareholders section
3. Choose a share tier or enter a custom USDC/XLM amount
4. KPAY tokens are minted and sent to your wallet on-chain
5. Monthly dividends begin accruing immediately

**For Organisations**

1. Join Kampay as a payroll client
2. Purchase a strategic KPAY allocation during onboarding
3. Distribute shares to employees as a compensation benefit
4. Employee KPAY vests on a configurable schedule via Soroban contracts
5. Employees earn dividends on vested shares while receiving streaming payroll

### 4.3 KPAY Share Tiers

| Tier          | KPAY Amount | Price (USDC) | Annual Dividend Estimate\* | Governance Weight |
| ------------- | ----------- | ------------ | -------------------------- | ----------------- |
| Contributor   | 100 KPAY    | $50          | ~$3–8                      | 1x                |
| Builder       | 500 KPAY    | $225         | ~$15–40                    | 1x                |
| Operator      | 2,000 KPAY  | $800         | ~$60–160                   | 1.5x              |
| Architect     | 10,000 KPAY | $3,500       | ~$300–800                  | 2x                |
| Institutional | Custom      | Custom       | Negotiated                 | 3x                |

_Illustrative estimates based on protocol volume. Not guaranteed returns._

### 4.4 Dividend Distribution

At month end, the Protocol Treasury contract automatically:

1. Calculates distributable balance (70% of fee revenue; 30% retained for ops/dev)
2. Computes each holder's share: `(holder KPAY / total KPAY supply) × distributable balance`
3. Sends distributions directly to shareholder wallets — no claim required
4. Publishes full distribution ledger on-chain and in the Kampay dashboard

### 4.5 Governance

KPAY holders vote on-chain on:

- Protocol fee rate adjustments
- Treasury allocation ratios
- New feature prioritisation
- Smart contract upgrade proposals
- New market or jurisdiction expansion

Votes are weighted by tier. Quorum: **10% of circulating KPAY**.

---

## 5. Technical Architecture

### 5.1 System Overview

| Layer           | Description                                                                                   |
| --------------- | --------------------------------------------------------------------------------------------- |
| Frontend        | Next.js 14 + TypeScript — employer dashboard, worker portal, shareholder hub                  |
| Wallet          | Freighter — signs all on-chain transactions; no private keys leave the device                 |
| AI Agent        | Python + LangChain — persistent background service with access to Kampay's internal API       |
| Smart Contracts | Soroban (Rust/WASM) — PayrollVault, StreamingPayment, ShareholderProtocol, ComplianceRegistry |
| Data Indexing   | Stellar Horizon API — transaction history, wallet balances, contract state                    |
| Stellar DEX     | Native on-chain swaps for FX hedging and USDC conversions via SDEX/AMM                        |

### 5.2 Smart Contracts

**`kampay-payroll-vault`**
Primary treasury contract per employer. Holds funds, tracks active streams, enforces minimum balances, emits events to the AI Agent.

- Functions: `deposit()`, `withdraw()`, `createStream()`, `cancelStream()`, `getBalance()`
- Events: `VaultLow`, `StreamCreated`, `StreamCancelled`, `PaymentMade`

**`kampay-streaming-payment`**
Deployed per worker engagement. Enforces payment rate, start/end conditions, cliff logic, pause/resume.

- Functions: `withdraw()`, `pause()`, `resume()`, `terminate()`, `earned()`
- Configurable: rate (per second), cliff (seconds), stop conditions

**`kampay-shareholder-protocol`**
Manages KPAY minting, vesting, governance voting, and dividend distributions.

- Functions: `purchaseShares()`, `vestShares()`, `claimDividend()`, `vote()`, `propose()`
- Governance: 48-hour time-lock on approved proposals

**`kampay-compliance-registry`**
Stores jurisdiction compliance parameters, enforces withholding, validates KYC via trustline flags.

- Functions: `registerWorker()`, `setWithholding()`, `generateReport()`, `flagAnomaly()`

### 5.3 AI Agent Tools

| Tool                    | Description                                 | Trigger                            |
| ----------------------- | ------------------------------------------- | ---------------------------------- |
| `checkVaultBalance`     | Queries PayrollVault via Horizon            | `VaultLow` event or scheduled poll |
| `topUpVault`            | Submits top-up from employer funding wallet | Balance < threshold                |
| `executeDEXSwap`        | FX conversion via Stellar DEX               | Cross-currency payroll event       |
| `generatePayrollReport` | Compiles Horizon payment history            | On demand or monthly               |
| `flagAnomaly`           | Writes flag to ComplianceRegistry           | Irregular activity detected        |
| `schedulePayment`       | Queues batch payment for optimal timing     | Payroll cycle trigger              |
| `distributeDividends`   | Triggers monthly KPAY distribution          | Monthly cron                       |

### 5.4 Tech Stack

| Layer            | Technology                          | Purpose                                            |
| ---------------- | ----------------------------------- | -------------------------------------------------- |
| Blockchain       | Stellar (Mainnet + Testnet)         | Settlement, asset issuance, DEX                    |
| Smart Contracts  | Soroban (Rust / WASM)               | Payroll logic, shareholder protocol, compliance    |
| Stable Asset     | USDC on Stellar (Circle)            | Stable-value payroll and dividends                 |
| Blockchain SDK   | `@stellar/stellar-sdk`              | Frontend and agent on-chain interactions           |
| Frontend         | Next.js 14, TypeScript, React 18    | Employer dashboard, worker portal, shareholder hub |
| Wallet           | Freighter                           | Browser wallet, transaction signing                |
| Data Indexing    | Stellar Horizon API                 | Transaction history, contract state                |
| AI Agent Runtime | Python, LangChain, FastAPI          | Autonomous treasury management                     |
| AI Model         | Claude (Anthropic API)              | Agent reasoning and decision-making                |
| Database         | PostgreSQL + Prisma                 | Off-chain agent state, user preferences            |
| Auth             | JWT + Stellar wallet signature      | Passwordless, wallet-native authentication         |
| Hosting          | Vercel (frontend) + Railway (agent) | Deployment infrastructure                          |

---

## 6. Monorepo Structure

```
kampay/
├── frontend/                  # Next.js 14 — employer dashboard, worker portal, shareholder hub
│   ├── app/
│   │   ├── dashboard/         # Employer payroll management
│   │   ├── worker/            # Worker earnings portal
│   │   ├── shareholders/      # KPAY share purchase + dividend tracking
│   │   └── onboarding/        # Wallet connect + KYC flow
│   └── components/
├── contracts/                 # Soroban smart contracts (Rust)
│   ├── payroll-vault/
│   ├── streaming-payment/
│   ├── shareholder-protocol/
│   └── compliance-registry/
├── agent/                     # AI Treasury Agent (Python)
│   ├── tools/                 # Agent tool definitions
│   ├── prompts/               # System prompts and agent persona
│   └── scheduler/             # Cron jobs and event listeners
├── packages/                  # Shared libraries
│   ├── stellar-utils/         # Horizon queries, tx builders
│   └── kpay-sdk/              # KPAY share interaction SDK
└── README.md
```

---

## 7. Product Roadmap

**Phase 1 — Foundation (Months 1–3)**

- Deploy PayrollVault and StreamingPayment contracts on Stellar Testnet
- Build employer dashboard: create workers, set streaming rates, fund vault
- Build worker portal: real-time earnings ticker, withdrawal flow
- Freighter wallet integration and wallet-native auth
- Basic AI Agent: vault monitoring and auto-replenishment

**Phase 2 — Shareholder Protocol (Months 4–6)**

- Deploy ShareholderProtocol contract on Testnet, then Mainnet
- Build shareholder hub: share purchase, vesting dashboard, dividend tracker
- Implement monthly dividend distribution automation
- Governance: on-chain voting for protocol parameters
- Organisation share distribution flow (employer-to-employee KPAY grants)

**Phase 3 — AI Intelligence (Months 7–9)**

- Advanced AI Agent: FX hedging, anomaly detection, payroll forecasting
- Compliance Engine: multi-jurisdiction withholding, automated tax reporting
- Mobile app: React Native worker and shareholder portal
- Institutional tier: custom pricing, dedicated agent configuration

**Phase 4 — Scale (Months 10–12)**

- Multi-chain support: USDC on additional networks with Stellar bridge
- API & SDK for third-party HR system integrations (Rippling, Deel, Gusto connectors)
- White-label offering: companies can run Kampay under their own brand
- DAO transition: full on-chain governance, team vesting completes

---

## 8. Business Model

### 8.1 Revenue Streams

| Revenue Source           | Rate                   | Notes                                            |
| ------------------------ | ---------------------- | ------------------------------------------------ |
| Protocol fee (streaming) | 0.25% of payment value | Primary revenue — every payroll transaction      |
| Protocol fee (batch)     | 0.15% of payment value | Bulk / scheduled payroll runs                    |
| Share issuance spread    | 2% on KPAY purchases   | One-time at share purchase                       |
| Enterprise plan          | $499–$2,999/month      | Custom AI Agent config, dedicated support, SLA   |
| Compliance reports       | $29/report or bundled  | Tax reports, audit logs for regulated industries |
| DEX routing fee          | 0.05% on FX swaps      | Agent-executed currency conversions              |

### 8.2 Fee Distribution

| Allocation            | %   | Purpose                                        |
| --------------------- | --- | ---------------------------------------------- |
| Shareholder dividends | 70% | Monthly distributions to KPAY holders          |
| Protocol operations   | 20% | Infrastructure, agent compute, security audits |
| Development reserve   | 10% | New features, grants, ecosystem growth         |

### 8.3 Unit Economics (Illustrative)

At **$10M monthly payroll volume**:

|                           |                |
| ------------------------- | -------------- |
| Protocol fees generated   | ~$25,000/month |
| Shareholder dividend pool | ~$17,500/month |
| Operations budget         | ~$5,000/month  |
| Development reserve       | ~$2,500/month  |

At **$100M monthly volume** (achievable at 500+ active employers), shareholder dividends reach ~$175,000/month — distributed automatically, on-chain, to every KPAY holder proportionally.

---

## 9. Security & Risk

### 9.1 Smart Contract Security

- All contracts professionally audited before Mainnet deployment
- Formal verification on PayrollVault and ShareholderProtocol contracts
- Multi-sig required for contract upgrades (3-of-5 core team keys)
- 48-hour governance time-lock on all approved proposals
- Bug bounty program live from Day 1 of Testnet

### 9.2 AI Agent Safety

- Hard-coded spending limits per action (no single tx > $50K without human approval)
- All agent actions logged on-chain and in audit database
- Employer can pause or revoke agent permissions at any time
- Every action generates a human-readable justification log

### 9.3 Risk Matrix

| Risk                      | Likelihood | Mitigation                                                              |
| ------------------------- | ---------- | ----------------------------------------------------------------------- |
| Smart contract exploit    | Low        | Audits, formal verification, bug bounty, upgrade path                   |
| Stellar network downtime  | Very Low   | 99.99%+ uptime; queue mechanism for outages                             |
| AI Agent bad decision     | Medium     | Spending limits, human override, explainability logs                    |
| Regulatory action on KPAY | Medium     | Legal counsel, jurisdiction-specific compliance, token structure review |
| USDC depeg                | Very Low   | Multi-asset support; fallback to XLM or other stablecoins               |

---

## 10. Why Stellar

| Property              | Stellar                  | Ethereum             | Solana               |
| --------------------- | ------------------------ | -------------------- | -------------------- |
| Finality              | 3–5 seconds              | 12–15 seconds        | ~0.4s (but forks)    |
| Tx cost               | ~$0.000001               | ~$0.50–$50           | ~$0.0005             |
| Native USDC           | Yes (Circle first-class) | Yes                  | Yes                  |
| Smart contracts       | Soroban (Rust/WASM)      | EVM (Solidity)       | BPF (Rust/C)         |
| Built-in DEX          | Yes (SDEX + AMM)         | No (external only)   | No (external only)   |
| Designed for payments | Yes — core mission       | No — general compute | No — general compute |
| Financial inclusion   | Explicit mission         | Implicit / secondary | Secondary            |

Stellar's architecture was designed from scratch for exactly the use case Kampay is building. That alignment means fewer workarounds, lower costs, and a network whose long-term incentives match Kampay's.

---

## 11. Contributing

Kampay is open source. The protocol — smart contracts, streaming engine, AI agent tools — belongs to the community. Shareholders govern it. Contributors build it.

**How to Contribute**

1. Open an issue to discuss the change before submitting a PR
2. Fork the repo and create a feature branch
3. Follow the code style guides in each sub-package (Rust, TypeScript, Python)
4. Submit a PR with a clear description and test coverage
5. Core team reviews within 72 hours

**Contributor Rewards**

Significant open-source contributors are eligible for KPAY grant allocations, issued quarterly at the discretion of the governance DAO — directly aligning contributors with the protocol's long-term success.

---

_Kampay Protocol | Product Spec v1.0 | Confidential — For Internal & Investor Use_
