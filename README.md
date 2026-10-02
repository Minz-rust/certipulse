# CertiPulse

> Decentralized, tamper-proof credential verification protocol on Solana. Instant verification with zero Web3 friction for employers.

[![Solana Devnet](https://img.shields.io/badge/Solana-Devnet-14F195?style=flat-square&logo=solana)](https://explorer.solana.com/address/C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn?cluster=devnet)
[![Live MVP](https://img.shields.io/badge/Live_MVP-Vercel-black?style=flat-square&logo=vercel)](https://certipulse-tau.vercel.app)
[![Colosseum Hackathon](https://img.shields.io/badge/Colosseum-Hackathon_2026-6C5CE7?style=flat-square)](https://arena.colosseum.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)

## 📌 Links & Resources

* **Live MVP Demo:** [https://certipulse-tau.vercel.app](https://certipulse-tau.vercel.app)
* **Pitch Deck:** [https://docs.google.com/presentation/d/1Yghhdibiqeyj3jHKecoqjW3u0ANYlD1Bd0hO7n2LiF4/edit?usp=sharing](https://docs.google.com/presentation/d/1Yghhdibiqeyj3jHKecoqjW3u0ANYlD1Bd0hO7n2LiF4/edit?usp=sharing)
* **Program ID (Devnet):** `C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn`

## 🎯 Overview

Credential fraud is a massive global issue: up to 85% of applicants embellish resumes, while manual verification takes 3–7 days and costs $20–$100+ per check.

CertiPulse solves this by anchoring cryptographic certificate hashes directly onto the Solana blockchain via Program Derived Addresses (PDAs):

* **Tamper-Proof:** Certificates cannot be forged or altered once registered.
* **Zero Web3 Friction:** Employers and HR managers can instantly verify authenticity without crypto wallets or transaction fees.
* **Sub-Second Finality:** Verification resolves in ~400ms using Solana high-performance RPCs.

## 🏗 Tech Stack & Architecture

* **Smart Contract:** Rust & Anchor Framework (Solana Program)
* **Client Frontend:** Vanilla JavaScript, HTML5, Tailwind CSS
* **Web3 Integration:** @solana/web3.js & RPC Devnet endpoint
* **Hashing:** Client-side SHA-256 cryptographic verification
* **Deployment:** Vercel edge deployment

### How It Works

1. **Issuer (Academy):** Calculates Document Hash (SHA-256).
2. **Anchor Program:** Stores hash & metadata inside Solana PDA account.
3. **Verifier (HR):** Uploads PDF or enters ID to check hash against Solana Devnet.

## 🚀 Getting Started & Local Setup

**Clone the repository:**
`git clone https://github.com/Minz-rust/certipulse.git`

**Run local server:**
Open `index.html` directly in your browser or run:
`npx serve .`

## 📜 On-Chain Program Details

* **Network:** Solana Devnet
* **Program ID:** `C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn`
* **Solana Explorer:** [https://explorer.solana.com/address/C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn?cluster=devnet](https://explorer.solana.com/address/C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn?cluster=devnet)

## 👥 Team

* **Akbari Ferdaus** — Solo Founder & Developer (Rust / Solana / Web3)
