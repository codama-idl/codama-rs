use codama::CodamaErrors;
use thiserror::Error;

#[derive(CodamaErrors, Error, Debug)]
pub enum SystemError {
    #[error("an account with the same address already exists")]
    AccountAlreadyInUse,

    #[error("account does not have enough SOL to perform the operation")]
    ResultWithNegativeLamports,

    #[error("cannot assign account to this program id")]
    InvalidProgramId,

    #[error("cannot allocate account data of this length")]
    InvalidAccountDataLength,

    #[error("length of requested seed is too long")]
    MaxSeedLengthExceeded,

    #[error("provided address does not match addressed derived from seed")]
    AddressWithSeedMismatch,

    #[error("advancing stored nonce requires a populated RecentBlockhashes sysvar")]
    NonceNoRecentBlockhashes,

    #[error("stored nonce is still in recent_blockhashes")]
    NonceBlockhashNotExpired,

    #[error("specified nonce does not match stored nonce")]
    NonceUnexpectedBlockhashValue,
}
