// #![allow(unexpected_cfgs)]
pub mod state;
pub mod instructions;
pub mod constants;

use pinocchio::{AccountView, Address, ProgramResult, address::declare_id, entrypoint, error::ProgramError};

use crate::instructions::{claim_fund::claim_fund, contribute::contribute, create_fundraiser::create_fundraiser, refund_fund::refund_fund};

entrypoint!(process_instruction);

declare_id!("8XguEtT1GjWtfZU3Tvjhnxmrk7owYhMcVE2WBUfXgq8h");  
pub fn process_instruction(
    program_id:&Address, accounts:&mut [AccountView], data:&[u8]
)->ProgramResult{
    if program_id!=&crate::ID{
        return Err(ProgramError::IncorrectProgramId);
    }
    let (discriminator,ix_data)=data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    match *discriminator{
        0=>{ create_fundraiser(accounts, ix_data)?; },
        1=>{ contribute(accounts, ix_data)?; },
        2=>{ claim_fund(accounts, ix_data)?; },
        3=>{ refund_fund(accounts, ix_data)?; }
        _=>{ return Err(ProgramError::InvalidInstructionData);}
    };
    Ok(())
}