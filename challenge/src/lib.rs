use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::set_return_data,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

const CLAIM_WELCOME_GRANT: u8 = 0;
const QUOTE_WELCOME_GRANT: u8 = 1;
const GET_SERVICE_METADATA: u8 = 2;
const WELCOME_VAULT_SEED: &[u8] = b"prize";
const SERVICE_METADATA: &[u8] = b"welcome-grants:v1";

enum WelcomeProgramInstruction {
    ClaimWelcomeGrant,
    QuoteWelcomeGrant,
    GetServiceMetadata,
}

impl WelcomeProgramInstruction {
    fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input {
            [CLAIM_WELCOME_GRANT] => Ok(Self::ClaimWelcomeGrant),
            [QUOTE_WELCOME_GRANT] => Ok(Self::QuoteWelcomeGrant),
            [GET_SERVICE_METADATA] => Ok(Self::GetServiceMetadata),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

fn welcome_vault_address(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[WELCOME_VAULT_SEED], program_id).0
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match WelcomeProgramInstruction::unpack(instruction_data)? {
        WelcomeProgramInstruction::ClaimWelcomeGrant => {
            process_claim_welcome_grant(program_id, accounts)
        }
        WelcomeProgramInstruction::QuoteWelcomeGrant => {
            process_quote_welcome_grant(program_id, accounts)
        }
        WelcomeProgramInstruction::GetServiceMetadata => {
            set_return_data(SERVICE_METADATA);
            Ok(())
        }
    }
}

fn process_quote_welcome_grant(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let welcome_vault = accounts.first().ok_or(ProgramError::NotEnoughAccountKeys)?;
    validate_welcome_vault(program_id, welcome_vault)?;
    set_return_data(&welcome_vault.lamports().to_le_bytes());
    Ok(())
}

fn validate_welcome_vault(program_id: &Pubkey, welcome_vault: &AccountInfo) -> ProgramResult {
    if welcome_vault.key != &welcome_vault_address(program_id) || welcome_vault.owner != program_id
    {
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}

fn process_claim_welcome_grant(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut iter = accounts.iter();
    let player = next_account_info(&mut iter)?;
    let welcome_vault = next_account_info(&mut iter)?;

    if !player.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !player.is_writable || !welcome_vault.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    validate_welcome_vault(program_id, welcome_vault)?;

    settle_welcome_grant(welcome_vault, player)
}

fn settle_welcome_grant(welcome_vault: &AccountInfo, player: &AccountInfo) -> ProgramResult {
    let amount = welcome_vault.lamports();
    if amount == 0 {
        return Err(ProgramError::InsufficientFunds);
    }

    **welcome_vault.try_borrow_mut_lamports()? = 0;
    **player.try_borrow_mut_lamports()? = player
        .lamports()
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(())
}
