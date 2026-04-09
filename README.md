# KAMPAY

Kampay is an AI-first, autonomous payroll protocol built on the Stellar blockchain — and at its core, it's a platform where businesses raise capital by selling shares, and shareholders get paid directly from the revenue those businesses generate.

Any organisation or business owner can list their shares on Kampay. Investors and community members buy in, hold shares in their Stellar wallet, and track the business's on-chain activity in real time — payroll processed, revenue earned, dividends distributed. When the business runs payroll through Kampay, a portion of that activity flows back to shareholders automatically, every month, on-chain. No intermediaries. No manual reporting. The numbers are public, immutable, and settled in seconds.

The global payroll industry processes over **$8 trillion annually**, yet the people and businesses powering that volume see none of the upside. They pay 3–8% in processing fees, wait 3–5 business days for settlements, and hand their money to intermediaries who pocket the margin.

Kampay flips that model entirely. Businesses run payroll as Soroban smart contracts on Stellar — settling in 3–5 seconds for fractions of a cent. An AI Treasury Agent handles scheduling, FX hedging, and cash flow autonomously. And the fee revenue that would have gone to a processor flows instead to shareholders: the investors, workers, and community members who bought into the businesses they believe in.

Stellar makes this possible. With native USDC support, a built-in DEX, 3–5 second finality, and transaction costs of ~$0.000001, Stellar is the only public blockchain built from the ground up for payments at global scale. Every share sale, every payroll run, every dividend distribution — settled natively on Stellar.

---

## Features

- **Streaming payroll** — workers paid per second via Soroban smart contracts, withdrawable at any time
- **Business share listings** — organisations list shares on-chain; investors buy in and track activity live
- **AI Treasury Agent** — autonomous vault monitoring, FX hedging, auto-replenishment, anomaly detection
- **On-chain compliance** — jurisdiction withholding, KYC/AML, and audit logs enforced at the contract layer
- **KPAY dividends** — 70% of protocol fee revenue distributed monthly to shareholders, automatically

---

## KPAY Shareholder Model

Every payroll transaction generates a 0.25% protocol fee. That revenue flows to a Protocol Treasury and is distributed monthly to KPAY shareholders — proportional to their holdings, straight to their Stellar wallet.

| Tier          | KPAY   | Price (USDC) | Dividend Estimate\* | Governance |
| ------------- | ------ | ------------ | ------------------- | ---------- |
| Contributor   | 100    | $50          | ~$3–8/yr            | 1x         |
| Builder       | 500    | $225         | ~$15–40/yr          | 1x         |
| Operator      | 2,000  | $800         | ~$60–160/yr         | 1.5x       |
| Architect     | 10,000 | $3,500       | ~$300–800/yr        | 2x         |
| Institutional | Custom | Custom       | Negotiated          | 3x         |

_Based on protocol volume. Not guaranteed._

Fee split: **70%** shareholders · **20%** operations · **10%** development reserve.

---

## Tech Stack

| Layer           | Technology                          |
| --------------- | ----------------------------------- |
| Blockchain      | Stellar (Mainnet + Testnet)         |
| Smart Contracts | Soroban — Rust/WASM                 |
| Stable Asset    | USDC on Stellar (Circle)            |
| Frontend        | Next.js 14, TypeScript, React 18    |
| Wallet          | Freighter                           |
| AI Agent        | Python, LangChain, FastAPI + Claude |
| Database        | PostgreSQL + Prisma                 |
| Hosting         | Vercel + Railway                    |

---

## Repo Structure

```
kampay/
├── frontend/          # Next.js 14 app (dashboard, worker portal, shareholder hub)
├── contracts/         # Soroban smart contracts (Rust)
│   ├── payroll-vault/
│   ├── streaming-payment/
│   ├── shareholder-protocol/
│   └── compliance-registry/
├── agent/             # AI Treasury Agent (Python)
└── packages/          # Shared libs (stellar-utils, kpay-sdk)
```

