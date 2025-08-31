use anchor_lang::prelude::*;

declare_id!("JCmHRfx897eap7kkkDiQUHj9aNu6HJmVFqKZANMMQzwb");

#[program]
pub mod counter_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
