use pinocchio::{AccountView, Address, ProgramResult, cpi::{Seed, Signer}, error::ProgramError, sysvars::{self, Sysvar}};
use pinocchio_system::instructions::CreateAccount;

use crate::state::EscrowVaultPda;
use pinocchio_log::{log,logger};

//data= bump()+ amount_a(u64)+ amount_b(u64)+ expiry(u64)+ escrow_id(u64)
pub fn make_offer(accounts:&mut [AccountView],instruction_data:&[u8])->ProgramResult{
    let [
        maker,
        escrow_vault_pda,
        escrow_ata,
        maker_ata,
        mint_a,
        mint_b,
        system_prog,
        token_prog,
        ata_prog
    ]=accounts 
    else{
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    //maker signer check
    //first cerate vault pda ,store info it
    // then tranfer token a to vault pda

    let bump=instruction_data[0];


    let x=unsafe{*(instruction_data.as_ptr().add(1) as *mut u64)};
    
    let amount_a=unsafe{*(instruction_data.as_ptr().add(1) as *const u64)};
    let amount_b=unsafe{*(instruction_data.as_ptr().add(9) as *const u64)};

    let expiry=unsafe{*(instruction_data.as_ptr().add(17) as *const u64)};
    let escrow_id=unsafe{*(instruction_data.as_ptr().add(25) as *const u64)};

    let rent=sysvars::rent::Rent::get()?;
    let min_bal=rent.try_minimum_balance(EscrowVaultPda::ESCROW_VAULT_PDA_SIZE)?;

    let escrow_id_bytes=escrow_id.to_le_bytes();
    let bump_bytes=[bump];
    let seeds=[
        Seed::from(b"escrow"),
        Seed::from(escrow_id_bytes.as_ref()),
        Seed::from(&bump_bytes)
    ];
    let signers=Signer::from(&seeds);

    log!("a");
    CreateAccount{
        from:maker,
        to:escrow_vault_pda,
        space:EscrowVaultPda::ESCROW_VAULT_PDA_SIZE as u64,
        lamports:min_bal,
        owner:&Address::new_from_array(crate::ID)
    }.invoke_signed(&[signers])?;

    // log!("owner {}", escrow_vault_pda.owner());
    log!("{}",&crate::ID);
    log!("b");
    let escrow_vault_data=unsafe{&mut *(escrow_vault_pda.borrow_unchecked_mut().as_mut_ptr() as *mut EscrowVaultPda)};

    escrow_vault_data.mint_a.copy_from_slice(mint_a.address().as_ref());
    escrow_vault_data.mint_b.copy_from_slice(mint_b.address().as_ref());
    escrow_vault_data.token_a_amount=amount_a.to_le_bytes();
    escrow_vault_data.token_b_amount=amount_b.to_le_bytes();
    escrow_vault_data.vault_status=0;
    escrow_vault_data.expiry_time=expiry.to_le_bytes();
    log!("c");
    //create vault ata now
    let i=pinocchio_associated_token_account::instructions::Create{
        funding_account:maker,
        account:escrow_ata,
        wallet:escrow_vault_pda,
        mint:mint_a,
        system_program:system_prog,
        token_program:token_prog
    }.invoke()?;    //@q why this is invoke and not invoke_signed
    
    log!("d");
    pinocchio_token::instructions::Transfer::new(
        maker_ata,
        escrow_ata,
        maker,
        amount_a,
     ) .invoke()?;
    Ok(())
}