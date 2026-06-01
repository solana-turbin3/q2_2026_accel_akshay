use std::{ str::FromStr};

use escrow::state::EscrowVault;
use litesvm::LiteSVM;
use litesvm_token::{
    CreateAssociatedTokenAccount, CreateMint, MintTo, get_spl_account, spl_token::state::Account
};
use solana_sdk::{
    clock::Clock,sysvar::Sysvar,
    message::{AccountMeta, Instruction}, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{Keypair,Signer}, transaction::Transaction
};
use borsh::{BorshDeserialize,BorshSerialize};
use solana_system_interface;
use spl_token_interface;
use spl_associated_token_account_interface::{self, address::get_associated_token_address};

fn setup()-> (LiteSVM, Pubkey, Keypair, Pubkey, u8){
    let mut svm=LiteSVM::new();
    let user=Keypair::new();
    let escrow_prog=Pubkey::from_str("CnjnrEyoBbcXD4jNzhjWoVn3cLjFzLtKSk5aDv2vH41D").unwrap();

    let escrow_id=1u64;
    let escrow_id_bytes=escrow_id.to_le_bytes();
    let escrow_seeds=&[b"escrow_vault",escrow_id_bytes.as_ref()];
    let (escrow_vault_pda,escrow_bump)=Pubkey::find_program_address(escrow_seeds, &escrow_prog);

    println!("escrow vault pda : {}, escrow vault bump :{}",escrow_vault_pda,escrow_bump);
    
    let escrow_seedss=&[b"escrow_vault",escrow_id_bytes.as_ref(), &[escrow_bump]];
    let expected_escrow_pda=Pubkey::create_program_address(escrow_seedss, &escrow_prog).unwrap();
    println!("expected escrow vault pda : {}",expected_escrow_pda);

    svm.airdrop(&user.pubkey(), LAMPORTS_PER_SOL).unwrap();
    // svm.add_program_from_file(escrow_prog, "../target/deploy/escrow.so").unwrap();
    svm.add_program_from_file(escrow_prog, "/home/akshay/wslProjects/solana-revise/escrow/target/deploy/escrow.so").unwrap();
    (svm, escrow_prog,user, escrow_vault_pda,escrow_bump)
}

#[derive(BorshSerialize,BorshDeserialize)]
pub struct MakeOfferInput{
    pub variant:u8,
    pub token_a_amount:u64,
    pub token_b_amount:u64,
    pub expiry_time:u64,
    pub escrow_id:u64,
    pub escrow_bump:u8
}

#[test]
fn test_make_offer(){
    let(mut svm, escrow_prog,user,escrow_vault_pda,escrow_bump)=setup();

    let maker=Keypair::new();
    let taker=Keypair::new();
    
    let mint_a=CreateMint::new(&mut svm, &user).authority(&user.pubkey()).decimals(5).send().unwrap();
    let mint_b=CreateMint::new(&mut svm, &user).authority(&user.pubkey()).decimals(5).send().unwrap();

    let maker_ata_a=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_a).owner(&maker.pubkey()).send().unwrap();
    let taker_ata_a=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_a).owner(&taker.pubkey()).send().unwrap();
   
    // let escrow_vault_ata_a=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_a).owner(&escrow_vault_pda).send().unwrap();
    let escrow_vault_ata_a=get_associated_token_address(&escrow_vault_pda, &mint_a);

    let maker_ata_b=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_b).owner(&maker.pubkey()).send().unwrap();
    let taker_ata_b=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_b).owner(&taker.pubkey()).send().unwrap();

    MintTo::new(&mut svm, &user, &mint_a, &maker_ata_a, 1000).owner(&user).send().unwrap();
    MintTo::new(&mut svm, &user, &mint_b, &taker_ata_b, 1000).owner(&user).send().unwrap();
    
    let maker_acc:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    let taker_acc:Account=get_spl_account(&svm, &taker_ata_b).unwrap();
    println!("maker ata a : {}, taker ata b : {}",maker_acc.amount,taker_acc.amount);
    
    let escrow_offer=MakeOfferInput{variant:0,token_a_amount:100, token_b_amount:200, escrow_id:1, expiry_time:10200, escrow_bump};
    let serialised_escrow_offer=borsh::to_vec(&escrow_offer).unwrap();

    svm.airdrop(&maker.pubkey(), LAMPORTS_PER_SOL).unwrap();

    let ix=Instruction{
        program_id:escrow_prog,
        accounts:vec![
            // {AccountMeta::new_readonly(user.pubkey(), true)},
            {AccountMeta::new_readonly(maker.pubkey(), true)},
            {AccountMeta::new(escrow_vault_pda, false)},

            {AccountMeta::new(maker_ata_a, false)},
            {AccountMeta::new(escrow_vault_ata_a, false)},
            {AccountMeta::new_readonly(mint_a, false)},
            {AccountMeta::new_readonly(mint_b, false)},

            {AccountMeta::new_readonly(spl_token_interface::ID, false)},
            {AccountMeta::new_readonly(spl_associated_token_account_interface::program::ID, false)},
            {AccountMeta::new_readonly(solana_system_interface::program::ID, false)},
        ],
        data:serialised_escrow_offer
    };
    let tx=Transaction::new_signed_with_payer(&[ix],
        // Some(&user.pubkey()), &[&user], svm.latest_blockhash());
        Some(&maker.pubkey()), &[&maker], svm.latest_blockhash());
    let result=svm.send_transaction(tx).unwrap();
    println!("tx status : {:?}",result.logs);

    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    let maker_ata_b_data:Account=get_spl_account(&svm, &maker_ata_b).unwrap();
    let taker_ata_a_data:Account=get_spl_account(&svm, &taker_ata_a).unwrap();
    let taker_ata_b_data:Account=get_spl_account(&svm, &taker_ata_b).unwrap();
    let escrow_ata_a_data:Account=get_spl_account(&svm, &escrow_vault_ata_a).unwrap();

    println!("maker ata a : {}, maker ata b : {}",maker_ata_a_data.amount,maker_ata_b_data.amount);
    println!("taker ata a : {}, taker ata b : {}",taker_ata_a_data.amount,taker_ata_b_data.amount);
    println!("escrow vault ata a : {} ",escrow_ata_a_data.amount);
}

#[derive(BorshSerialize,BorshDeserialize)]
pub struct TakeOfferIxInput{
    pub variant:u8,
    pub escrow_id:u64,
    pub escrow_bump:u8
}

#[test]
fn test_take_offer(){
    let(mut svm, escrow_prog,user,escrow_vault_pda,escrow_bump)=setup();

    let time:Clock=svm.get_sysvar();
    println!("time: {} , epoch : {}, slot:{}",time.unix_timestamp,time.epoch, time.slot);
    svm.warp_to_slot(time.slot+100);

    
    let maker=Keypair::new();
    let taker=Keypair::new();

    svm.airdrop(&maker.pubkey(), LAMPORTS_PER_SOL).unwrap();
    svm.airdrop(&taker.pubkey(), LAMPORTS_PER_SOL).unwrap();

    let mint_a=CreateMint::new(&mut svm, &user).authority(&user.pubkey()).decimals(5).send().unwrap();
    let mint_b=CreateMint::new(&mut svm, &user).authority(&user.pubkey()).decimals(5).send().unwrap();

    let maker_ata_a=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_a).owner(&maker.pubkey()).send().unwrap();
    let taker_ata_a=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_a).owner(&taker.pubkey()).send().unwrap();
   
    let maker_ata_b=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_b).owner(&maker.pubkey()).send().unwrap();
    let taker_ata_b=CreateAssociatedTokenAccount::new(&mut svm, &user, &mint_b).owner(&taker.pubkey()).send().unwrap();

    let escrow_vault_ata_a=get_associated_token_address(&escrow_vault_pda, &mint_a);
    MintTo::new(&mut svm, &user, &mint_a, &maker_ata_a, 1000).owner(&user).send().unwrap();
    MintTo::new(&mut svm, &user, &mint_b, &taker_ata_b, 1000).owner(&user).send().unwrap();


    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    let taker_ata_b_data:Account=get_spl_account(&svm, &taker_ata_b).unwrap();
    println!("maker ata a : {}, taker ata b : {}",maker_ata_a_data.amount,taker_ata_b_data.amount);

    //now making offer first
    let make_offer_ix_input=MakeOfferInput{variant:0,token_a_amount:100,token_b_amount:150,
        escrow_id:1,expiry_time:10200,escrow_bump};
    let serrialised_make_offer_ix_input=borsh::to_vec(&make_offer_ix_input).unwrap();
    
    let ix0=Instruction{
        program_id:escrow_prog,
        accounts:vec![
            {AccountMeta::new(maker.pubkey(), true)},
            {AccountMeta::new(escrow_vault_pda, false)},
            {AccountMeta::new(maker_ata_a, false)},

            {AccountMeta::new(escrow_vault_ata_a, false)},
            {AccountMeta::new_readonly(mint_a, false)},
            {AccountMeta::new_readonly(mint_b, false)},

            {AccountMeta::new_readonly(spl_token_interface::ID, false)},
            {AccountMeta::new_readonly(spl_associated_token_account_interface::program::ID, false)},
            {AccountMeta::new_readonly(solana_system_interface::program::ID, false)},
        ],
        data:serrialised_make_offer_ix_input
    };
    let tx0=Transaction::new_signed_with_payer(&[ix0],
        Some(&user.pubkey()), &[&user,&maker], svm.latest_blockhash());
    let tx0_status=svm.send_transaction(tx0).unwrap();
    println!("tx0_status : {:?}",tx0_status.logs);

    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    let maker_ata_b_data:Account=get_spl_account(&svm, &maker_ata_b).unwrap();

    let taker_ata_a_data:Account=get_spl_account(&svm, &taker_ata_a).unwrap();
    let taker_ata_b_data:Account=get_spl_account(&svm, &taker_ata_b).unwrap();
    let escrow_ata_a_data:Account=get_spl_account(&svm, &escrow_vault_ata_a).unwrap();
    
    println!("after making offer : ");
    println!("maker ata a : {}, maker ata b : {}",maker_ata_a_data.amount,maker_ata_b_data.amount);
    println!("taker ata a : {}, taker ata b : {}`",taker_ata_a_data.amount,taker_ata_b_data.amount);
    println!("escrow vault ata a : {}",escrow_ata_a_data.amount);

    let escrow_vault_pda_data=svm.get_account(&escrow_vault_pda).unwrap();
    let deserialised_escrow_vault_data=EscrowVault::try_from_slice(&escrow_vault_pda_data.data).unwrap();
    println!("escrow vault pda data : {:?}\n",deserialised_escrow_vault_data);

    //now taking offer
    let take_offer_ix_input=TakeOfferIxInput{variant:1, escrow_bump, escrow_id:1};
    let serialised_take_offer_ix_input=borsh::to_vec(&take_offer_ix_input).unwrap();
    let ix=Instruction{
        program_id:escrow_prog,
        accounts:vec![
            AccountMeta::new_readonly(taker.pubkey(), true),
            AccountMeta::new(escrow_vault_pda, false),
            AccountMeta::new(taker_ata_a, false),

            AccountMeta::new(taker_ata_b, false),  //qtry them as non writable
            AccountMeta::new(maker_ata_b, false),  //qtry them as non writable
            AccountMeta::new(escrow_vault_ata_a, false),  

            AccountMeta::new_readonly(spl_token_interface::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account_interface::program::ID, false),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data:serialised_take_offer_ix_input
    };
    let tx=Transaction::new_signed_with_payer(&[ix],
        Some(&taker.pubkey()), &[&taker], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("tx_status : {:?}",tx_status.logs);
    
    println!("after taking offer : ");
    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    let maker_ata_b_data:Account=get_spl_account(&svm, &maker_ata_b).unwrap();
    let taker_ata_a_data:Account=get_spl_account(&svm, &taker_ata_a).unwrap();
    let taker_ata_b_data:Account=get_spl_account(&svm, &taker_ata_b).unwrap();
    let escrow_ata_a_data:Account=get_spl_account(&svm, &escrow_vault_ata_a).unwrap();
    
    println!("maker ata a : {}, maker ata b : {}",maker_ata_a_data.amount,maker_ata_b_data.amount);
    println!("taker ata a : {}, taker ata b : {}",taker_ata_a_data.amount,taker_ata_b_data.amount);
    println!("escrow vault ata a : {}",escrow_ata_a_data.amount);

    

    let escrow_vault_pda_data=svm.get_account(&escrow_vault_pda).unwrap();
    let deserialised_escrow_vault_data=EscrowVault::try_from_slice(&escrow_vault_pda_data.data).unwrap();
    println!("escrow vault pda data : {:?}\n",deserialised_escrow_vault_data);

    let time1:Clock=svm.get_sysvar();
    println!("time: {} , epoch : {}, slot:{}",time1.unix_timestamp,time1.epoch, time1.slot);
}

//dounts