#[repr(C)]
pub struct FundraiserVault{
    pub maker:[u8;32],
    pub token_mint:[u8;32],
    pub target_amount:[u8;8],
    pub total_amount:[u8;8],
    pub start_time:[u8;8],
    pub expiry_time:[u8;8],
    pub bump:u8,
}

impl FundraiserVault{
    pub const FUNDRAISER_PDA_SIZE:usize=32+ 32+ 8+ 8+ 8+ 8+ 1;
}


#[repr(C)]
pub struct UserContribution{
    pub user:[u8;32],
    pub amount_contributed:[u8;8],
    pub bump:u8
}

impl UserContribution{
    pub const USER_CONTRIBUTION_SIZE:usize=32 + 8+ 1;
}