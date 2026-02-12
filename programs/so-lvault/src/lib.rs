use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("EAth84uR4AP7WF6SAc3Pu3yaPJyjg2AE7XD51EoTCD4r");

#[program]
pub mod so_lvault {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault: &mut Initialize<'_> = ctx.accounts;
        vault.new_account.payer = vault.payer.key(); //assign authority
        msg!("Initialized Program : {:?}", ctx.program_id);
        Ok(())
    }

    pub fn withdraw(ctx:Context<ModifyVault>, amount:u64) -> Result<()>{
            let bump = ctx.bumps.vault;

            let signer_seeds: &[&[&[u8]]] = &[&[
                b"solvault",
                ctx.accounts.payer.key.as_ref(),
                &[bump],
            ]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.vault.to_account_info().clone(),
                to: ctx.accounts.payer.to_account_info().clone(),
            },
            signer_seeds
        ); // make pda as signer and transfer sol
        system_program::transfer(cpi_context, amount)?;
        msg!("Withdrawal Done : {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx:Context<ModifyVault> , amount:u64)-> Result<()>{
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info().clone(),
                to: ctx.accounts.vault.to_account_info().clone(),
            },
        ); // trasfer sol form depositor to vault
        system_program::transfer(cpi_context, amount)?;
        msg!("Deposit Done : {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init ,
        payer = payer,
        space= 8+32,
        seeds = [b"solvault", payer.key().as_ref()],
        bump
    )]
    pub new_account : Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyVault <'info>{
    #[account(
        mut,
        has_one = payer,
        seeds = [b"solvault", payer.key().as_ref()],
        bump
    )] //every depositor have different vault 
    pub vault:Account<'info , Vault>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[account]
pub struct Vault{
    pub payer: Pubkey
}

