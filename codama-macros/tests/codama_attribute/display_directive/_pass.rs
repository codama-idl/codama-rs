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

#[codama(display(unit("SOL")))]
pub struct UnitDisplayTest;

#[codama(display(unit(public_key("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"))))]
pub struct AddressUnitDisplayTest;

#[codama(display(amount(
    decimals = 6,
    unit = public_key("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
)))]
pub struct AddressAmountDisplayTest;

#[codama(account(
    name = "payer",
    display(label = "Payer", skip = always)
))]
pub struct AccountDisplayTest;

fn main() {}
