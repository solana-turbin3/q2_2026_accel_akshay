use pinocchio::{AccountView, ProgramResult, error::ProgramError, sysvars::Sysvar};

use crate::state::FundraiserVault;

pub fn claim_fund(
    accounts:&mut [AccountView], _ix_data:&[u8]
)->ProgramResult{

    let [
        maker,
        fundraise_pda,
        fundraise_pda_ata,
        maker_ata,
        token_prog
    ]=accounts else{
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !maker.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *token_prog.address()!=pinocchio_token::id(){
        return Err(ProgramError::IncorrectProgramId);
    }
    if *fundraise_pda.owner()!=crate::ID{
        return Err(ProgramError::InvalidAccountOwner);
    }
    
    let fundraise_pda_data=unsafe{&mut *(fundraise_pda.borrow_unchecked_mut().as_ptr() as *mut FundraiserVault)};
    let seeds=&[b"fundraiser",maker.address().as_ref(),&[fundraise_pda_data.bump]];
    let derived_fundraise_pda=solana_pubkey::Pubkey::derive_address(seeds, None, &crate::ID);
    if *fundraise_pda.address()!=derived_fundraise_pda{
        return Err(ProgramError::InvalidSeeds);
    }

    let curr_time=pinocchio::sysvars::clock::Clock::get()?.unix_timestamp as u64;
    let expiry_time=u64::from_le_bytes(fundraise_pda_data.expiry_time);

    if curr_time<expiry_time && u64::from_le_bytes(fundraise_pda_data.total_amount)<u64::from_le_bytes(fundraise_pda_data.target_amount){
        return Err(ProgramError::InvalidArgument);
    }

    pinocchio_token::instructions::Transfer::new(
        fundraise_pda_ata,
        maker_ata,
        fundraise_pda,
        u64::from_le_bytes(fundraise_pda_data.total_amount)
    ).invoke()?;
    
    //close ata
    pinocchio_token::instructions::CloseAccount::new(fundraise_pda_ata, maker, fundraise_pda).invoke()?;
   
   //@q can we transfer sol from pda using system progr or not
   pinocchio_system::instructions::Transfer{from:fundraise_pda, to:maker,lamports:fundraise_pda.lamports()}.invoke()?;
   let fundraise_pda_data=unsafe{fundraise_pda.borrow_unchecked_mut()};
   fundraise_pda_data.fill(0);

    //check if total amount>=target amount and curr_time>=expiry , then only can clain fund
    //after claiming, close the fundriase pda and ata if possible
    Ok(())
}