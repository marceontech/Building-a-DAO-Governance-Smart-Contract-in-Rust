use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

declare_id!("FILL_WITH_YOUR_PROGRAM_ID");

#[program]
pub mod dao_governance {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal_account;
        proposal.title = title;
        proposal.votes_yes = 0;
        proposal.votes_no = 0;
        proposal.executed = false;
        
        msg!("Proposal Created: {}", proposal.title);
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, approve: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal_account;
        require!(!proposal.executed, ErrorCode::ProposalAlreadyExecuted);

        if approve {
            proposal.votes_yes += 1;
        } else {
            proposal.votes_no += 1;
        }

        msg!("Vote Recorded: Yes = {}, No = {}", proposal.votes_yes, proposal.votes_no);
        Ok(())
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal_account;
        require!(proposal.votes_yes > proposal.votes_no, ErrorCode::NotEnoughSupport);
        
        proposal.executed = true;
        msg!("Proposal Executed: {}", proposal.title);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = proposer, space = 256)]
    pub proposal_account: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal_account: Account<'info, Proposal>,
    #[account(mut)]
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub proposal_account: Account<'info, Proposal>,
    #[account(signer)]
    pub admin: Signer<'info>,
}

#[account]
pub struct Proposal {
    pub title: String,
    pub votes_yes: u64,
    pub votes_no: u64,
    pub executed: bool,
}
