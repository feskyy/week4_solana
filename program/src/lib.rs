use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub const MAX_ECHO_LEN: usize = 128;

pub enum HelloInstruction<'a> {
    Hello,
    Echo(&'a str),
}

impl<'a> HelloInstruction<'a> {
    pub fn unpack(data: &'a [u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => Ok(Self::Hello),
            1 => {
                if rest.len() > MAX_ECHO_LEN {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let text =
                    core::str::from_utf8(rest).map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::Echo(text))
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match HelloInstruction::unpack(instruction_data)? {
        HelloInstruction::Hello => {
            msg!("Hello, Solana! Program ID: {}", program_id);
            msg!("Accounts passed: {}", accounts.len());
        }
        HelloInstruction::Echo(text) => {
            msg!("Echo: {}", text);
        }
    }
    Ok(())
}
