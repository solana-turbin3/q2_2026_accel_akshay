pub mod make_offer;
pub mod take_offer;

use pinocchio::error::ProgramError;

pub enum EscrowIstructions{
    Make,
    Take
}

impl TryFrom<&u8> for EscrowIstructions{
    type Error=ProgramError;
    fn try_from(ix:&u8)->Result<Self,Self::Error>{
        match *ix{
            0=>Ok(EscrowIstructions::Make),
            1=>Ok(EscrowIstructions::Take),
            _=>Err(ProgramError::InvalidInstructionData)
        }
    }
}










// impl TryFrom<&u8> for EscrowIstructions{
//     type Error=ProgramError;
//     fn try_from(val:&u8)->Result<Self,Self::Error>{
//         let x=match *val{
//             0=>Ok(EscrowIstructions::Make),
//             1=>Ok(EscrowIstructions::Take),
//             _=>Err(ProgramError::InvalidInstructionData)
//         };
//         x
//     }
// }