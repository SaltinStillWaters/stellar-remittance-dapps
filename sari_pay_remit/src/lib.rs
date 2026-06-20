#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token::TokenClient, Address, Bytes,
    BytesN, Env,
};

// Errors surfaced to the caller; the generated client panics on Err,
// so `#[should_panic]` tests can assert these failure paths.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyExists = 1,
    NotFound = 2,
    AlreadyClaimed = 3,
    InvalidCode = 4,
    InvalidAmount = 5,
}

// Typed storage keys prevent key collisions between config and per-remittance state.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,       // SAC address of USDC
    Remit(u64),  // one escrow record per remittance id
}

#[contracttype]
#[derive(Clone)]
pub struct Remittance {
    pub sender: Address,
    pub store: Address,        // cash-out agent allowed to claim
    pub amount: i128,
    pub code_hash: BytesN<32>, // sha256 of the secret OTP given to the recipient
    pub claimed: bool,
}

#[contract]
pub struct SariPayRemit;

#[contractimpl]
impl SariPayRemit {
    // Runs once at deploy: pins the admin and the USDC token contract.
    pub fn __constructor(env: Env, admin: Address, token: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    // Sender escrows USDC under a hashed one-time code. This is the "send" half of the MVP.
    pub fn create_remittance(
        env: Env,
        sender: Address,
        id: u64,
        store: Address,
        amount: i128,
        code_hash: BytesN<32>,
    ) -> Result<(), Error> {
        sender.require_auth(); // only the sender can spend their USDC
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if env.storage().persistent().has(&DataKey::Remit(id)) {
            return Err(Error::AlreadyExists); // ids are unique, no overwrite
        }

        // Pull real USDC from sender into the contract (the escrow).
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        TokenClient::new(&env, &token).transfer(&sender, &env.current_contract_address(), &amount);

        let remit = Remittance { sender, store, amount, code_hash, claimed: false };
        env.storage().persistent().set(&DataKey::Remit(id), &remit);
        // Keep the escrow alive ~30 days so it can't be archived before pickup.
        env.storage().persistent().extend_ttl(&DataKey::Remit(id), 17_280, 518_400);
        Ok(())
    }

    // Store redeems by presenting the secret code. This is the "cash-out" half of the MVP.
    pub fn claim(env: Env, id: u64, code: Bytes) -> Result<(), Error> {
        let mut remit: Remittance = env
            .storage()
            .persistent()
            .get(&DataKey::Remit(id))
            .ok_or(Error::NotFound)?;

        remit.store.require_auth(); // only the designated cash-out agent
        if remit.claimed {
            return Err(Error::AlreadyClaimed); // no double spend
        }

        // Commitment check: the code must hash to what the sender committed to.
        let computed: BytesN<32> = env.crypto().sha256(&code).into();
        if computed != remit.code_hash {
            return Err(Error::InvalidCode);
        }

        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        TokenClient::new(&env, &token)
            .transfer(&env.current_contract_address(), &remit.store, &remit.amount);

        remit.claimed = true;
        env.storage().persistent().set(&DataKey::Remit(id), &remit);
        Ok(())
    }

    // Read-only lookup for UIs / receipts.
    pub fn get_remittance(env: Env, id: u64) -> Option<Remittance> {
        env.storage().persistent().get(&DataKey::Remit(id))
    }
}

mod test;
