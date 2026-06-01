use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey
};
use borsh::{BorshDeserialize,BorshSerialize};
use crate::instruction::InstructionType;
use crate::instructions::{
    make_offer::make_offer,
    take_offer::take_offer,
    cancel_offer::cancel_offer
};

pub fn process_instruction(
    program_id:&Pubkey, accounts:&[AccountInfo],instruction_data:&[u8]
)->ProgramResult{
    let ix=InstructionType::try_from_slice(instruction_data)
        .map_err(|_|ProgramError::InvalidInstructionData)?;
    match ix{
        InstructionType::MakeOffer { token_a_amount, token_b_amount, expiry_time, escrow_id, escrow_bump }=>{
            msg!("make offer ix called");
            make_offer(program_id, accounts, token_a_amount, token_b_amount, expiry_time, escrow_id, escrow_bump)?;
        },
        InstructionType::TakeOffer { escrow_id, escrow_bump }=>{
            msg!("take offer ix called");
            take_offer(program_id, accounts, escrow_id, escrow_bump)?;
        },
        InstructionType::CancelOffer { escrow_id, escrow_bump }=>{
            msg!("cancel offer ix called");
            cancel_offer(program_id, accounts, escrow_id, escrow_bump)?;
        }
    }
    Ok(())
}