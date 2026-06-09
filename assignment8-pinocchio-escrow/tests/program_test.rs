use litesvm::LiteSVM;
use litesvm_token::{get_spl_account, spl_token::state::Account};
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::{Keypair, Signature, Signer};
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::{Pubkey};
use solana_address::{Address};
use solana_transaction::{Message, Transaction};

// #[test]
fn setup()->(LiteSVM,Keypair,Keypair,Keypair,Address,Address,u8){
    let mut svm=LiteSVM::new();
    let program_id=solana_pubkey::Pubkey::from_str_const("4vGPskg8Ku4jGnDY3s4kRiuK6zGct7tmiexbxqEd3BTY");
    let program_id2=solana_pubkey::Pubkey::new_from_array([42, 180, 5, 68, 243, 235, 189, 56, 197, 37, 17, 85, 205, 189, 100, 191, 64, 74, 171, 3, 37, 193, 199, 195, 213, 54, 156, 198, 228, 15, 248, 188]);
    println!("program id : {}, {:?}",program_id,program_id.to_bytes());
    println!("program id : {}, {:?}",program_id2,program_id2.to_bytes());
    
    svm.add_program_from_file(program_id, "./target/deploy/assignment8_pinocchio_escrow.so").unwrap();

    let user=Keypair::new();
    let maker=Keypair::new();
    let taker=Keypair::new();

    println!("user : {}. maker : {}, taker :{}",user.pubkey(), maker.pubkey(), taker.pubkey());
    svm.airdrop(&user.pubkey(), 1*LAMPORTS_PER_SOL).expect("Airdrop failed");
    svm.airdrop(&maker.pubkey(), 1*LAMPORTS_PER_SOL).expect("Airdrop failed");
    svm.airdrop(&taker.pubkey(), 1*LAMPORTS_PER_SOL).expect("Airdrop failed");

    let escrow_id=1u64;
    let (escrow_pda,bump)=Pubkey::find_program_address(&[b"escrow",escrow_id.to_le_bytes().as_ref()], &program_id);
    println!("escrow pda : {}",escrow_pda);
    return (svm,user,maker,taker, program_id,escrow_pda,bump);
}



fn create_mint_and_ata(
    svm:&mut LiteSVM,user:&Keypair, maker:&Keypair, taker:&Keypair, escrow_pda:&Address
)->(Address, Address, Address, Address,Address,Address,Address){
    
    //creating mint a and mint b
    let mint_a=litesvm_token::CreateMint::new(svm, user)
                            .authority(&user.pubkey()).decimals(5).send().unwrap();

    let mint_b=litesvm_token::CreateMint::new(svm, user).
                                authority(&user.pubkey()).decimals(4).send().unwrap();

    //cerating ata for maker and taker for mint a and mint b
    let maker_ata_a=litesvm_token::CreateAssociatedTokenAccount::new(svm, user, &mint_a).owner(&maker.pubkey()).send().unwrap();
    // let escrow_ata_a=litesvm_token::CreateAssociatedTokenAccount::new(svm, user, &mint_a).owner(escrow_pda).send().unwrap();
    let escrow_ata_a=spl_associated_token_account_interface::address::get_associated_token_address(escrow_pda, &mint_a);
    let taker_ata_a=litesvm_token::CreateAssociatedTokenAccount::new(svm, user, &mint_a).owner(&taker.pubkey()).send().unwrap();
    
    let maker_ata_b=litesvm_token::CreateAssociatedTokenAccount::new(svm, user, &mint_b).owner(&maker.pubkey()).send().unwrap();
    let taker_ata_b=litesvm_token::CreateAssociatedTokenAccount::new(svm, user, &mint_b).owner(&taker.pubkey()).send().unwrap();
    

    //minting to maker ata a and taker ata b
    litesvm_token::MintTo::new(svm, user, &mint_a, &maker_ata_a, 100).owner(user).send().unwrap();
    litesvm_token::MintTo::new(svm, user, &mint_b, &taker_ata_b, 150).owner(user).send().unwrap();

    return (mint_a,mint_b,maker_ata_a,escrow_ata_a,maker_ata_b,taker_ata_b,taker_ata_a);
}

#[test]
fn make_offer(){
    let (mut svm,user,maker,taker, program_id,escrow_pda,bump)=setup();

    let (mint_a,mint_b,maker_ata_a,escrow_ata_a,maker_ata_b,taker_ata_b,taker_ata_a)=create_mint_and_ata(&mut svm, &user,&maker, &taker,&escrow_pda);

    let mut v=vec![0u8,bump];
    
    let amount_a=u64::to_le_bytes(50);
    let amount_b=u64::to_le_bytes(60);
    v.extend_from_slice(&amount_a);
    v.extend_from_slice(&amount_b);

    let expiry=u64::to_le_bytes(1222);
    v.extend_from_slice(&expiry);

    let escrow_id=1u64;
    v.extend_from_slice(&escrow_id.to_le_bytes());

    let ix=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:maker.pubkey(), is_signer:true, is_writable:true},
            AccountMeta{pubkey:escrow_pda, is_signer:false, is_writable:true},

            AccountMeta{pubkey:escrow_ata_a, is_signer:false, is_writable:true},
            AccountMeta{pubkey:maker_ata_a, is_signer:false, is_writable:true},
            AccountMeta{pubkey:mint_a, is_signer:false, is_writable:false},
            AccountMeta{pubkey:mint_b, is_signer:false, is_writable:false},

            AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},
        ],
        data:v
    };

    // let message=Message::new_with_blockhash(&[ix], Some(&user.pubkey()), &svm.latest_blockhash());
    let tx=solana_transaction::Transaction::new_signed_with_payer(&[ix],Some(&user.pubkey()), &[user,maker],svm.latest_blockhash());

    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    println!("maker ata a data : {}",maker_ata_a_data.amount);
    
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("tx status : {:?}",tx_status.logs);

    let escrow_ata_data:Account=get_spl_account(&svm, &escrow_ata_a).unwrap();
    println!("escrow ata data : {}",escrow_ata_data.amount);
    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    println!("maker ata a data : {}",maker_ata_a_data.amount);
}






#[test]
fn take_offer(){
    let (mut svm,user,maker,taker, program_id,escrow_pda,bump)=setup();
    let (mint_a,mint_b,maker_ata_a,escrow_ata_a,maker_ata_b,taker_ata_b,taker_ata_a)=create_mint_and_ata(&mut svm, &user,&maker, &taker,&escrow_pda);

    let mut v=vec![0u8,bump];

    let amount_a=u64::to_le_bytes(50);
    let amount_b=u64::to_le_bytes(60);
    v.extend_from_slice(&amount_a);
    v.extend_from_slice(&amount_b);
    let expiry=u64::to_le_bytes(1222);
    v.extend_from_slice(&expiry);

    let escrow_id=1u64;
    v.extend_from_slice(&escrow_id.to_le_bytes());

    let ix=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:maker.pubkey(), is_signer:true, is_writable:true},
            AccountMeta{pubkey:escrow_pda, is_signer:false, is_writable:true},

            AccountMeta{pubkey:escrow_ata_a, is_signer:false, is_writable:true},
            AccountMeta{pubkey:maker_ata_a, is_signer:false, is_writable:true},
            AccountMeta{pubkey:mint_a, is_signer:false, is_writable:false},
            AccountMeta{pubkey:mint_b, is_signer:false, is_writable:false},

            AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},
        ],
        data:v
    };

    let tx=solana_transaction::Transaction::new_signed_with_payer(&[ix],Some(&user.pubkey()), &[user,maker],svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx).unwrap();
    println!("tx status : {:?}",tx_status.logs);

    let escrow_ata_data:Account=get_spl_account(&svm, &escrow_ata_a).unwrap();
    println!("escrow ata data : {}",escrow_ata_data.amount);
    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    println!("maker ata a data : {}",maker_ata_a_data.amount);


    let mut v2=vec![1u8,bump];
    let escrow_id=1u64;
    v2.extend_from_slice(&escrow_id.to_le_bytes());
    let ix2=Instruction{
        program_id,
        accounts:vec![
            AccountMeta{pubkey:taker.pubkey(), is_signer:true, is_writable:true},
            AccountMeta{pubkey:escrow_pda, is_signer:false, is_writable:true},

            AccountMeta{pubkey:taker_ata_b, is_signer:false, is_writable:true},
            AccountMeta{pubkey:maker_ata_b, is_signer:false, is_writable:true},
            AccountMeta{pubkey:escrow_ata_a, is_signer:false, is_writable:true},
            AccountMeta{pubkey:taker_ata_a, is_signer:false, is_writable:true},
            // AccountMeta{pubkey:mint_a, is_signer:false, is_writable:false},
            // AccountMeta{pubkey:mint_b, is_signer:false, is_writable:false},

            // AccountMeta{pubkey:pinocchio_system::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_token::ID, is_signer:false, is_writable:false},
            AccountMeta{pubkey:pinocchio_associated_token_account::ID, is_signer:false, is_writable:false},

        ],
        data:v2
    };
    let tx2=Transaction::new_signed_with_payer(&[ix2], Some(&taker.pubkey()), &[taker], svm.latest_blockhash());
    let tx_status=svm.send_transaction(tx2).unwrap();
    println!("take offer tx_status : {:?}",tx_status.logs);


    println!("after taking offer");
    let escrow_ata_data:Account=get_spl_account(&svm, &escrow_ata_a).unwrap();
    println!("escrow ata data : {}",escrow_ata_data.amount);
    let maker_ata_a_data:Account=get_spl_account(&svm, &maker_ata_a).unwrap();
    println!("maker ata a data : {}",maker_ata_a_data.amount);
    let maker_ata_b_data:Account=get_spl_account(&svm, &maker_ata_b).unwrap();
    println!("maker ata b data : {}",maker_ata_b_data.amount);

    let taker_ata_a_data:Account=get_spl_account(&svm, &taker_ata_a).unwrap();
    println!("taker ata a data : {}",taker_ata_a_data.amount);
    let taker_ata_b_data:Account=get_spl_account(&svm, &taker_ata_b).unwrap();
    println!("taker ata b data : {}",taker_ata_b_data.amount);

}