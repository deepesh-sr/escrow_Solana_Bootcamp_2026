use anchor_lang::prelude::*; 

#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub maker : Pubkey,
    pub taker : Pubkey,
    pub seed : u32,
    pub amount : u64, 
    pub bump : u8, 
    pub vault_bump : u8
}