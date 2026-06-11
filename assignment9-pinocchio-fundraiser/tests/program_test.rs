use std::str::FromStr;

use litesvm::LiteSVM;
use litesvm_token::spl_token::state::Account;
use pinocchio::{Address, sysvars::{clock::Clock,Sysvar}};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair,Signer,Signature};
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::{Pubkey};
use solana_transaction::Transaction;
// use solana_signer::Signer;

fn setup()->(LiteSVM, Keypair, Keypair, Address, Address, Address){
    let mut svm=LiteSVM::new();

    let maker=Keypair::new();
    let contributer=Keypair::new();
    let program_id=Address::from_str("8XguEtT1GjWtfZU3Tvjhnxmrk7owYhMcVE2WBUfXgq8h").unwrap();
    svm.add_program_from_file(program_id, "./target/deploy/assignment9_pinocchio_fundraiser.so").unwrap();

    svm.airdrop(&maker.pubkey(), 1*LAMPORTS_PER_SOL).unwrap();
    svm.airdrop(&contributer.pubkey(), 1*LAMPORTS_PER_SOL).unwrap();

    let maker_bytes=maker.pubkey().to_bytes();
    let fundraiser_seeds=&[b"fundraiser".as_ref(),maker_bytes.as_ref()];
    println!("maker : {}",maker.pubkey());
    let (fundraiser_pda,bump)=Pubkey::find_program_address(fundraiser_seeds, &program_id);
    
    let contributor_bytes=contributer.pubkey().to_bytes();
    let contributor_seeds=&[b"contributor",contributor_bytes.as_ref()];
    let (contributor_pda,_)=Pubkey::find_program_address(contributor_seeds, &program_id);
    
    println!("maker : {}, fundraise pda : {}, contributor_pda : {}",maker.pubkey(),fundraiser_pda,contributor_pda);
    return (svm,maker,contributer,program_id,fundraiser_pda,contributor_pda);
}

#[test]
fn create_fundraise(){
    let (mut svm, maker,contributor,program_id,fundraise_pda,contributor_pda)=setup();

    let token_mint=litesvm_token::CreateMint::new(&mut svm, &maker).authority(&maker.pubkey()).decimals(5).send().unwrap();
    let fundraise_pda_ata=spl_associated_token_account_interface::address::get_associated_token_address(&fundraise_pda, &token_mint);

    // let mut v=vec![0u8,bump];
    let mut v=vec![0u8];
    let target_amount=200u64;
    let start_time=100u64;
    let expiry_time=120u64;
    v.extend_from_slice(target_amount.to_le_bytes().as_ref());
    v.extend_from_slice(&start_time.to_le_bytes());
    v.extend_from_slice(&expiry_time.to_le_bytes());

    let ix=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:maker.pubkey(), is_signer:true, is_writable:true},  //signer needs to be writable
            AccountMeta{pubkey:fundraise_pda, is_signer:false, is_writable:true},
            AccountMeta{pubkey:fundraise_pda_ata, is_signer:false, is_writable:true},
            AccountMeta{pubkey:token_mint, is_signer:false, is_writable:false},

            AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},
        ],
        data:v
    };
    let tx=Transaction::new_signed_with_payer(&[ix], Some(&maker.pubkey()), &[maker], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("tx_status : {:?}",tx_status);

    let fundraise_pda_data=svm.get_account(&fundraise_pda).unwrap();
    println!("fundraise pda data : {:?}",fundraise_pda_data.data);
}


#[test]
fn test_contribute(){
    let (mut svm, maker,contributor,program_id,fundraise_pda,contributor_pda)=setup();

    let token_mint=litesvm_token::CreateMint::new(&mut svm, &maker).authority(&maker.pubkey()).decimals(5).send().unwrap();
    //create contributor ata and mint tokens for contribution
    let contributor_ata=litesvm_token::CreateAssociatedTokenAccount::new(&mut svm, &contributor, &token_mint).owner(&contributor.pubkey()).send().unwrap();
    litesvm_token::MintTo::new(&mut svm, &contributor, &token_mint, &contributor_ata, 200u64).owner(&maker).send().unwrap();

    let fundraise_pda_ata=spl_associated_token_account_interface::address::get_associated_token_address(&fundraise_pda, &token_mint);

    // let mut v=vec![0u8,bump];
    let mut v=vec![0u8];
    let target_amount=200u64;
    let start_time=100u64;
    let expiry_time=120u64;
    v.extend_from_slice(target_amount.to_le_bytes().as_ref());
    v.extend_from_slice(&start_time.to_le_bytes());
    v.extend_from_slice(&expiry_time.to_le_bytes());

    let ix=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:maker.pubkey(), is_signer:true, is_writable:true},  //signer needs to be writable
            AccountMeta{pubkey:fundraise_pda, is_signer:false, is_writable:true},
            AccountMeta{pubkey:fundraise_pda_ata, is_signer:false, is_writable:true},
            AccountMeta{pubkey:token_mint, is_signer:false, is_writable:false},

            AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},
        ],
        data:v
    };
    let tx=Transaction::new_signed_with_payer(&[ix], Some(&maker.pubkey()), &[&maker], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("tx_status : {:?}",tx_status);


    let mut v2=vec![1u8];
    let contribute_amount=20u64.to_le_bytes();
    v2.extend_from_slice(&contribute_amount);

    println!("fundraise pda ata : {}",fundraise_pda_ata);
    let ix2=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:contributor.pubkey(), is_signer:true, is_writable:true},
            AccountMeta{pubkey:maker.pubkey(), is_signer:false, is_writable:false},
            AccountMeta{pubkey:fundraise_pda, is_signer:false, is_writable:true},
            AccountMeta{pubkey:contributor_pda, is_signer:false, is_writable:true},

            AccountMeta{pubkey:contributor_ata, is_signer:false, is_writable:true}, 
            AccountMeta{pubkey:fundraise_pda_ata, is_signer:false, is_writable:true}, 

            AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},
        ],
        data:v2
    };
    let tx=Transaction::new_signed_with_payer(&[ix2], Some(&contributor.pubkey()), &[&contributor], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("contribute tx_status : {:?}",tx_status);

    let fundraise_ata_data:Account=litesvm_token::get_spl_account(&svm, &fundraise_pda_ata).unwrap();
    println!("fundriase pda ata balance : {}",fundraise_ata_data.amount);

    let contributor_ata_data:Account=litesvm_token::get_spl_account(&svm, &contributor_ata).unwrap();
    println!("contributor ata balance : {}",contributor_ata_data.amount);
}


#[test]
fn test_refund(){
    let (mut svm, maker,contributor,program_id,fundraise_pda,contributor_pda)=setup();

    let token_mint=litesvm_token::CreateMint::new(&mut svm, &maker).authority(&maker.pubkey()).decimals(5).send().unwrap();
    //create contributor ata and mint tokens for contribution
    let contributor_ata=litesvm_token::CreateAssociatedTokenAccount::new(&mut svm, &contributor, &token_mint).owner(&contributor.pubkey()).send().unwrap();
    litesvm_token::MintTo::new(&mut svm, &contributor, &token_mint, &contributor_ata, 200u64).owner(&maker).send().unwrap();

    let fundraise_pda_ata=spl_associated_token_account_interface::address::get_associated_token_address(&fundraise_pda, &token_mint);

    // let mut v=vec![0u8,bump];
    let mut v=vec![0u8];
    let target_amount=200u64;
    let start_time=100u64;
    let expiry_time=120u64;
    v.extend_from_slice(target_amount.to_le_bytes().as_ref());
    v.extend_from_slice(&start_time.to_le_bytes());
    v.extend_from_slice(&expiry_time.to_le_bytes());

    let ix=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:maker.pubkey(), is_signer:true, is_writable:true},  //signer needs to be writable
            AccountMeta{pubkey:fundraise_pda, is_signer:false, is_writable:true},
            AccountMeta{pubkey:fundraise_pda_ata, is_signer:false, is_writable:true},
            AccountMeta{pubkey:token_mint, is_signer:false, is_writable:false},

            AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},
        ],
        data:v
    };
    let tx=Transaction::new_signed_with_payer(&[ix], Some(&maker.pubkey()), &[&maker], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("tx_status : {:?}",tx_status);


    let mut v2=vec![1u8];
    let contribute_amount=20u64.to_le_bytes();
    v2.extend_from_slice(&contribute_amount);

    println!("fundraise pda ata : {}",fundraise_pda_ata);
    let ix2=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:contributor.pubkey(), is_signer:true, is_writable:true},
            AccountMeta{pubkey:maker.pubkey(), is_signer:false, is_writable:false},
            AccountMeta{pubkey:fundraise_pda, is_signer:false, is_writable:true},
            AccountMeta{pubkey:contributor_pda, is_signer:false, is_writable:true},

            AccountMeta{pubkey:contributor_ata, is_signer:false, is_writable:true}, 
            AccountMeta{pubkey:fundraise_pda_ata, is_signer:false, is_writable:true}, 

            AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},
        ],
        data:v2
    };
    let tx=Transaction::new_signed_with_payer(&[ix2], Some(&contributor.pubkey()), &[&contributor], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("contribute tx_status : {:?}",tx_status);

    let fundraise_ata_data:Account=litesvm_token::get_spl_account(&svm, &fundraise_pda_ata).unwrap();
    println!("fundriase pda ata balance : {}",fundraise_ata_data.amount);

    let contributor_ata_data:Account=litesvm_token::get_spl_account(&svm, &contributor_ata).unwrap();
    println!("contributor ata balance : {}",contributor_ata_data.amount);


    let clock:solana_sdk::sysvar::clock::Clock=svm.get_sysvar();
    println!("old time: {}, old slot : {}",clock.unix_timestamp, clock.slot);

    // Warp forward 1 hour (3600 seconds)
    let target_slot = clock.slot + (3600.0 / 0.4) as u64; // ~0.4s per slot
    svm.warp_to_slot(target_slot);
    // Verify timestamp increased
    let new_clock: solana_sdk::sysvar::clock::Clock = svm.get_sysvar();
    
    println!("time : {} new time: {}, new slot : {}",clock.unix_timestamp, new_clock.unix_timestamp, new_clock.slot);
    assert!(new_clock.unix_timestamp >= clock.unix_timestamp + 3600);

}