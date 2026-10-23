use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::error::ErrorCode;
use crate::Escrow;


#[derive(Accounts)]
#[instruction(seed : u32, taker : Pubkey, amount : u64)]
pub struct Make<'info>{

    #[account(mut)]
    pub maker : Signer<'info>, 

    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE, 
        seeds = [b"escrow" , maker.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub escrow_pda : Account<'info, Escrow>,

    #[account(
        mut, 
        seeds = [b"vault", escrow_pda.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>, 

    pub system_program : Program<'info, System>
}


pub fn make(
    ctx : Context<Make>, 
    seed : u32,
    taker : Pubkey, 
    amount : u64 
)->Result<()>{

    require!(amount > 0 ,ErrorCode::InvalidAmount); 

    let escrow = &mut ctx.accounts.escrow_pda; 
    escrow.maker = ctx.accounts.maker.key();
    escrow.seed = seed;
    escrow.taker = taker; 
    escrow.amount = amount; 
    escrow.bump = ctx.bumps.escrow_pda; 
    escrow.vault_bump = ctx.bumps.vault; 
    

    transfer(
        CpiContext::new(
        ctx.accounts.system_program.key(), 
        
        Transfer 
        { from: ctx.accounts.maker.to_account_info() , to: ctx.accounts.vault.to_account_info() 
        }
    ), 
        amount
    )?; 

    Ok(())
}