# Paluwagan Pool

One-line: A trustless on-chain rotating savings circle (paluwagan/arisan) that enforces fair, paid-in-full payouts in USDC.

## Problem & Solution
**Problem:** Indonesian workers in Kuala Lumpur run an envelope-based rotating savings circle, and members skipping their turn's contribution shortchange the month's recipient with no recourse.
**Solution:** Members lock USDC per round; the contract only pays the rotation's recipient once every member has funded, making the circle tamper-proof.

## Timeline (bootcamp)
- Day 1: Contract + tests (this repo)
- Day 2: Mobile-first join/contribute UI
- Day 3: Testnet demo of a full 3-member round

## Stellar Features Used
- USDC transfers via Stellar Asset Contract (`TokenClient`)
- Soroban smart contract with deterministic round-robin payouts
- Trustlines

## Vision & Purpose
Bring the fairness guarantees of code to a savings ritual millions across SEA already practice, without changing how they think about it.

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
  --wasm target/wasm32-unknown-unknown/release/paluwagan_pool.wasm \
  --source alice --network testnet \
  -- --admin alice --token <USDC_SAC_ADDRESS> --contribution 100
```

## Sample CLI invocation (MVP)
```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  join --member alice
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  contribute --member alice --round 0
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  payout --round 0
```

## License
MIT
