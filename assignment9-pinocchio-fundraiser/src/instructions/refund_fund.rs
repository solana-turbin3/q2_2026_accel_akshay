use pinocchio::{AccountView, ProgramResult, error::ProgramError, sysvars::Sysvar};

use crate::state::{FundraiserVault, UserContribution};

pub fn refund_fund(accounts:&mut [AccountView], ix_data:&[u8])->ProgramResult{
    let [
        contributor,
        maker,
        fundraise_pda,
        contributor_pda,
        fundraise_ata,
        contributor_ata,
        token_prog
    ]=accounts else{
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !contributor.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *token_prog.address()!=pinocchio_token::id(){
        return Err(ProgramError::IncorrectProgramId);
    }
    if *fundraise_pda.owner()!=crate::ID || *contributor_pda.owner()!=crate::ID{
        return Err(ProgramError::InvalidAccountOwner);
    }
    
    let fundraise_pda_data=unsafe{&mut *(fundraise_pda.borrow_unchecked_mut().as_ptr() as *mut FundraiserVault)};
    let contributor_pda_data=unsafe{&mut *(contributor_pda.borrow_unchecked_mut().as_ptr() as *mut UserContribution)};

    let fundraise_seeds=&[b"fundraiser",maker.address().as_ref(),&[fundraise_pda_data.bump]];
    let derived_fundraise_pda=solana_pubkey::Pubkey::derive_address(fundraise_seeds, None, &crate::ID);
    if *fundraise_pda.address()!=derived_fundraise_pda{
        return Err(ProgramError::InvalidSeeds);
    }

    let contributor_seeds=&[b"contributor",contributor.address().as_ref(),&[contributor_pda_data.bump]];
    let derived_contributor_pda=solana_pubkey::Pubkey::derive_address(contributor_seeds, None, &crate::ID);
    if *contributor_pda.address()!=derived_contributor_pda{
        return Err(ProgramError::InvalidSeeds);
    }
    //derive pda and pda check
    //@q token mint checks needed

    let curr_time=pinocchio::sysvars::clock::Clock::get()?.unix_timestamp as u64;

    let expiry=u64::from_le_bytes(fundraise_pda_data.expiry_time);
    let target_amount=u64::from_le_bytes(fundraise_pda_data.target_amount);
    let total_amount=u64::from_le_bytes(fundraise_pda_data.total_amount);
    let amount_contributed=u64::from_le_bytes(contributor_pda_data.amount_contributed);

    //check if fundraise has failed means, after expiry time, total_amount < target_amount
    //if so , then contributr can refund how much they contributed to fundriaser,else dont allow he refund
    if curr_time<expiry || total_amount>target_amount{ //dont allow refunds
        return Err(ProgramError::InvalidArgument);
    }
    pinocchio_token::instructions::Transfer::new(
        fundraise_ata, contributor_ata, fundraise_pda, amount_contributed
    ).invoke()?;

    //update total_amount in fundraiser pda after refund
    let total_amount_after_refund=(total_amount-amount_contributed).to_le_bytes();
    fundraise_pda_data.total_amount=total_amount_after_refund;


    //close contributor pda after refund 
    pinocchio_system::instructions::Transfer{
        from:contributor_pda,to:contributor,lamports:contributor_pda.lamports()
    }.invoke()?;
    let contributor_data=unsafe{contributor_pda.borrow_unchecked_mut()};
    contributor_data.fill(0);

    Ok(())
}