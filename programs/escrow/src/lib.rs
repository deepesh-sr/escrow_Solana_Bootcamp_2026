pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8pdKGcQmZShTZxUMJbpnvcpuTc2d9Rn76Y9iqBAVRqUL");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn make(ctx : Context<Make>, seed : u32,
    taker : Pubkey, 
    amount : u64 )->Result<()>{
        maker::make(ctx, seed, taker, amount)
    }

    pub fn take(ctx : Context<Take>)->Result<()>{
        take::take(ctx)
    }
}
