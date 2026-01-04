# DOMINION

## Sovereign Identity Primitive for Autonomi

Dominion is the sovereign identity primitive for Autonomi — enabling eternal, human-readable aliases that are cryptographically owned forever, resolved peer-to-peer via its native DHT and four-word networking, protected by pure post-quantum-secure architecture and bootstrapped by a permissionless social trust layer. 

No central authority. No expiration. No censorship.

yourname.p2p → owned forever, resolved everywhere.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with egui](https://img.shields.io/badge/built%20with-egui-blue)](https://github.com/emilk/egui)
[![Saorsa PQC](https://img.shields.io/badge/PQC-ML--DSA--65-green)](https://github.com/saorsa-labs)

## Completed Features (v0.3 Prototype)

### Naming & Resolution
- **Native Cross-Platform GUI**  
Responsive egui interface for all commands. Serves as the primary frontend for identity management and future App Marketplace integration.

- **DHT-Based Design**  
Uses Autonomi's native DHT (Saorsa Components) for spam-free, decentralized storage and updates. Enables eternal identity mapping, supporting Storage & Retrieval and expanding Advanced Agents with self-organizing identities.

- **Quantum-Secure Keypair Generation**  
Generates pure ML-DSA-65 keypairs for ownership proofs. Leverages Quantum Threshold Cryptography for post-quantum security and integrates with Saorsa Components for PQC-protected identity.

- **Wallet Connectivity**  
Connects Arbitrum-compatible wallets for signed operations. Supports the Autonomi Network Token economy with planned Fiat Gateway and Paymaster integration — enabling payment-linked domains while requiring only AUTONOMI token for reduced friction.

- **Domain Registration with Anti-Squatting**  
Registers human-readable domains with wallet-based staking. Includes premium auctions for high-value names, tying into Emissions Service Refinements for long-term rewards.

- **Simulated AUTONOMI Staking**  
  Enforces staking simulation for registration, supporting Data Payment Gas Enhancements and Agentic Payment System for low-friction identity creation.

- **Multi-TLD Support (.p2p Default)**  
Flagship .p2p TLD with full validation for crypto alternates like .ant, .btc, .eth, .sol, and planned support for others. Leverages Autonomi's ant-QUIC for quantum-proof addressing and supports Location-Aware Networking via optional geo-records.

- **Domain Resolution & Lookup**  
  Resolves domains to four-word addresses with full details (owners, records, auctions). Supports Automatic Node Upgrades via notification feeds and Relay Removal for efficient topology.

### Ownership & Security
- **Sovereign Multi-Party Ownership**  
  Full multi-party ownership with `add-owner`, `remove-owner`, `transfer`, and `rotate-key`. Expands Quantum Threshold Cryptography for DAO-controlled domains and supports "list:id" in X402 payments.

- **Signature Threshold Verification**  
  Configurable signature threshold per domain (`set-threshold`). Prepares for real multi-sig in future PQC implementations.

### Social Trust Layer (Web of Trust)
- **PQC-Signed Endorsements**  
PQC-signed endorsements for reputation building. Owners can endorse other domains with cryptographically secure messages. Supports Advanced Agents with trust in self-organizing systems and App Marketplace for verified developer profiles.

- **Reputation Foundation**  
Enables trust-based filtering for the upcoming App Marketplace and AI agent ecosystems.

- **Fully Sovereign**  
No central authority; endorsements are stored in the DHT and verifiable by anyone.

- **Future Extensions**  
Trust graph visualization, weighted scores, and transitive trust for advanced agent networking.

### App Marketplace Integration
- **App Publishing**  
  `publish-app` command stores immutable manifest hashes and descriptions. Enables tamper-proof app distribution in the App Marketplace and supports verified listings.

- **AI-Agent Identity Hooks**  
  `agent-register` creates identities for AI agents with endpoints. Supports OpenAI Spec and AI-Native Networking — Dominion becomes the registry for autonomous agents in agent-driven marketplaces.

- **Agentic Payment System**  
  `agent-pay` and `pay-domain` simulate autonomous and direct payments. Expands Agentic Payment System and Paymaster for frictionless, domain-linked transactions.

### Advanced & Ecosystem Features
- **Geo-Identity Mapping**  
  Stores `GEO` records for location data. Supports Location-Aware Networking for region-specific apps and local storage networks in the Marketplace.

- **Markdown Domain Pages**  
  `add-record MD` enables simple web pages on domains. Directly supports The Markdown Web for developer docs and app landing pages.

- **Node Upgrade Notifications**  
  `notify-upgrade` sends alerts via endorsements. Supports Automatic Node Upgrades with domain-based network feeds.

- **Premium Domain Auctions**  
  Decentralized 7-day auctions for high-value domains. Monetizes identities using Autonomi Network Token.

### Utilities & UX
- **AI-Driven Name Suggestion**  
  Local thematic name generator. Enhances AI-Native Networking with intelligent identity creation.

- **Eternal Links**  
  Permanent content links (`https://domain.p2p/content/hash`). Supports Streaming APIs and Dave Phase 1/2 for immutable file access.

- **Enhanced PQC Verification Badge**  
  Visual quantum-security confirmation in lookup with multi-party owner display.

- **Key Backup & Restore**  
  Simulated secure vault (ready for real implementation). Supports Mobile SDKs.

### Ubiquity & Resolver Path (In Development)
- **Browser Extension Resolver**  
One-click Chrome/Firefox extension for seamless .p2p resolution with legacy DNS fallback.

- **Local DNS Proxy**  
System-wide resolution for apps and tools.

- **Standalone Resolver App**  
lightweight gateway that bundles Autonomi client setup — the frictionless on-ramp to the network.

## Vision

Dominion is the bridge to a sovereign digital future — where identities are cryptographically derived, eternally owned, and resolved peer-to-peer on Autonomi. This is not DNS on blockchain. This is native P2P identity for a native P2P network.

`.p2p` is the future.  
Your dominion awaits.

## Quick Start

```bash
git clone https://github.com/grebnief/dominion.git
cd dominion
cargo run
```
Once the program starts, type this command and press Enter:

```bash
register dominion.p2p blue.river.mountain.star
```
You now own dominion.p2p in the simulated DHT!

With this:
```markdown
## Claim Your Sovereign Identity
```
```bash
git clone https://github.com/grebnief/dominion.git
cd dominion
cargo run
```

## Acknowledgements

- **Built with Grok (xAI)** — powering reasoning, code generation, and rapid prototyping.
- **Saorsa Labs** — led by David Irvine, for the foundational post-quantum cryptography library **saorsa-pqc** (pure Rust implementation of NIST-standardized ML-DSA, including ML-DSA-65) and **saorsa-core** (DHT-based P2P platform with four-word addressing and quantum-resistant primitives). ([github.com/saorsa-labs](https://github.com/saorsa-labs))
- **Autonomi Community** — past, present, and future contributors, for roadmap inspiration, ant-QUIC transport, and the vision of a fully decentralized, quantum-resistant network. ([autonomi.com](https://autonomi.com))
- **NIST Post-Quantum Cryptography Project** — for standardizing ML-DSA and other algorithms enabling quantum-resistant signatures. ([csrc.nist.gov/projects/post-quantum-cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography))
- **egui** — for the immediate-mode GUI framework powering the native cross-platform interface. ([github.com/emilk/egui](https://github.com/emilk/egui))
- **ethers-rs** — for Ethereum wallet simulation and signing. ([github.com/gakonst/ethers-rs](https://github.com/gakonst/ethers-rs))
- **Open Quantum Safe (OQS)** — for pioneering PQC research and reference implementations that influenced the Saorsa ecosystem. ([open-quantum-safe.org](https://open-quantum-safe.org))
- **Developed for Autonomi** — the world's first fully decentralized, quantum-resistant network.
  

