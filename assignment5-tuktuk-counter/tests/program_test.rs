use std::str::FromStr;

use litesvm::LiteSVM;
use solana_sdk::{message::{AccountMeta, Instruction}, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
#[test]
fn create_counter(){
    let mut svm=LiteSVM::new();

    let program_id=Pubkey::from_str("8cSbvaEfqayChL254447rPKaq66ynGXNgZZZ3SXYfrjp").unwrap();
    svm.add_program_from_file(program_id, "/home/akshay/wslProjects/q2_2026_accel_akshay/tuktuk/target/deploy/tuktuk.so").unwrap();
   
    let user=Keypair::new();
    svm.airdrop(&user.pubkey(), LAMPORTS_PER_SOL).unwrap();


    let counter_seeds=&[b"counter".as_ref()];
    let (counter_pda,bump)=Pubkey::find_program_address(counter_seeds, &program_id);

    let ix=Instruction{
        program_id,
        accounts:vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data:vec![]
    };
    let tx=Transaction::new_signed_with_payer(&[ix],
        Some(&user.pubkey()), &[&user], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("result status : {:?}",tx_status.logs);

    let counter_data=svm.get_account(&counter_pda).unwrap();
    println!("counter data : {:?}",counter_data.data);


    svm.expire_blockhash();
    let ix1=Instruction{
        program_id,
        accounts:vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
        ],
        data:vec![]
    };
    let tx1=Transaction::new_signed_with_payer(&[ix1],
        Some(&user.pubkey()), &[&user], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx1).unwrap();
    println!("result status : {:?}",tx_status.logs);


    let counter_data=svm.get_account(&counter_pda).unwrap();
    println!("counter data : {:?}",counter_data.data);
}