# Tinda Escrow

One-line: Deadline-protected USDC escrow for cross-border ASEAN SME trade — pay on delivery, refund on no-show.

## Problem & Solution
**Problem:** A Jakarta batik-shop owner must wire 50% upfront to a Ho Chi Minh supplier and eats the loss on short shipments, with no affordable neutral middleman.
**Solution:** The buyer escrows USDC against the order and releases it only after delivery, or reclaims it after a deadline — settled cross-border in seconds with no correspondent banks.

## Timeline (bootcamp)
- Day 1: Contract + tests (this repo)
- Day 2: Buyer/supplier web app + invoice QR
- Day 3: Testnet demo of both confirm and refund flows

## Stellar Features Used
- USDC transfers via Stellar Asset Contract (`TokenClient`)
- Soroban smart contract escrow with status state machine
- Ledger timestamp for deadline-based refunds
- Trustlines

## Vision & Purpose
Give small intra-ASEAN importers the trust guarantees of a letter of credit at the cost of a Stellar transaction.

## Prerequisites
- Rust (stable) + `wasm32-unknown-unknown` target
- Stellar CLI 22+

## Build
```bash
stellar contract build
```

## Test
```bash
cargo test
```

## Deploy to testnet
```bash
stellar keys generate --global alice --network testnet --fund
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/tinda_escrow.wasm \
  --source alice --network testnet \
  -- --admin alice --token <USDC_SAC_ADDRESS>
```

## Sample CLI invocation (MVP)
```bash
stellar contract invoke --id <CONTRACT_ID> --source buyer --network testnet -- \
  fund_invoice --buyer buyer --id 1 --supplier <SUPPLIER_ADDR> \
  --amount 600 --deadline 1750000000
stellar contract invoke --id <CONTRACT_ID> --source buyer --network testnet -- \
  confirm_delivery --id 1
```

## License
MIT
