use anchor_lang::prelude::*; 
use anchor_lang::system_program::{Transfer, transfer};
use crate::error::ErrorCode;
use crate::{Escrow, Make};


#[derive(Accounts)]
pub struct Take<'info>{

    #[account(mut)]
    pub taker : Signer<'info>, 


    #[account(mut)]
    pub maker : SystemAccount<'info>,

    #[account(
        mut,
        has_one = taker, 
        close = maker
    )]
    pub escrow_pda : Account<'info, Escrow>,

    #[account(
        mut,
        seeds = [b"vault", escrow_pda.key().as_ref()],
        bump = escrow_pda.vault_bump
    )]
    pub vault : SystemAccount<'info>, 

    pub system_program : Program<'info, System>
}

pub fn take(
    ctx : Context<Take>, 
)->Result<()>{

    let escrow_pda = &ctx.accounts.escrow_pda; 
    let address = &ctx.accounts.escrow_pda.key(); 

    let bump = [escrow_pda.vault_bump]; 
    let signer_seeds : &[&[u8]] = &[b"vault", address.as_ref(), &bump];

    transfer(
        CpiContext::new_with_signer(ctx.accounts.system_program.key(), 
            Transfer { 
                from: ctx.accounts.vault.to_account_info() , 
                to: ctx.accounts.taker.to_account_info()
            }, &[signer_seeds]),
             ctx.accounts.escrow_pda.amount)?;
    Ok(())
}