use pinocchio::{AccountView, ProgramResult, cpi::{Seed, Signer}, error::ProgramError, sysvars::Sysvar};
use pinocchio_log::log;
use pinocchio_token::instructions::Transfer;

use crate::state::EscrowVaultPda;


//data = bump(u8)+ escrow_id(u64)
pub fn take_offer(accounts:&mut [AccountView], instruction_data:&[u8])->ProgramResult{
    let [
        taker,
        escrow_vault_pda,
        taker_ata_b,
        maker_ata_b,
        escrow_ata_a,
        taker_ata_a,
        token_prog,
        ata_prog
    ]=accounts else{
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //check taker is signer check
    //check correct escrow vault pda
    //check escrow is still valid

    log!("a");
    if !taker.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
    }

    let bump=instruction_data[0];
    let escrow_id=unsafe{*(instruction_data.as_ptr().add(1) as *const u64)};
    let escrow_id_bytes=escrow_id.to_le_bytes();
    let seeds=&[b"escrow".as_ref(),escrow_id_bytes.as_ref()];

    let expected_escrow_pda=pinocchio_pubkey::derive_address(seeds, Some(bump), &crate::ID);
    
    if escrow_vault_pda.address().to_bytes()!=expected_escrow_pda{
        return Err(ProgramError::MaxSeedLengthExceeded);
    }
    let curr_time=pinocchio::sysvars::clock::Clock::get()?.unix_timestamp as u64;
    log!("b");
    let escrow_vault_data=unsafe{&mut *(escrow_vault_pda.borrow_unchecked_mut().as_mut_ptr() as *mut EscrowVaultPda)};
    let expiry_time=u64::from_be_bytes(escrow_vault_data.expiry_time);
    let amount_a=u64::from_le_bytes(escrow_vault_data.token_a_amount);
    let amount_b=u64::from_le_bytes(escrow_vault_data.token_b_amount);
  
    if escrow_vault_data.vault_status!=0 || expiry_time<curr_time{
        return Err(ProgramError::InvalidArgument);
    }
    log!("c");
    //first taker tranfer token b to maker ata b
    //then update escrow vault info
    // transfer then tokena a from escrow vault ata to taker ata

    pinocchio_token::instructions::Transfer::new(
        taker_ata_b, maker_ata_b, taker, amount_b
    ).invoke()?;

    log!("d");
    let bump_bytes=[bump];
    let seeds=[
        Seed::from(b"escrow"),
        Seed::from(escrow_id_bytes.as_ref()),
        Seed::from(&bump_bytes)
    ];
    let signers=Signer::from(&seeds);
    pinocchio_token::instructions::Transfer::new(
        escrow_ata_a, taker_ata_a, escrow_vault_pda, amount_a
    ).invoke_signed(&[signers])?;

    log!("e");
    escrow_vault_data.vault_status=1;

    //@q close this escrow vault account now, not needed
    Ok(())
}