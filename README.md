# Alloy-based Ethereum Balance Fetcher for zkLight Client

`Enabling large-scale lending on Solana using Ethereum assets (ETH or stablecoins) as collateral — without bridging the actual tokens — by proving Ethereum balances trustlessly via zkSNARKs.`

## 🔍 What Does the zkLight Client Actually Prove?
- You're proving that some account balance (ETH or stablecoin) existed on Ethereum at a specific block height, by:
    - Fetching Ethereum data (e.g., from a full node or archive node).
    - Proving this state via a zkSNARK proof (using libraries like Halo2 or Plonky2).
    - Verifying this proof trustlessly on Solana.

## 🎯 Problem Today:
- To borrow or use Ethereum ETH or stablecoins as collateral on Solana (or other chains), users currently must:
    - Rely on trusted bridges to transfer assets cross-chain.
    - Trust centralized or semi-trusted oracles or relayers to attest Ethereum balances.
    - Risk funds due to bridge hacks or oracle failures.

## ✅ My zk Solution:
- Allow the user to prove their Ethereum ETH or stablecoin balance at a given block using zkSNARKs → verified on Solana → the dApp accepts this proof and enables borrowing, trading, or other DeFi use cases.

## 🔐 Trustless Setup:
  - No actual asset bridging is needed.
  - No trusted relayer or oracle required.
  - Only a zero-knowledge proof of balance is submitted and verified on-chain on Solana.
  - The proof cryptographically guarantees the Ethereum balance at block X.

# Flow: zkETH/Stablecoin Balance Proof for DeFi Lending
1. User connects Ethereum wallet to the dApp.

2. User selects the asset type to prove (e.g., ETH, USDC, DAI).

3. User provides their Ethereum address and optionally the block number at which they want to prove the balance. If no block is specified, the latest finalized block is used.

4. Your Rust zk prover service:
    - Fetches the Ethereum block header for the selected block number.

    - Calls eth_getProof RPC to obtain the Merkle proof of the user's account balance (for ETH) or ERC20 balanceOf slot (for stablecoins).

    - Generates a zkSNARK proof verifying that the balance is at least the claimed amount at the given Ethereum state root.

5. The zkSNARK proof and public inputs (block hash, address, asset type, claimed balance) are submitted to the Solana on-chain verifier contract.

6. Solana verifies the proof, and upon success, the DeFi protocol unlocks collateralized borrowing, trading, or other functionality for the user.
