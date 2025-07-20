# My Solana Token Project

A decentralized "hold to earn" token built on the Solana blockchain using Rust and Anchor.  
Users earn rewards based on token holdings and staking duration with gamified reputation and manual claim mechanisms.

## Features

- Hold-to-earn tokenomics on Solana
- Buy/sell tax with reward, charity, and founder pools
- Manual reward claim via a vault system
- Reputation Points (RP) system for rewarding engagement
- Staking with multiplier benefits
- Quest-based RP earning and perks (NFT mint access, voting, tax discounts)

## Tech Stack

- Rust (for Solana programs)
- Anchor framework (for easier Solana smart contract development)
- Solana CLI and SDK tools
- TypeScript / JavaScript (for frontend & tests)

## Getting Started

### Prerequisites

- Rust (with nightly toolchain)
- Solana CLI
- Anchor CLI
- Node.js (for frontend and tests)
- Yarn or npm

### Installation

```bash
# Clone repo
git clone https://github.com/denisthemenace42/Causa-Project.git
cd Causa-Project

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Build the Solana program
anchor build

# Run tests
anchor test
