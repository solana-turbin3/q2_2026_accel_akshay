pub mod processor;
pub mod instruction;
pub mod state;
pub mod instructions;

use solana_program::{entrypoint};
// use borsh::{BorshDeserialize,BorshSerialize};

use crate::processor::process_instruction;
entrypoint!(process_instruction);
