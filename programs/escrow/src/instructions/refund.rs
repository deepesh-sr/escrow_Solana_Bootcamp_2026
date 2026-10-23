use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        has_one = maker,
        close = maker
    )]
    pub escrow_pda: Account<'info, Escrow>,

    #[account(
        mut,
        seeds = [b"vault", escrow_pda.key().as_ref()],
        bump = escrow_pda.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn refund(ctx: Context<Refund>) -> Result<()> {
    let escrow = &ctx.accounts.escrow_pda;

    let escrow_key = escrow.key();
    let bump = [escrow.vault_bump];
    let seeds: &[&[u8]] = &[b"vault", escrow_key.as_ref(), &bump];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.key(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.maker.to_account_info(),
            },
            &[seeds],
        ),
        escrow.amount,
    )?;

    Ok(())
}
