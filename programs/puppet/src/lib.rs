use anchor_lang::prelude::*;

declare_id!("7YWUjFm6NtfzSJjTfRBuR65V3N5Zoi2wDDasdxkkJZcF");

#[program]
pub mod puppet {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        ctx.accounts.puppet.authority = authority;
        Ok(())
    }
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<u64> {
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(data)
    }    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
}

#[account]
pub struct Data {
    pub data: u64,
    pub authority: Pubkey
}
