use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("DscCutv9QLURJGrs1hwoYAgeztnVyZQnkau5ZLnfLKQ9");

#[program]
pub mod gifs_dapp_containerized {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        base_account.gif_list = Vec::new(); // To Always Initialize the vector
        Ok(())
    }
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer=user, space=8 + 8 + 4 + (10 * (4 + 32 + 256)))]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}