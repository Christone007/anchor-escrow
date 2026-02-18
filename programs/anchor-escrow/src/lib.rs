use anchor_lang::prelude::*;

declare_id!("Cu8F3b5j5noX3p9ayGAsZdgidpgFbm2AgWWHudryi5xD");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
