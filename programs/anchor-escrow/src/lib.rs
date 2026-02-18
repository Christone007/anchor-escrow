use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("Cu8F3b5j5noX3p9ayGAsZdgidpgFbm2AgWWHudryi5xD");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>) -> Result<()> {
        ctx.accounts.init_escrow();
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
