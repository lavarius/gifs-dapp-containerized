use anchor_lang::prelude::*;

declare_id!("D5ErZkzVpvkhv4pZoFWoN9Fku7SWzwpKxqux9rMwqNR7");

#[program]
pub mod gifs_dapp_containerized {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
