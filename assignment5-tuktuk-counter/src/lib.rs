use solana_program::{account_info::{AccountInfo, next_account_info},
 entrypoint::{ProgramResult}, entrypoint, instruction::AccountMeta, msg, program::invoke_signed, pubkey::Pubkey, rent::Rent, sysvar::Sysvar};
use borsh::{BorshDeserialize,BorshSerialize};
use solana_system_interface::{instruction::create_account};

#[derive(BorshSerialize,BorshDeserialize)]
pub struct Counter{
    pub value:u32
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id:&Pubkey, accounts:&[AccountInfo],instruction_data:&[u8]
)->ProgramResult{

    let mut accounts_iter=accounts.iter();
    let user=next_account_info(&mut accounts_iter)?;
    let counter_pda=next_account_info(&mut accounts_iter)?;
    let system_prog=next_account_info(&mut accounts_iter)?;

    let counter_seeds=&[b"counter".as_ref()];
    let (expected_counter_pda,bump)=Pubkey::find_program_address(counter_seeds, program_id);
    let counter_seeds_with_bump=&[b"counter".as_ref(),&[bump]];

    // if counter_pda.data_is_empty(){
    if counter_pda.lamports()==0{
        let rent=Rent::get()?;
        let counter_rent=rent.minimum_balance(4);

        let create_counter_ix=create_account(user.key,
            counter_pda.key, counter_rent, 4, program_id);
        invoke_signed(&create_counter_ix,
            &[user.clone(), counter_pda.clone(),system_prog.clone()],
            &[counter_seeds_with_bump])?;
        msg!("counter pda created");
        let counter_data=Counter{value:0};
        counter_data.serialize(&mut *counter_pda.data.borrow_mut())?;
    }else{
        let mut counter_data=Counter::try_from_slice(&counter_pda.data.borrow())?;
        counter_data.value+=1;
        counter_data.serialize(&mut *counter_pda.data.borrow_mut())?;
    }
    Ok(())    
}