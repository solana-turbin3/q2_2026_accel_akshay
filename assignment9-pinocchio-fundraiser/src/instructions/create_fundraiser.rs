use pinocchio::{AccountView, Address, ProgramResult, cpi::{Seed, Signer}, error::ProgramError, sysvars::{Sysvar, rent}};

use crate::state::FundraiserVault;

// data = target_amount(u64) + start_time(u64) + expiry_time(u64)
pub fn create_fundraiser(accounts:&mut[AccountView], ix_data:&[u8])->ProgramResult{
    let [
        maker,
        fundraiser_vault_pda,
        fundraiser_vault_ata,
        token_mint,
        system_prog,
        token_prog,
        ata_prog
    ]=accounts else{
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //check maker is signer
    if !maker.is_signer(){
        return Err(ProgramError::MissingRequiredSignature);
    }    
    //things to do
    //1. first create fundriser pda, if not exist. if exist ive error that t already exists
    //2. update the info of fundraiser pda
    //3. create ata account of fundriaaser vault pda where users will deposit funds

    //check if funraiser pda alreday exists
    if !fundraiser_vault_pda.is_data_empty(){
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let fundraiser_seed=&[b"fundraiser",maker.address().as_ref()];
    let (expected_fundraiser_pda,bump)=solana_pubkey::Pubkey::find_program_address(fundraiser_seed, &crate::ID);
    if expected_fundraiser_pda.as_ref()!=fundraiser_vault_pda.address().to_bytes(){
        return Err(ProgramError::InvalidSeeds);
    }

    let rent=rent::Rent::get()?;
    let bump_bytes=[bump];
    let seeds=[
        Seed::from(b"fundraiser"),
        Seed::from(maker.address().as_ref()),
        Seed::from(&bump_bytes)
    ];
    let signers=Signer::from(&seeds);
    // let x=[1u8;32];    //@q think about this
    // let x2=&x;
    // let y=maker.address().as_ref();
    // let z=b"a";
    // if x==y{

    // }
    pinocchio_system::instructions::CreateAccount{
        from:maker,
        to:fundraiser_vault_pda,
        space:FundraiserVault::FUNDRAISER_PDA_SIZE as u64,
        lamports:rent.minimum_balance_unchecked(FundraiserVault::FUNDRAISER_PDA_SIZE),
        owner:&crate::ID
    }.invoke_signed(&[signers])?;

    let target_amount=unsafe{*(ix_data.as_ptr().add(0) as *const u64)};
    let start_time=unsafe{*(ix_data.as_ptr().add(8) as *const u64)};    
    let expiry_time=unsafe{*(ix_data.as_ptr().add(16) as *const u64)};    

    let fundraiser_vault_data=unsafe{&mut *(fundraiser_vault_pda.borrow_unchecked_mut().as_mut_ptr() as *mut FundraiserVault)};
    fundraiser_vault_data.maker=maker.address().to_bytes();
    fundraiser_vault_data.token_mint=token_mint.address().to_bytes();

    fundraiser_vault_data.bump=bump;
    fundraiser_vault_data.start_time=start_time.to_le_bytes();
    fundraiser_vault_data.expiry_time=expiry_time.to_le_bytes();
    fundraiser_vault_data.target_amount=target_amount.to_le_bytes();
    fundraiser_vault_data.total_amount=0u64.to_le_bytes();

    //cretae ata of fundraiser vault pda
    pinocchio_associated_token_account::instructions::Create{
        funding_account:maker,
        account:fundraiser_vault_ata,
        mint:token_mint,
        wallet:fundraiser_vault_pda,
        system_program:system_prog,
        token_program:token_prog,
    }.invoke()?;
    Ok(())
}