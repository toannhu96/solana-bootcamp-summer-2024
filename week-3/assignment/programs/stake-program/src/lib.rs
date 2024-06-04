use anchor_lang::prelude::*;
use instructions::*;

mod contants;
mod errors;
mod instructions;
mod state;

declare_id!("C153Jyim6dDSPeLMHFQe3LTPrhPXLVr16Cz5rPEeKvrJ");

#[program]
pub mod stake_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake(ctx)
    }
}
