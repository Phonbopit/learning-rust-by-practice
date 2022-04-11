use solana_program::{account_info::AccountInfo, entrypoint, msg, pubkey::Pubkey};

// Solana program's entrypoint
entrypoint!(process_instruction);

// program_id, accounts and instruction_data are required.
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    msg!("Hello Solaan from Rust entrypoint.");
    Ok(())
}
