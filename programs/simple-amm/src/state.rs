use anchor_lang::prelude::*;

// define a pub struct for the AMM account
#[account]
#[derive(Default)]
pub struct Amm {
    // adding an id field so that we can deterministically generate 
    // admin field won't do anything yet
    // but it could be useful for limiting certain instructions executed via the program (given this is a PDA) to this address
    pub admin: Pubkey,

    // LP fee in basis points
    pub fee: u16,
}

#[account]
#[derive(Default)]
pub struct Pool {
    pub amm: Pubkey,

    pub mint_a: Pubkey,

    pub mint_b: Pubkey,
}
