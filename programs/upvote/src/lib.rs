use anchor_lang::prelude::*;

declare_id!("7LGT8vNzfWuQ1utvdLU8r5iYiydaHpKKJTfwB2pQL7PZ");

#[program]
pub mod upvote {

    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let base_account: &mut Account<BaseAccount> = &mut _ctx.accounts.base_account;
        base_account.total_submissions = 0;
        Ok(())
    }

    pub fn add_submission(_ctx: Context<AddSubmission>, title: String, description:String) -> Result <()> {
        let base_account = &mut _ctx.accounts.base_account;
        let user = &mut _ctx.accounts.user;
        let clock: Clock = Clock::get().unwrap();

        let submission = SubmissionStruct {
            timestamp: clock.unix_timestamp,
            title: title.to_string(),
            description: description.to_string(),
            user_address: *user.to_account_info().key,
        };
        base_account.submission_list.push(submission);
        base_account.total_submissions += 1;
        Ok(())

    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddSubmission<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SubmissionStruct {
    pub timestamp: i64,
    pub title: String,
    pub description: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_submissions: u64,
    pub submission_list: Vec<SubmissionStruct>,
}
