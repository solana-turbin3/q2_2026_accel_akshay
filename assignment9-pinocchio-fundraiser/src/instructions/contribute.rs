use pinocchio::{AccountView, ProgramResult, cpi::{Seed, Signer}, error::ProgramError, sysvars::Sysvar};
use pinocchio_log::log;

use crate::{constants::MIN_CONTRIBUTION_AMOUNT, state::{FundraiserVault, UserContribution}};

//@c do ata and mint checks
// data =  contribute_amount(u64)
pub fn contribute(accounts:&mut [AccountView],ix_data:&[u8])->ProgramResult{

    log!("accounts len : {}",accounts.len());
    let [
        contributor,
        maker,
        fundraiser_vault_pda,
        contributor_pda,
        contributor_ata,
        fundraiser_vault_ata,
        system_prog,
        token_prog,
        ata_prog
    ]=accounts else{
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !contributor.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *fundraiser_vault_pda.owner()!=crate::ID{
        return Err(ProgramError::IncorrectProgramId);
    }
    if system_prog.address()!=&pinocchio_system::ID || token_prog.address()!=&pinocchio_token::ID || ata_prog.address()!=&pinocchio_associated_token_account::ID{
        return Err(ProgramError::IncorrectProgramId);
    }

    if ix_data.len()<8{
        return Err(ProgramError::InvalidInstructionData);
    }
    let fundraiser_data=unsafe{&mut *(fundraiser_vault_pda.borrow_unchecked_mut().as_mut_ptr() as *mut FundraiserVault)};
    let fundraiser_seeds=[b"fundraiser", maker.address().as_ref(), &[fundraiser_data.bump]];
    let expected_fundraiser_pda=solana_pubkey::Pubkey::derive_address(&fundraiser_seeds, None, &crate::ID);
    if expected_fundraiser_pda!=*fundraiser_vault_pda.address(){
        return Err(ProgramError::InvalidSeeds);
    }

    let clock=pinocchio::sysvars::clock::Clock::get()?;
    let curr_time=clock.unix_timestamp as u64;
    let contribute_amount=unsafe{*(ix_data.as_ptr().add(0) as *const u64)};
    
    let fundraise_target_amount=u64::from_le_bytes(fundraiser_data.target_amount);
    let fundraise_current_amount=u64::from_le_bytes(fundraiser_data.total_amount);
    let fundraise_amount_after_contribution=fundraise_current_amount.checked_add(contribute_amount)
                                        .ok_or(ProgramError::ArithmeticOverflow)?;
    let expiry_time=u64::from_le_bytes(fundraiser_data.expiry_time);
    if curr_time>=expiry_time || fundraise_current_amount>=fundraise_target_amount{
        return Err(ProgramError::InvalidArgument);
    }

    //contributor first time funding
    if contributor_pda.data_len()==0{
        let contributor_seeds=&[b"contributor",contributor.address().as_ref()];
        let (expected_contributor_pda,contributor_bump)=solana_pubkey::Pubkey::find_program_address(contributor_seeds, &crate::ID);
        if &expected_contributor_pda!=contributor_pda.address(){
            return Err(ProgramError::InvalidSeeds);
        }
        if contribute_amount.checked_mul(10).ok_or(ProgramError::ArithmeticOverflow)? >fundraise_target_amount  || contribute_amount<MIN_CONTRIBUTION_AMOUNT{
            return Err(ProgramError::InvalidArgument);
        }
        let contributor_bump_bytes=[contributor_bump];
        let seeds=[
            Seed::from(b"contributor"),
            Seed::from(contributor.address().as_ref()),
            Seed::from(&contributor_bump_bytes)
        ];
        let signers=Signer::from(&seeds);
        pinocchio_system::instructions::CreateAccount{
            from:contributor,to:contributor_pda,space:UserContribution::USER_CONTRIBUTION_SIZE as u64,
            lamports:pinocchio::sysvars::rent::Rent::get()?.minimum_balance_unchecked(UserContribution::USER_CONTRIBUTION_SIZE),
            owner:&crate::ID
        }.invoke_signed(&[signers])?;

        let contributor_data=unsafe{&mut *(contributor_pda.borrow_unchecked_mut().as_ptr() as *mut UserContribution)};
        contributor_data.bump=contributor_bump;
        contributor_data.user=contributor.address().to_bytes();
        contributor_data.amount_contributed=contribute_amount.to_le_bytes();

    }else{
        if contributor_pda.owner()!=&crate::ID{
            return Err(ProgramError::IncorrectProgramId);
        }
        let contributor_data=unsafe{&mut *(contributor_pda.borrow_unchecked_mut().as_ptr() as *mut UserContribution)};
        let contributor_seeds=[b"contributor", contributor.address().as_ref(), &[contributor_data.bump]];
        let derived_contributor_pda=solana_pubkey::Pubkey::derive_address(&contributor_seeds, None, &crate::ID);
        if *contributor_pda.address()!=derived_contributor_pda{
            return Err(ProgramError::InvalidSeeds);
        }

        let total_contributed=u64::from_le_bytes(contributor_data.amount_contributed);
        let user_contribution=contribute_amount.checked_add(total_contributed)
                            .ok_or(ProgramError::ArithmeticOverflow)?;

        if user_contribution.checked_mul(10).ok_or(ProgramError::ArithmeticOverflow)? >fundraise_target_amount  || contribute_amount<MIN_CONTRIBUTION_AMOUNT{
            return Err(ProgramError::InvalidArgument);
        }
        contributor_data.amount_contributed=user_contribution.to_le_bytes();
    }
    
    pinocchio_token::instructions::Transfer::new(
        contributor_ata, fundraiser_vault_ata, contributor, contribute_amount
    ).invoke()?;

    fundraiser_data.total_amount=fundraise_amount_after_contribution.to_le_bytes();
    //create user pda for contribtion to save all how much user contributed, will need this in case of refund
    //contribute to fundraise ata
    //update fndraise pda, total amount
    
    //checks
    //1. can contribute only if total<target and curr_time<expiry
    //2. user can contribute min 1 token and max 10% of total amount
    Ok(())
}