#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token::TokenClient, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyExists = 1,
    NotFound = 2,
    AlreadySettled = 3,      // already released or refunded
    DeadlineNotReached = 4,
    InvalidAmount = 5,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Status {
    Funded,
    Released,
    Refunded,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Invoice(u64),
}

#[contracttype]
#[derive(Clone)]
pub struct Invoice {
    pub buyer: Address,
    pub supplier: Address,
    pub amount: i128,
    pub deadline: u64, // ledger timestamp after which the buyer may refund
    pub status: Status,
}

#[contract]
pub struct TindaEscrow;

#[contractimpl]
impl TindaEscrow {
    pub fn __constructor(env: Env, admin: Address, token: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    // Buyer escrows the order amount in USDC against a new invoice id.
    pub fn fund_invoice(
        env: Env,
        buyer: Address,
        id: u64,
        supplier: Address,
        amount: i128,
        deadline: u64,
    ) -> Result<(), Error> {
        buyer.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if env.storage().persistent().has(&DataKey::Invoice(id)) {
            return Err(Error::AlreadyExists);
        }
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        TokenClient::new(&env, &token).transfer(&buyer, &env.current_contract_address(), &amount);

        let inv = Invoice { buyer, supplier, amount, deadline, status: Status::Funded };
        env.storage().persistent().set(&DataKey::Invoice(id), &inv);
        env.storage().persistent().extend_ttl(&DataKey::Invoice(id), 17_280, 518_400);
        Ok(())
    }

    // Buyer confirms goods arrived -> release escrow to supplier.
    pub fn confirm_delivery(env: Env, id: u64) -> Result<(), Error> {
        let mut inv: Invoice = env
            .storage()
            .persistent()
            .get(&DataKey::Invoice(id))
            .ok_or(Error::NotFound)?;
        inv.buyer.require_auth(); // only the buyer can release their escrow
        if inv.status != Status::Funded {
            return Err(Error::AlreadySettled);
        }
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        TokenClient::new(&env, &token)
            .transfer(&env.current_contract_address(), &inv.supplier, &inv.amount);

        inv.status = Status::Released;
        env.storage().persistent().set(&DataKey::Invoice(id), &inv);
        Ok(())
    }

    // Buyer reclaims funds if the deadline passes without delivery.
    pub fn refund(env: Env, id: u64) -> Result<(), Error> {
        let mut inv: Invoice = env
            .storage()
            .persistent()
            .get(&DataKey::Invoice(id))
            .ok_or(Error::NotFound)?;
        inv.buyer.require_auth();
        if inv.status != Status::Funded {
            return Err(Error::AlreadySettled);
        }
        if env.ledger().timestamp() < inv.deadline {
            return Err(Error::DeadlineNotReached); // can't refund early
        }
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        TokenClient::new(&env, &token)
            .transfer(&env.current_contract_address(), &inv.buyer, &inv.amount);

        inv.status = Status::Refunded;
        env.storage().persistent().set(&DataKey::Invoice(id), &inv);
        Ok(())
    }

    pub fn get_invoice(env: Env, id: u64) -> Option<Invoice> {
        env.storage().persistent().get(&DataKey::Invoice(id))
    }
}

mod test;
