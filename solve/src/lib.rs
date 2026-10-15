use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let player = next_account_info(&mut iter)?;
    let prize = next_account_info(&mut iter)?;
    let challenge_program = next_account_info(&mut iter)?;

    invoke(
        &Instruction {
            program_id: *challenge_program.key,
            accounts: vec![
                AccountMeta::new(*player.key, true),   // player (signer + writable)
                AccountMeta::new(*prize.key, false),  // prize PDA (writable)
            ],
            data: vec![0],  // ClaimWelcomeGrant
        },
        &[player.clone(), prize.clone(), challenge_program.clone()],
    )
}