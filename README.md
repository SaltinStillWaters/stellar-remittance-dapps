# Stellar Remittance dApps

Three demo-ready Soroban smart contracts tackling **cross-border remittance and coordination** for unbanked workers and SMEs across **Southeast Asia**. Every contract moves **real USDC** through the Stellar Asset Contract (SAC) interface, so each MVP is a genuine money-movement demo, not a mock.

Built for a hackathon / bootcamp timeframe — each contract is self-contained, compile-ready, and ships with exactly 5 unit tests (happy path, edge cases, and state verification).

## Contracts

| Folder | Name | One-line | Stellar features |
|--------|------|----------|------------------|
| [`sari_pay_remit/`](./sari_pay_remit) | **SariPay Remit** | Code-redeemable USDC escrow turning any sari-sari store into an instant remittance cash-out point | USDC (SAC), Soroban, trustlines, `sha256` commitment |
| [`paluwagan_pool/`](./paluwagan_pool) | **Paluwagan Pool** | Trustless on-chain rotating savings circle (paluwagan/arisan) with fair paid-in-full payouts | USDC (SAC), Soroban round-robin, trustlines |
| [`tinda_escrow/`](./tinda_escrow) | **Tinda Escrow** | Deadline-protected USDC escrow for cross-border ASEAN SME trade — pay on delivery, refund on no-show | USDC (SAC), Soroban, ledger timestamps, trustlines |

## Quick start (any contract)

```bash
rustup target add wasm32-unknown-unknown   # once

cd sari_pay_remit        # or paluwagan_pool / tinda_escrow
cargo test               # runs the 5 unit tests
stellar contract build   # produces the optimized wasm
```

## Prerequisites
- Rust (stable) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli) 22+ — `cargo install stellar-cli --locked`

> All three pin `soroban-sdk = "22.0.0"`. If your toolchain ships a newer SDK, bump both `soroban-sdk` lines in each `Cargo.toml` to match — the APIs used are stable through 25.x.

## Region & users
- **Region:** Southeast Asia (Philippines, Indonesia, Vietnam, Malaysia)
- **Users:** Unbanked migrant workers, sari-sari store owners, and small importers/exporters
- **Theme:** Remittance / cross-border payments

## License
[MIT](./LICENSE)
