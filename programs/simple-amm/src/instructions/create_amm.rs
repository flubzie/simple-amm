use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::state::Amm;

// create the AMM accounts
pub fn create_amm(ctx: Context<CreateAmm>, fee: u16) -> Result<()> {
    let amm = &mut ctx.accounts.amm;
    // set the admin value to be the admin pubkey
    amm.admin = ctx.accounts.admin.key();
    // set the fee
    amm.fee = fee;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateAmm<'info> {
    // add the amm account, the admin account, payer account for the amm init
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Amm>(),
        seeds = [
            "simple-amm".as_ref() //TODO: make this a constant
        ],
        bump,
        // @TODO: add fee constraint
    )]
    pub amm: Account<'info, Amm>,

    // include admin, we want to pull the admin account here using the pubkey
    pub admin: AccountInfo<'info>,

    // payer for init rent
    #[account(mut)]
    pub payer: Signer<'info>,

    // system program for clarity
    pub system_program: Program<'info, System>,
}
