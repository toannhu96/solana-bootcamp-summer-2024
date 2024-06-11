use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, Mint, Token, TokenAccount, Transfer},
};
use spl_math::uint::U256;

use crate::state::Pool;

pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, _amount_desired: u64) -> Result<()> {
    let liquidity = if _amount_desired > ctx.accounts.depositor_account_liquidity.amount {
        ctx.accounts.depositor_account_liquidity.amount
    } else {
        _amount_desired
    };

    let amount_a = U256::from(ctx.accounts.pool_account_a.amount)
        .checked_div(U256::from(ctx.accounts.pool_account_b.amount))
        .unwrap()
        .integer_sqrt()
        .checked_mul(U256::from(liquidity))
        .unwrap()
        .as_u64();

    let amount_b = U256::from(amount_a)
        .checked_mul(U256::from(ctx.accounts.pool_account_b.amount))
        .unwrap()
        .checked_div(U256::from(ctx.accounts.pool_account_a.amount))
        .unwrap()
        .as_u64();

    let authority_bump = ctx.bumps.pool_authority;

    let authority_seeds = &[
        &ctx.accounts.pool.amm.to_bytes(),
        &ctx.accounts.pool.mint_a.to_bytes(),
        &ctx.accounts.pool.mint_b.to_bytes(),
        b"authority".as_ref(),
        &[authority_bump],
    ];

    let signer_seeds = &[&authority_seeds[..]];

    // transfer token A pool A to user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool_account_a.to_account_info(),
                to: ctx.accounts.depositor_account_a.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            signer_seeds,
        ),
        amount_a,
    )?;

    // transfer token B pool B to user
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool_account_b.to_account_info(),
                to: ctx.accounts.depositor_account_b.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            signer_seeds,
        ),
        amount_b,
    )?;

    // Burn LP token of LProvider
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint_liquidity.to_account_info(),
                from: ctx.accounts.depositor_account_liquidity.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        liquidity,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(
        mut,
        seeds = [
            pool.amm.key().as_ref(),
            pool.mint_a.key().as_ref(),
            pool.mint_b.key().as_ref()
        ],
        bump,
        has_one = mint_a,
        has_one = mint_b
    )]
    pool: Box<Account<'info, Pool>>,

    /// CHECK read-only account
    #[account(
        seeds = [
            pool.amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            b"authority"
        ],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    pub mint_a: Box<Account<'info, Mint>>,

    pub mint_b: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            pool.amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            b"mint_liquidity"
        ],
        bump,
        mint::decimals = 6,
        mint::authority = pool_authority
    )]
    pub mint_liquidity: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pool_account_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pool_account_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    depositor_account_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = depositor,
    )]
    depositor_account_b: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_liquidity,
        associated_token::authority = depositor,
    )]
    depositor_account_liquidity: Account<'info, TokenAccount>,

    #[account(mut)]
    depositor: Signer<'info>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}
