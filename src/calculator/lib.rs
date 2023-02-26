use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Calculator {
    pub value: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorInstructions {
    pub value: u32,
    pub operation: u32,
}

impl CalculatorInstructions {
    pub fn execute(&self, value: u32) -> u32 {
        match self.operation {
            0 => value + self.value,
            1 => value + self.value,
            2 => value + self.value,
            3 => value + self.value,
            _ => value
        }
    }
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Account not belongs to this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut calculator = Calculator::try_from_slice(&account.data.borrow())?;

    let calculator_instruction = CalculatorInstructions::try_from_slice(&instruction_data)?;

    msg!("Value Before: {}", calculator.value);

    calculator.value = calculator_instruction.execute(calculator.value);

    calculator.serialize(&mut *account.data.borrow_mut())?;

    msg!("Value After: {}", calculator.value);

    Ok(());
}