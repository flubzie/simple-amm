use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use std::mem::size_of;

use crate::state::{Amm, Pool};

// create pool account
pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.amm  = ctx.accounts.amm.key();

    pool.mint_a = ctx.accounts.mint_a.key();
    pool.mint_b = ctx.accounts.mint_a.key();

    Ok(())
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    // add the pool account, init if it doesn't already exist
    // validate that the PDA is what we expect
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Pool>(),
        seeds = [
            amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
        ],
        bump,
        // TODO: constraint to enforce order of token mint keys
    )]
    pub pool: Account<'info, Pool>,

    // we need the amm account here to use its key
    #[account(
        seeds = [
            "simple-amm".as_ref()
        ],
        bump,
    )]
    pub amm: Account<'info, Amm>,

    // we need the mint addresses of both pool tokens
    pub mint_a: Box<Account<'info, Mint>>,

    pub mint_b: Box<Account<'info, Mint>>,

    // we need to create the mint account of the liquidity tokens
    #[account(
        init,
        payer = payer,
        seeds = [
            amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            "liquidity".as_ref(), //TODO: make this a constant
        ],
        bump,
        mint::decimals = 6,
        mint:: authority = amm_authority, // needs an authority that should be separate from our other accounts
    )]
    pub mint_liquidity: Box<Account<'info, Mint>>,

    // need a amm_authority which is a PDA that has authority over our liquidity token mints
    #[account(
        seeds = [
            amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            "authority".as_ref(), //TODO: make this a constant
        ],
        bump,
    )]
    pub amm_authority: AccountInfo<'info>, //maybe we could have one pool_authority for every pool, but managing that may become difficult

    // need pool accounts which will be associated token accounts holding the two tokens of the pair
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_a, //this account holds token a
        associated_token::authority = amm_authority,
    )]
    pub pool_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_b, //this account holds token b
        associated_token::authority = amm_authority,
    )]
    pub pool_account_b: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
