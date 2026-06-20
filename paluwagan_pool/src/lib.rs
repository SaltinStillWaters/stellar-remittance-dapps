#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token::TokenClient, Address, Env, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotMember = 1,
    AlreadyMember = 2,
    AlreadyContributed = 3,
    RoundNotFunded = 4,
    PoolComplete = 5,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Contribution,        // fixed USDC amount per member per round
    Members,             // Vec<Address>, join order == payout order
    Round,               // current round index (u32)
    Paid(u32, Address),  // contribution flag for (round, member)
}

#[contract]
pub struct PaluwaganPool;

#[contractimpl]
impl PaluwaganPool {
    // Deploy-time config: admin, USDC token, and the per-round contribution.
    pub fn __constructor(env: Env, admin: Address, token: Address, contribution: i128) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Contribution, &contribution);
        env.storage().instance().set(&DataKey::Members, &Vec::<Address>::new(&env));
        env.storage().instance().set(&DataKey::Round, &0u32);
    }

    // A worker joins the circle before contributions begin.
    pub fn join(env: Env, member: Address) -> Result<(), Error> {
        member.require_auth();
        let mut members: Vec<Address> = env.storage().instance().get(&DataKey::Members).unwrap();
        if members.contains(&member) {
            return Err(Error::AlreadyMember);
        }
        members.push_back(member);
        env.storage().instance().set(&DataKey::Members, &members);
        Ok(())
    }

    // Member locks their fixed USDC for the given round.
    pub fn contribute(env: Env, member: Address, round: u32) -> Result<(), Error> {
        member.require_auth();
        let members: Vec<Address> = env.storage().instance().get(&DataKey::Members).unwrap();
        if !members.contains(&member) {
            return Err(Error::NotMember); // only members can pay in
        }
        if env.storage().persistent().has(&DataKey::Paid(round, member.clone())) {
            return Err(Error::AlreadyContributed); // one contribution per round
        }
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Contribution).unwrap();
        TokenClient::new(&env, &token)
            .transfer(&member, &env.current_contract_address(), &amount);
        env.storage().persistent().set(&DataKey::Paid(round, member), &true);
        Ok(())
    }

    // Once every member has paid, release the whole pot to this round's recipient.
    pub fn payout(env: Env, round: u32) -> Result<Address, Error> {
        let members: Vec<Address> = env.storage().instance().get(&DataKey::Members).unwrap();
        let current: u32 = env.storage().instance().get(&DataKey::Round).unwrap();
        if round != current {
            return Err(Error::RoundNotFunded);
        }
        if current >= members.len() {
            return Err(Error::PoolComplete); // every member already received a turn
        }
        // Fairness gate: no payout until all members funded this round.
        for m in members.iter() {
            if !env.storage().persistent().has(&DataKey::Paid(round, m.clone())) {
                return Err(Error::RoundNotFunded);
            }
        }
        let amount: i128 = env.storage().instance().get(&DataKey::Contribution).unwrap();
        let pot = amount.checked_mul(members.len() as i128).unwrap(); // overflow-safe
        let recipient = members.get(current).unwrap(); // round r -> member r

        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        TokenClient::new(&env, &token)
            .transfer(&env.current_contract_address(), &recipient, &pot);

        env.storage().instance().set(&DataKey::Round, &(current + 1));
        Ok(recipient)
    }

    pub fn current_round(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Round).unwrap()
    }

    pub fn members(env: Env) -> Vec<Address> {
        env.storage().instance().get(&DataKey::Members).unwrap()
    }
}

mod test;
