use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("2fyDo7MXWaRHn5pFanYxs2EenfbUkWrfJhDVzfmz6zvX");

#[program]
pub mod crowdfunding {
    use super::*;
    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.admin = *ctx.accounts.user.key;
        Ok(())
    }//context is always the first argument in anchor
    //context of the create function is the list of accounts from which the data can be retrieved from
    pub fn withhdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;
        if campaign.admin != *user.key{
            return Err(ProgramError::IncorrectProgramId);
        }
        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        //rent is based on the amount of data that's stored in the account
        if **campaign.to_account_info().lamports.borrow()-rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        **campaign.to_account_info().try_borrow_mut_lamports() ? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult{
        //the way we donate to campaign is different from the way we withdraw
        //code will be different
        //System Instruction Transfer is used
        //why?? the fund's are being sent from a user's wallet program doesnt 
        //have the authority over the user's wallet, we can only do the transaction
        //via a system instruction
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info()
            ]
        );
        (&mut ctx.accounts.campaign).amount_donated += amount;
        Ok(())
    }
}


#[derive(Accounts)]//this is a macro with the hash symbol
//the macro is used to indicate that it is a context
pub struct Create<'info> {
    //describing properties for the accounts we are going to use
    //we dont have a campaign account yet because we need to create a campaign account with this function
    //we add an init macro above this campaign property
    //amount of space required to allocate for the campaign account on the blockchain
    //campaign account needs to be a program derived account
    #[account(init, payer=user, space=9000, seeds=[b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
    //using these seeds Solana will use a hash function to determine the address for a new program derived account
    //there is a possibility that the address is used for someone else's wallet somewhere, to prevent it we can add a bump
    //bump adds an 8 bit bump to the hash function until we find an address that isnt being used for a wallet.
    pub campaign: Account<'info, Campaign>,
    //we dont use init for user or system_program because we dont create them
    #[account(mut)]//makes user account mutable - we can change it
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct Donate<'info>{
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Campaign{
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_donated: u64
}









/*
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
#[derive(Accounts)]
pub struct Initialize {}

*/