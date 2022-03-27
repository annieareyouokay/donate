use std::convert::TryInto;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    
    // let amount_lamports = _instruction_data
    //     .get(..8)
    //     .and_then(|slice| slice.try_into().ok())
    //     .map(u64::from_le_bytes)
    //     .ok_or(ProgramError::InvalidInstructionData)?;

    let iter_acc = &mut accounts.iter();
    let to_account = next_account_info(iter_acc)?;
    let from_account = next_account_info(iter_acc)?;

    // if account.owner != program_id {
    //     msg!("Incorrect program id!");
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    // let mut g_acc = ProgramAccount::try_from_slice(&account.data.borrow())?;
    // g_acc.counter += 1;
    // g_acc.serialize(&mut &mut account.data.borrow_mut()[..])?;

    invoke(
        &system_instruction::transfer(from_account.key, to_account.key, 25u64),
        &[from_account.clone(), to_account.clone()]
    )?;

    Ok(())
}