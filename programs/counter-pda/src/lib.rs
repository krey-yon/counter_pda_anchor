use anchor_lang::prelude::*;

declare_id!("JCmHRfx897eap7kkkDiQUHj9aNu6HJmVFqKZANMMQzwb");

#[program]
pub mod counter_pda {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.authority.key();

        msg!("creating a counter");

        msg!("current counter is {}", counter.count);
        msg!("the admin pubkey is this {}", counter.authority);
        Ok(())
    }

    pub fn update_counter(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("adding one to the counter");

        counter.count += 1;

        msg!("Current counter is {}", counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"counter1", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 8
    )]
    counter: Account<'info, Counter>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    authority: Pubkey,
    count: u64,
}
