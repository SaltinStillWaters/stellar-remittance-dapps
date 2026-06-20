#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Bytes, BytesN, Env,
};

// Register a test USDC asset and return its contract address.
fn make_usdc(env: &Env) -> Address {
    let issuer = Address::generate(env);
    env.register_stellar_asset_contract_v2(issuer).address()
}

fn setup(env: &Env) -> (SariPayRemitClient<'static>, Address, Address, Address, BytesN<32>, Bytes) {
    let admin = Address::generate(env);
    let token = make_usdc(env);
    let sender = Address::generate(env);
    let store = Address::generate(env);
    StellarAssetClient::new(env, &token).mint(&sender, &1_000);

    let id = env.register(SariPayRemit, (admin, token.clone()));
    let client = SariPayRemitClient::new(env, &id);

    let code = Bytes::from_slice(env, b"otp-CEBU-4821");
    let code_hash: BytesN<32> = env.crypto().sha256(&code).into();
    (client, token, sender, store, code_hash, code)
}

// 1. Happy path: send then cash out end-to-end.
#[test]
fn test_create_and_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token, sender, store, code_hash, code) = setup(&env);

    client.create_remittance(&sender, &1u64, &store, &500i128, &code_hash);
    client.claim(&1u64, &code);

    let usdc = TokenClient::new(&env, &token);
    assert_eq!(usdc.balance(&store), 500);  // store received the cash-out
    assert_eq!(usdc.balance(&sender), 500); // 1000 minted - 500 sent
}

// 2. Edge: wrong code is rejected.
#[test]
#[should_panic]
fn test_claim_wrong_code_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, sender, store, code_hash, _code) = setup(&env);

    client.create_remittance(&sender, &1u64, &store, &500i128, &code_hash);
    let wrong = Bytes::from_slice(&env, b"not-the-code");
    client.claim(&1u64, &wrong); // InvalidCode -> panic
}

// 3. State verification: record + escrow reflect a completed claim.
#[test]
fn test_state_after_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token, sender, store, code_hash, code) = setup(&env);

    client.create_remittance(&sender, &1u64, &store, &500i128, &code_hash);
    client.claim(&1u64, &code);

    let remit = client.get_remittance(&1u64).unwrap();
    assert!(remit.claimed);
    let usdc = TokenClient::new(&env, &token);
    assert_eq!(usdc.balance(&client.address), 0); // escrow fully drained
}

// 4. Edge: a remittance can only be claimed once.
#[test]
#[should_panic]
fn test_double_claim_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, sender, store, code_hash, code) = setup(&env);

    client.create_remittance(&sender, &1u64, &store, &500i128, &code_hash);
    client.claim(&1u64, &code);
    client.claim(&1u64, &code); // AlreadyClaimed -> panic
}

// 5. Edge: duplicate remittance id is refused.
#[test]
#[should_panic]
fn test_duplicate_id_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, sender, store, code_hash, _code) = setup(&env);

    client.create_remittance(&sender, &1u64, &store, &100i128, &code_hash);
    client.create_remittance(&sender, &1u64, &store, &100i128, &code_hash); // AlreadyExists -> panic
}
