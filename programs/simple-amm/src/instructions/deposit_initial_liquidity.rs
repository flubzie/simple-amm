use anchor_lang::prelude::*;
use anchor_spl:: {
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{Amm, Pool};

//instruction to deposit liquidity for the first time in a new pool and set initial ratio

// Accounts
// amm (fee), pool, amm_authority
// from and to addresses for mint: depositor_account_a, depositor_account_b && pool_account_a, pool_account_b
// from and to addresses for liquidity: mint_liquidity, depositor_account_liquidity
// mint_a and mint_b which will be the token pair

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

    pub mint_a: Box<Account<'info, Mint>>,

    pub mint_b: Box<Account<'info, Mint>>,

    #[account(
        seeds = [
            "simple-amm".as_ref(), //@TODO: make this a constant
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            "authority".as_ref(), //TODO: make this a constant
        ],
        bump
    )]
    pub amm_authority: AccountInfo<'info>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_b: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            "authority".as_ref(), //@TODO: make this a constant
        ],
        bump
    )]
    pub mint_liquidity: Box<Account<'info, Mint>>,

    // associated token account to hold depositor's liquidity tokens
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_liquidity,
        associated_token::authority = depositor,
    )]
    pub depositor_account_liqudity: Box<Account<'info, TokenAccount>>,

    pub depositor: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
