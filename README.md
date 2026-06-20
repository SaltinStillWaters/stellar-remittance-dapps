# Stellar Remittance dApps

## Project description

Three demo-ready Soroban smart contracts tackling **cross-border remittance and coordination** for unbanked workers and SMEs across **Southeast Asia**. Every contract moves **real USDC** through the Stellar Asset Contract (SAC) interface, so each MVP is a genuine money-movement demo — not a mock. Each contract is self-contained, compile-ready, and ships with exactly 5 passing unit tests (happy path, edge cases, state verification).

| Folder | Name | What it does | Stellar features |
|--------|------|--------------|------------------|
| [`sari_pay_remit/`](./sari_pay_remit) | **SariPay Remit** | Code-redeemable USDC escrow that turns any sari-sari store into an instant remittance cash-out point | USDC (SAC), Soroban, trustlines, `sha256` commitment |
| [`paluwagan_pool/`](./paluwagan_pool) | **Paluwagan Pool** | Trustless on-chain rotating savings circle (paluwagan/arisan) with fair, paid-in-full payouts | USDC (SAC), Soroban round-robin, trustlines |
| [`tinda_escrow/`](./tinda_escrow) | **Tinda Escrow** | Deadline-protected USDC escrow for cross-border ASEAN SME trade — pay on delivery, refund on no-show | USDC (SAC), Soroban, ledger timestamps, trustlines |

- **Region:** Southeast Asia (Philippines, Indonesia, Vietnam, Malaysia)
- **Users:** Unbanked migrant workers, sari-sari store owners, small importers/exporters
- **Theme:** Remittance / cross-border payments

## Setup instructions (how to run locally)

**Prerequisites**
- Rust (stable) with the wasm target: `rustup target add wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli) 22+: `cargo install stellar-cli --locked`

**Clone**
```bash
git clone https://github.com/SaltinStillWaters/stellar-remittance-dapps
cd stellar-remittance-dapps
```

**Run the tests** (works for any contract — pick a folder)
```bash
cd tinda_escrow        # or sari_pay_remit / paluwagan_pool
cargo test
```

**Build the WASM**
```bash
stellar contract build
# output: target/wasm32-unknown-unknown/release/<contract>.wasm
```

**Deploy to testnet** (constructors require args after `--`)
```bash
stellar keys generate --global alice --network testnet --fund

# tinda_escrow / sari_pay_remit need --admin and --token
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/tinda_escrow.wasm \
  --source alice --network testnet \
  -- \
  --admin alice \
  --token $(stellar contract id asset --asset native --network testnet)

# paluwagan_pool additionally needs --contribution <amount>
```

**Invoke the MVP** (example: Tinda Escrow)
```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  fund_invoice --buyer alice --id 1 --supplier <SUPPLIER_ADDR> --amount 600 --deadline 1750000000

stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  confirm_delivery --id 1
```

> Each contract's own `README.md` has its full deploy/invoke walkthrough.

## Screenshots

_Add screenshots here once the demo is running (e.g. test output, a successful testnet deploy, or the frontend)._

| | |
|---|---|
| ![Tests passing](docs/screenshots/tests.png) | ![Testnet deploy](docs/screenshots/deploy.png) |

To add them: drop image files in `docs/screenshots/` and commit. Until then the links above will show broken-image placeholders.

## License

[MIT](./LICENSE)
