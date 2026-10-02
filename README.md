# ⚡ CertiPulse • On-Chain Credential Verification

> Decentralized, tamper-proof academic and professional credential registry built on Solana.  
> **Submitted for the Solana Workshop / Hackathon ($50K Prize Track).**

---

### 🎥 Project Presentation & Demo
* 📺 **[Watch Product Demo Video (28s)]**(https://youtu.be/u8RML9GnAT0?si=0ApRB-wd_0KQirO0) — Live interface walkthrough & fraud detection test.
* 🎙️ **[Watch Pitch Deck Video (1m)]**(https://youtu.be/yJjmL74qo7c?si=pIY45Az0VBaV65xs) — Problem overview, architecture & market vision.

---

## 📌 Problem Overview
Conventional digital certificates and PDF diplomas are trivially forged using basic photo editors. Employers, universities, and recruitment teams waste weeks on manual email inquiries or risk hiring candidates with fraudulent credentials.

## 💡 Solution
CertiPulse registers cryptographic document fingerprints (SHA-256) directly into Solana Program Derived Addresses (PDAs):
- **Instant Proof:** Educational institutions anchor credentials in sub-second transactions.
- **Permissionless Verification:** Recruiters and HR verify credentials instantly without needing a Web3 wallet or browser extension.
- **Tamper-Proof:** Altering even a single character in the document generates an immediate counterfeit alert.

---

## ⚡ Tech Stack & On-Chain Deployment
- **Network:** Solana Devnet
- **Program ID:** `C4q98LErU614JFw48JEHsHwz3TVrJ2jBcx4CJeUxkyzn`
- **Framework:** Anchor (Rust)
- **Frontend:** HTML5, Tailwind CSS, JavaScript (SHA-256 Cryptography & Web3 RPC)

---

## 🚀 Live Workflow
1. **Issue:** Educational academy enters recipient credentials and clicks `Mint Proof to Solana`.
2. **On-Chain Anchor:** A cryptographic SHA-256 hash is anchored onto the ledger under our Solana program.
3. **Verify:** Employer enters the hash to verify authenticity. An altered document triggers an immediate fraud alert.

---

## 📄 License
MIT License. Built for the Solana Global Hackathon & Workshop.
