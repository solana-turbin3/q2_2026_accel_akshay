use borsh::{BorshDeserialize,BorshSerialize};

#[derive(BorshSerialize,BorshDeserialize)]
pub enum InstructionType{
    MakeOffer{token_a_amount:u64, token_b_amount:u64, expiry_time:u64, escrow_id:u64, escrow_bump:u8},
    TakeOffer{escrow_id:u64,escrow_bump:u8},
    CancelOffer{escrow_id:u64,escrow_bump:u8}
}