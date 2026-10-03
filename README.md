# CertiPulse — Decentralized Credential Protocol

[![Solana Devnet](https://img.shields.io/badge/Solana-devnet-blueviolet)](https://explorer.solana.com/address/C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn?cluster=devnet)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Colosseum 2026](https://img.shields.io/badge/Colosseum-2026-blue)](https://arena.colosseum.org)

> Decentralized, tamper-proof credential verification protocol on Solana — eliminates fake diplomas and reduces background check turnaround from days to ~400 ms.

[Live Demo](https://certipulse-tau.vercel.app) · [Video Walkthrough](https://youtu.be/u8RML9GnAT0?si=02ci2l44LteOhmRJ)) · [Pitch Deck](https://docs.google.com/presentation/d/1Yghhdibiqeyj3jHKecoqjW3u0ANYlD1Bd0hO7n2LiF4/edit?usp=sharing) · [Colosseum Submission](https://colosseum.com/arena/projects/15119/submission)

---

## Submission to 2026 Solana National Hackathon

| Name | Role | Contact |
| :--- | :--- | :--- |
| **Akbari Ferdaus** | Founder & Lead Engineer | [GitHub](https://github.com/Minz-rust) · [Colosseum](https://colosseum.com/arena/projects/15119/submission) |

---

## Problem and Solution

### 1. Widespread Credential Fraud
* **Problem:** Over 85% of applicants embellish qualifications or forge digital PDF credentials.
* **CertiPulse:** Issues immutable, cryptographically signed certificate hashes directly into Solana PDAs.

### 2. Slow and Costly Background Verification
* **Problem:** Traditional manual verification takes 3–7 business days and costs $20–$100+ per check.
* **CertiPulse:** Verification lookups query Solana Devnet RPC directly, achieving finality in ~400 ms at zero cost to employers.

### 3. High Web3 Barrier for Recruiters
* **Problem:** Existing decentralized identity tools require recruiters to install crypto wallets and pay gas fees.
* **CertiPulse:** Frictionless client architecture enables verification with a single click or document hash match without wallets.

---

## Why Solana

* **Speed** — 400 ms block times enable instant certificate resolution during interviews without latency.
* **Cost** — Rent-exempt PDA storage and minimal network fees make issuing thousands of student credentials feasible.
* **Composability** — Solana Program Derived Addresses (PDAs) deterministically map Certificate IDs to on-chain state without centralized indexing databases.
* **Immutability** — Devnet and Mainnet validator consensus ensures educational records cannot be retroactively modified.

---

## Summary of Features

* Direct Anchor Smart Contract issuance on Solana Devnet
* Deterministic PDA account generation based on unique Certificate IDs
* Client-side SHA-256 cryptographic document hashing
* Zero-wallet verification flow for enterprise and HR portals
* On-chain metadata validation with timestamped issuance authority

---

## Tech Stack

| Layer | Technology |
| :--- | :--- |
| **On-chain programs** | Rust · Anchor Framework |
| **SDK / Client** | JavaScript · @solana/web3.js |
| **Frontend** | HTML5 · TailwindCSS · Vercel Edge |
| **Testing & Deployment** | Anchor CLI · Solana Devnet |

---

## Architecture

+--------------------+       +-----------------------------+       +-------------------+
| Issuing Authority  | ----> |      CertiPulse Relay       | ----> | Solana Devnet     |
| (University / Web) |       | (SHA-256 Hash Computation)  |       | (Anchor Program)  |
+--------------------+       +-----------------------------+       +-------------------+
|                                |
v                                v
+-------------------------+         +-------------------+
| Deterministic PDA Seed  | ------> | Validated Account |
| [b"certificate", id]    |         | (Immutable State) |
+-------------------------+         +-------------------+

---

## Quick Start

**Prerequisites:** Node.js 18+, Rust, Solana CLI, Anchor CLI

```bash
# Clone the repository
git clone https://github.com/Minz-rust/certipulse.git
cd certipulse

# Build Solana programs
anchor build

# Run tests
anchor test

# Start frontend locally
npx serve .
```

---

## Roadmap

- [x] Solana Anchor smart contract core design
- [x] Devnet deployment (C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn)
- [x] Client frontend and Web3 RPC integration
- [x] Zero-friction verification terminal on Vercel
- [ ] Multi-tenant university admin dashboard
- [ ] Solana Mainnet migration

---

## Resources
Live Application: https://certipulse-tau.vercel.app/

Colosseum Submission: https://colosseum.com/arena/projects/15119/submission

Project Presentation: https://docs.google.com/presentation/d/1Yghhdibiqeyj3jHKecoqjW3u0ANYlD1Bd0hO7n2LiF4/edit?slide=id.p1#slide=id.p1

Solana Devnet Explorer: https://explorer.solana.com/address/C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn?cluster=devnet

* [Video Demo](https://youtu.be/u8RML9GnAT0?si=voje9t_4Z2TTtQSp)

## License

MIT — see [LICENSE](LICENSE)
