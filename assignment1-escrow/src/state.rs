use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::{pubkey::Pubkey};

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct EscrowVault{
    pub mint_a:Pubkey,
    pub mint_b:Pubkey,

    pub token_a_amount:u64,
    pub token_b_amount:u64,
    
    pub expiry_time:u64,
    pub escrow_state:EscrowVaultState
}

impl EscrowVault{
    pub const ESCROW_VAULT_SIZE:usize=32+ 32+ 8+ 8+ 8+ 1;
}

#[derive(BorshSerialize,BorshDeserialize,PartialEq,Debug)]
pub enum EscrowVaultState{
    COMPLETED,
    PENDING,
    CANCELLED
}