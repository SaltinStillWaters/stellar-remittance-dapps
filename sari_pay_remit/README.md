# SariPay Remit

One-line: Code-redeemable USDC escrow that turns any sari-sari store into an instant remittance cash-out point.

## Problem & Solution
**Problem:** A Filipina worker in Singapore pays ₱180–₱350 and waits a day to send money, and her mother in rural Cebu travels two jeepney rides to a remittance branch to collect it.
**Solution:** The sender escrows USDC under a one-time hashed code; the recipient redeems it at a neighborhood store, which releases the USDC on-chain in ~5s for a sub-cent fee.

## Timeline (bootcamp)
- Day 1: Contract + tests (this repo)
- Day 2: Freighter/Lobstr cash-out UI + SMS code delivery
- Day 3: Testnet deploy, live send→claim demo

## Stellar Features Used
- USDC transfers via Stellar Asset Contract (`TokenClient`)
- Soroban smart contract escrow
- Trustlines (store & recipient hold USDC)
- `sha256` code commitment

## Vision & Purpose
Replace high-fee remittance branches with the corner shops people already trust, keeping the cash-out margin inside the local economy.

## Prerequisites
- Rust (stable) + `wasm32-unknown-unknown` target
- Stellar CLI 22+ (`cargo install stellar-cli --locked`)

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
  --wasm target/wasm32-unknown-unknown/release/sari_pay_remit.wasm \
  --source alice --network testnet \
  -- --admin alice --token <USDC_SAC_ADDRESS>
```

## Sample CLI invocation (MVP)
```bash
# sender escrows 500 USDC (units) under a code hash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  create_remittance --sender alice --id 1 --store <STORE_ADDR> \
  --amount 500 --code_hash <SHA256_HEX_32B>

# store redeems with the secret code
stellar contract invoke --id <CONTRACT_ID> --source store --network testnet -- \
  claim --id 1 --code <CODE_BYTES_HEX>
```

## License
MIT
