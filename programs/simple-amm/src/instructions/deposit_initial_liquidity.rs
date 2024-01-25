use anchor_lang::prelude::*;

use crate::state::{Amm, Pool};

//instruction to deposit liquidity for the first time in a new pool and set initial ratio

// Accounts
// amm (fee), pool, amm_authority
// from and to addresses for mint: depositor_account_a, depositor_account_b && pool_account_a, pool_account_b
// from and to addresses for liquidity: mint_liquidity, depositor_account_liquidity

// Instruction
// Take the amounts of each token being passed, create a ratio, calculate liquidity using geo mean,
//  deposit tokens, mint liquidity tokens
pub fn deposit_initial_liquidity(ctx: Context<DepositInitialLiquidity>) -> Result<()> {


    Ok(())
}

#[derive(Accounts)]
pub struct DepositInitialLiquidity<'info> {
    #[account(
        seeds = [
            "simple-amm".as_ref() //@TODO: make this a constant
        ],
        bump
    )]
    pub amm: Account<'info, Amm>,

    #[account(
        seeds = [
            amm.key().as_ref(), //@TODO: make this a constant
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
        ],
        bump,
    )]
    pub pool: Account<'info, Pool>,
}
