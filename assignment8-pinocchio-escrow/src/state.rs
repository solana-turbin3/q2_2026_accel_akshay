#[repr(C)]
#[derive(Debug)]
pub struct EscrowVaultPda{
    pub mint_a:[u8;32],
    pub mint_b:[u8;32],
    pub token_a_amount:[u8;8],
    pub token_b_amount:[u8;8],

    pub expiry_time:[u8;8],
    pub vault_status:u8   // 0=pending,1=completed,2=cancelled
}

impl EscrowVaultPda{
    pub const ESCROW_VAULT_PDA_SIZE:usize=32+32+8+8+8+1;
}