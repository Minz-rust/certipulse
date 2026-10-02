# CertiPulse • On-Chain Credential Verification

> Decentralized, tamper-proof academic and professional credential registry built on Solana.

---

## 📌 Problem Overview
Conventional paper and PDF certificates are vulnerable to digital alterations. Employers, universities, and certification bodies lack an instant, trustless mechanism to verify authenticity without slow manual inquiries.

## 💡 Solution
CertiPulse registers cryptographic document fingerprints (SHA-256) directly on the Solana ledger:
- **Instant Proof:** Educational institutions anchor certificate metadata in sub-second transactions.
- **Permissionless Verification:** Recruiters and HR can verify authentic records instantly without needing a wallet.
- **Tamper-Proof:** Modifying even a single character in the original document invalidates the on-chain hash match.

---

## ⚡ Tech Stack & Architecture
- **Blockchain Network:** Solana Devnet
- **Program ID:** `C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn`
- **Framework:** Anchor (Rust)
- **Frontend:** HTML5, Tailwind CSS, JavaScript (SHA-256 & Web3)

---

## 🚀 Live Demo Workflow
1. **Issue:** Academy enters student data and clicks `Mint Proof to Solana`.
2. **On-Chain Anchor:** A cryptographic hash is generated and anchored on the ledger.
3. **Verify:** Employer enters the hash to verify authenticity. An altered document triggers an immediate counterfeit alert.

## 📄 License
MIT License. Built for the Solana Hackathon / Colosseum.
