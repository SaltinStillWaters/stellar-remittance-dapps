#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{StellarAssetClient, TokenClient},
    Address, Env,
};

fn make_usdc(env: &Env) -> Address {
    let issuer = Address::generate(env);
    env.register_stellar_asset_contract_v2(issuer).address()
}

fn setup(env: &Env) -> (TindaEscrowClient<'static>, Address, Address, Address) {
    let admin = Address::generate(env);
    let token = make_usdc(env);
    let buyer = Address::generate(env);
    let supplier = Address::generate(env);
    StellarAssetClient::new(env, &token).mint(&buyer, &1_000);

    let id = env.register(TindaEscrow, (admin, token.clone()));
    let client = TindaEscrowClient::new(env, &id);
    (client, token, buyer, supplier)
}

// 1. Happy path: fund then confirm pays the supplier.
#[test]
fn test_fund_and_confirm() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token, buyer, supplier) = setup(&env);

    client.fund_invoice(&buyer, &1u64, &supplier, &600i128, &1_000u64);
    client.confirm_delivery(&1u64);

    let usdc = TokenClient::new(&env, &token);
    assert_eq!(usdc.balance(&supplier), 600);
    assert_eq!(usdc.balance(&buyer), 400); // 1000 - 600
}

// 2. Edge: refund before the deadline is rejected.
#[test]
#[should_panic]
fn test_refund_before_deadline_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, buyer, supplier) = setup(&env);

    env.ledger().set_timestamp(500);
    client.fund_invoice(&buyer, &1u64, &supplier, &600i128, &1_000u64);
    client.refund(&1u64); // DeadlineNotReached -> panic
}

// 3. State verification: confirm marks Released and drains the escrow.
#[test]
fn test_state_after_confirm() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token, buyer, supplier) = setup(&env);

    client.fund_invoice(&buyer, &1u64, &supplier, &600i128, &1_000u64);
    client.confirm_delivery(&1u64);

    assert_eq!(client.get_invoice(&1u64).unwrap().status, Status::Released);
    assert_eq!(TokenClient::new(&env, &token).balance(&client.address), 0);
}

// 4. Refund path: after the deadline the buyer gets their USDC back.
#[test]
fn test_refund_after_deadline() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token, buyer, supplier) = setup(&env);

    client.fund_invoice(&buyer, &1u64, &supplier, &600i128, &1_000u64);
    env.ledger().set_timestamp(2_000); // past deadline
    client.refund(&1u64);

    assert_eq!(client.get_invoice(&1u64).unwrap().status, Status::Refunded);
    assert_eq!(TokenClient::new(&env, &token).balance(&buyer), 1_000); // made whole
}

// 5. Edge: an invoice cannot be confirmed twice.
#[test]
#[should_panic]
fn test_double_confirm_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, buyer, supplier) = setup(&env);

    client.fund_invoice(&buyer, &1u64, &supplier, &600i128, &1_000u64);
    client.confirm_delivery(&1u64);
    client.confirm_delivery(&1u64); // AlreadySettled -> panic
}
