#![allow(unexpected_cfgs)]
use pinocchio::{
    AccountView, Address, ProgramResult, entrypoint, error::ProgramError
};
use pinocchio_log::{log, logger::Logger};
use pinocchio_pubkey::declare_id;

use crate::instructions::{EscrowIstructions, make_offer::make_offer, take_offer::take_offer};

pub mod instructions;
pub mod state;

entrypoint!(process_instruction);
declare_id!("4vGPskg8Ku4jGnDY3s4kRiuK6zGct7tmiexbxqEd3BTY");   //@q

pub fn process_instruction(
    program_id:&Address, accounts:&mut [AccountView], instruction_data:&[u8]
)->ProgramResult{

    let (discriminator,data)=instruction_data.split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowIstructions::try_from(discriminator)?{
        EscrowIstructions::Make=>{
            // make_offer(accounts, instruction_data)?;
            make_offer(accounts, data)?;
        },
        EscrowIstructions::Take=>{
            take_offer(accounts,data)?;
        }
    }
    Ok(())
}