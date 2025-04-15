use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};

// Определяем структуру данных для счетчика
#[derive(Default)]
pub struct Counter {
    pub is_initialized: bool,
    pub count: u64,
}

impl Sealed for Counter {}

impl IsInitialized for Counter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Counter {
    const LEN: usize = 9; // 1 байт для is_initialized + 8 байт для count

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Counter::LEN];
        let (is_initialized_dst, count_dst) = mut_array_refs![dst, 1, 8];
        is_initialized_dst[0] = self.is_initialized as u8;
        *count_dst = self.count.to_le_bytes();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Counter::LEN];
        let (is_initialized, count) = array_refs![src, 1, 8];
        Ok(Counter {
            is_initialized: match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            count: u64::from_le_bytes(*count),
        })
    }
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction");

    let account_info_iter = &mut accounts.iter();
    let counter_account = next_account_info(account_info_iter)?;

    // Проверяем, что аккаунт принадлежит нашей программе
    if counter_account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Проверяем, что аккаунт подписан
    if !counter_account.is_signer {
        msg!("Counter account must be signed");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut counter = Counter::unpack_unchecked(&counter_account.data.borrow())?;
    
    if !counter.is_initialized {
        counter.is_initialized = true;
        counter.count = 0;
    } else {
        counter.count = counter.count.checked_add(1).ok_or(ProgramError::Overflow)?;
    }

    Counter::pack(counter, &mut counter_account.data.borrow_mut())?;
    msg!("Counter incremented to: {}", counter.count);

    Ok(())
} 