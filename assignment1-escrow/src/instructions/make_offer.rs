use solana_program::{
    pubkey::Pubkey, account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, program_error::ProgramError,
    rent::Rent, sysvar::Sysvar,msg,
    program::{invoke,invoke_signed}
};
use solana_system_interface;

use spl_associated_token_account_interface::{instruction::create_associated_token_account};
use spl_token_interface::{instruction::transfer};

use crate::state::{EscrowVault, EscrowVaultState};
use borsh::{BorshSerialize};

pub fn make_offer(program_id:&Pubkey, accounts:&[AccountInfo], token_a_amount:u64, token_b_amount:u64, expiry_time:u64, escrow_id:u64, escrow_bump:u8)->ProgramResult{
    let accounts_iter=&mut accounts.iter();

    let maker=next_account_info(accounts_iter)?;
    let escrow_vault_pda=next_account_info(accounts_iter)?;

    let maker_ata_a=next_account_info(accounts_iter)?;
    let escrow_vault_ata_a=next_account_info(accounts_iter)?;
    let mint_a=next_account_info(accounts_iter)?;
    let mint_b=next_account_info(accounts_iter)?;
    
    let token_prog=next_account_info(accounts_iter)?;
    let ata_prog=next_account_info(accounts_iter)?;
    let system_prog=next_account_info(accounts_iter)?;
    
    if !maker.is_signer{
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !escrow_vault_pda.data_is_empty(){
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    if *system_prog.key!=solana_system_interface::program::ID{
        return Err(ProgramError::IncorrectProgramId);
    }

    let escrow_id_bytes=escrow_id.to_le_bytes();
    let escrow_seeds=&[b"escrow_vault",escrow_id_bytes.as_ref(), &[escrow_bump]];
    
    msg!("escrow pda : {}, escrow bump : {}",escrow_vault_pda.key, escrow_bump);
    let expected_escrow_pda=Pubkey::create_program_address(escrow_seeds, program_id)?;
    msg!("escrow pda : {}, expected escrow pda : {}",escrow_vault_pda.key, expected_escrow_pda);
    if *escrow_vault_pda.key!=expected_escrow_pda{
        return Err(ProgramError::InvalidSeeds);
    }
    let rent=Rent::get()?;
    let escrow_pda_min_rent=rent.minimum_balance(EscrowVault::ESCROW_VAULT_SIZE);

    let escrow_pda_create_ix=solana_system_interface::instruction::create_account(
        maker.key, escrow_vault_pda.key,
        escrow_pda_min_rent, EscrowVault::ESCROW_VAULT_SIZE as u64, program_id);
    invoke_signed(&escrow_pda_create_ix,
        &[maker.clone(), escrow_vault_pda.clone(),system_prog.clone()], &[escrow_seeds])?;
    msg!("escrow pda created!!");

    let escrow=EscrowVault{
        escrow_state:EscrowVaultState::PENDING,
        token_a_amount,
        token_b_amount,
        expiry_time,
        mint_a:*mint_a.key,
        mint_b:*mint_b.key
    };
    escrow.serialize(&mut *escrow_vault_pda.data.borrow_mut())?;

    //now maker transfers x tokenA to escrow vault ataA
    // first create ata of escrow vault for tokenA
    let create_escrow_vault_ata_a_ix=create_associated_token_account(maker.key,
        escrow_vault_pda.key, mint_a.key, &spl_token_interface::ID);
    invoke(&create_escrow_vault_ata_a_ix,
        &[maker.clone(), escrow_vault_ata_a.clone(), escrow_vault_pda.clone(),
        mint_a.clone(), system_prog.clone(), token_prog.clone(),ata_prog.clone()])?;
    msg!("escrow vault ata a created!!");

    let transfer_ix=transfer(&spl_token_interface::ID,
        maker_ata_a.key, escrow_vault_ata_a.key, maker.key,
        &[maker.key], token_a_amount)?;
    invoke(&transfer_ix, &[maker_ata_a.clone(), escrow_vault_ata_a.clone(), maker.clone()])?;

    Ok(())
}

//dounts
//1. transfer and ata create ix have to be invoke or invoke signed and why
//2. what is signer pubkeys in transfer