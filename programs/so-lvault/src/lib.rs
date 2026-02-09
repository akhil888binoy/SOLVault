use anchor_lang::prelude::*;

declare_id!("EAth84uR4AP7WF6SAc3Pu3yaPJyjg2AE7XD51EoTCD4r");

#[program]
pub mod so_lvault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
