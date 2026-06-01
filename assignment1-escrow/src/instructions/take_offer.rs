use solana_program::{
    pubkey::Pubkey, account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, program_error::ProgramError,
    rent::Rent, sysvar::Sysvar,msg,clock::Clock,
    program::{invoke,invoke_signed}
};
use solana_system_interface;

use spl_associated_token_account_interface::{instruction::create_associated_token_account};
use spl_token_interface::{instruction::transfer};

use crate::state::{EscrowVault, EscrowVaultState};
use borsh::{BorshSerialize,BorshDeserialize};

pub fn take_offer(program_id:&Pubkey, accounts:&[AccountInfo], escrow_id:u64,escrow_bump:u8)->ProgramResult{
    let accounts_iter=&mut accounts.iter();

    let taker=next_account_info(accounts_iter)?;
    let escrow_vault_pda=next_account_info(accounts_iter)?;
    let taker_ata_a=next_account_info(accounts_iter)?;
    let taker_ata_b=next_account_info(accounts_iter)?;

    let maker_ata_b=next_account_info(accounts_iter)?;
    let escrow_vault_ata_a=next_account_info(accounts_iter)?;
    
    let token_prog=next_account_info(accounts_iter)?;
    let ata_prog=next_account_info(accounts_iter)?;
    let system_prog=next_account_info(accounts_iter)?;
    
    if !taker.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *system_prog.key!=solana_system_interface::program::ID{
        return Err(ProgramError::IncorrectProgramId);
    }
    
    let escrow_id_bytes=escrow_id.to_le_bytes();
    let escrow_seeds=&[b"escrow_vault",escrow_id_bytes.as_ref(), &[escrow_bump]];
    
    let expected_escrow_pda=Pubkey::create_program_address(escrow_seeds, program_id)?;
    if *escrow_vault_pda.key!=expected_escrow_pda{
        return Err(ProgramError::InvalidSeeds);
    }
    
    let mut escrow=EscrowVault::try_from_slice(&escrow_vault_pda.data.borrow())?;
    
    let curr_time=Clock::get()?.unix_timestamp as u64;
    if escrow.escrow_state!=EscrowVaultState::PENDING && escrow.expiry_time<=curr_time{
        return Err(ProgramError::InvalidArgument);
    }

    // let token_a_amount=escrow.token_a_amount;
    let transfer_to_maker_ix=transfer(&spl_token_interface::ID,
        taker_ata_b.key, maker_ata_b.key,
        taker.key, &[], escrow.token_b_amount)?;
    invoke(&transfer_to_maker_ix,
        &[taker_ata_b.clone(),maker_ata_b.clone(),taker.clone()])?;

    //now transfer tokenA from vault ata to taker ata
    let transfer_to_taker_ix=transfer(&spl_token_interface::ID,
        escrow_vault_ata_a.key, taker_ata_a.key,
        escrow_vault_pda.key, &[escrow_vault_pda.key], escrow.token_a_amount)?;
    invoke_signed(&transfer_to_taker_ix,
        &[escrow_vault_ata_a.clone(),taker_ata_a.clone(),escrow_vault_pda.clone()],
        &[escrow_seeds])?;
    
    escrow.escrow_state=EscrowVaultState::COMPLETED;
    escrow.serialize(&mut *escrow_vault_pda.data.borrow_mut())?;

    **taker.lamports.borrow_mut()+=escrow_vault_pda.lamports();
    **escrow_vault_pda.lamports.borrow_mut()=0;
    let a=escrow_vault_pda.data;
    Ok(())
}

//dounts
//1. transfer and ata create ix have to be invoke or invoke signed and why
//2. what is signer pubkeys in transfer