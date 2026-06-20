#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env,
};

fn make_usdc(env: &Env) -> Address {
    let issuer = Address::generate(env);
    env.register_stellar_asset_contract_v2(issuer).address()
}

// 3-member circle, 100 USDC contribution, everyone minted 1000.
fn setup(env: &Env) -> (PaluwaganPoolClient<'static>, Address, [Address; 3]) {
    let admin = Address::generate(env);
    let token = make_usdc(env);
    let m = [Address::generate(env), Address::generate(env), Address::generate(env)];
    let minter = StellarAssetClient::new(env, &token);
    for a in m.iter() {
        minter.mint(a, &1_000);
    }
    let id = env.register(PaluwaganPool, (admin, token.clone(), 100i128));
    let client = PaluwaganPoolClient::new(env, &id);
    for a in m.iter() {
        client.join(a);
    }
    (client, token, m)
}

// 1. Happy path: all contribute, round-0 member receives the full pot.
#[test]
fn test_full_round_payout() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, token, m) = setup(&env);

    for a in m.iter() {
        client.contribute(a, &0u32);
    }
    let recipient = client.payout(&0u32);
    assert_eq!(recipient, m[0]);

    let usdc = TokenClient::new(&env, &token);
    assert_eq!(usdc.balance(&m[0]), 1_200); // 1000 - 100 + 300 pot
}

// 2. Edge: payout blocked until everyone has funded.
#[test]
#[should_panic]
fn test_payout_before_funded_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, m) = setup(&env);

    client.contribute(&m[0], &0u32);
    client.contribute(&m[1], &0u32); // m[2] missing
    client.payout(&0u32); // RoundNotFunded -> panic
}

// 3. State verification: round advances after a payout.
#[test]
fn test_round_advances() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, m) = setup(&env);

    assert_eq!(client.current_round(), 0);
    for a in m.iter() {
        client.contribute(a, &0u32);
    }
    client.payout(&0u32);
    assert_eq!(client.current_round(), 1);
}

// 4. Edge: a member cannot contribute twice in one round.
#[test]
#[should_panic]
fn test_double_contribute_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, m) = setup(&env);

    client.contribute(&m[0], &0u32);
    client.contribute(&m[0], &0u32); // AlreadyContributed -> panic
}

// 5. Edge: a non-member cannot pay into the pool.
#[test]
#[should_panic]
fn test_non_member_contribute_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _token, _m) = setup(&env);

    let outsider = Address::generate(&env);
    client.contribute(&outsider, &0u32); // NotMember -> panic
}
