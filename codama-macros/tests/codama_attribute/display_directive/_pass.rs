use codama_macros::{codama, CodamaType};

#[codama(display(
    intent = "Transfer tokens",
    interpolated_intent = "Transfer ${data.amount} to ${accounts.destination}"
))]
pub struct InstructionDisplayTest;

#[derive(CodamaType)]
pub struct FieldDisplayTest {
    #[codama(display(
        label = "Authorities",
        skip = when_injected,
        flatten,
        flatten_prefix = "Authority "
    ))]
    pub authorities: u64,
}

#[codama(display(amount(decimals = 9, unit = "SOL")))]
pub struct AmountDisplayTest;

#[codama(display(date_time))]
pub struct DateTimeDisplayTest;

#[codama(account(
    name = "payer",
    display(label = "Payer", skip = always)
))]
pub struct AccountDisplayTest;

fn main() {}
