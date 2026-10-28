// Spec v2 has separate integer and float values.
// A number written as `1` becomes a float value when its type is a float, also when the type is behind a link.
//
// Cases:
// - Float types written directly: `scale`, `factor` (with `value = ...`) and a type override (`weight`).
// - Links to a float type: `rate` and a chain of two links (`fee`).
// - Values that stay as written: a link to an integer type (`max_orders`) and an option (`threshold`).
// - Constants in types: a sentinel with a linked type (`label`) and hidden prefix constants (`memo`).
// - `field(...)` forms: on the account (`bonus`, `spread`), as a type (`limit`, `cap`), on the instruction (`bonus`).
// - An enum variant with a linked field (`Pricing::Fixed`).
// - PDA seeds: a float type (`RatePda`), a linked type (`LinkedRatePda`), a type from a field (`FieldRatePda`) and
//   constant seeds (`ConstRatePda`).
// - PDA values in account defaults: the PDAs above, this program by name, and PDAs in the `oracle` program.
// - A PDA value in `provides` (`derived`).
// - Instruction data: a float field (`scale`), a linked field (`amount`) and tuple items (`SetScale`, `SetFee`).

#[derive(CodamaType)]
pub struct Rate(f64);

#[derive(CodamaType)]
pub struct FeeRate(Rate);

#[derive(CodamaType)]
pub struct Count(u64);

#[derive(CodamaType)]
pub enum Pricing {
    Fixed {
        #[codama(default_value = 1)]
        rate: Rate,
    },
    Floating,
}

#[derive(CodamaAccount)]
#[codama(field("bonus", float(f32), default_value = 1))]
#[codama(field("spread", link("rate"), default_value = 1))]
pub struct Config {
    #[codama(default_value = 1)]
    pub scale: f64,
    #[codama(value = 1)]
    pub factor: f64,
    #[codama(type = float(f64))]
    #[codama(default_value = 1)]
    pub weight: u64,
    #[codama(default_value = 1)]
    pub rate: Rate,
    #[codama(default_value = 1)]
    pub fee: FeeRate,
    #[codama(default_value = 1)]
    pub max_orders: Count,
    #[codama(default_value = 1)]
    pub threshold: Option<f64>,
    #[codama(type = sentinel(string, constant(link("rate"), 1)))]
    pub label: String,
    #[codama(type = hidden_prefix(string, constant(float(f32), 1), constant(type = float(f64), value = -2)))]
    pub memo: String,
    #[codama(type = field("limit", float(f64), default_value = 1))]
    pub limit: u8,
    #[codama(type = field("cap", link("rate"), default_value = 1))]
    pub cap: u8,
}

#[derive(CodamaPda)]
#[codama(seed(name = "rate", type = float(f64)))]
pub struct RatePda;

#[derive(CodamaPda)]
#[codama(seed(name = "rate", type = link("rate")))]
pub struct LinkedRatePda;

#[derive(CodamaPda)]
#[codama(seed(name = "rate"))]
pub struct FieldRatePda {
    pub rate: Rate,
}

#[derive(CodamaPda)]
#[codama(seed(type = float(f32), value = 1))]
#[codama(seed(type = link("rate"), value = 1))]
pub struct ConstRatePda;

#[codama(program(
    name = "oracle",
    address = "Price111111111111111111111111111111111111111"
))]
pub mod oracle {
    #[derive(CodamaType)]
    pub struct Price(f64);

    #[derive(CodamaPda)]
    #[codama(seed(name = "price", type = float(f64)))]
    pub struct PricePda;

    #[derive(CodamaPda)]
    #[codama(seed(name = "price", type = link("price")))]
    pub struct LinkedPricePda;
}

#[derive(CodamaInstruction)]
#[codama(field("bonus", link("rate"), default_value = 1))]
pub struct UpdateRate {
    #[codama(account(writable, default_value = pda("ratePda", [seed("rate", 1)])))]
    pub rate_account: AccountMeta,
    #[codama(account(writable, default_value = pda("linkedRatePda", [seed("rate", 1)])))]
    pub linked_rate_account: AccountMeta,
    #[codama(account(writable, default_value = pda("fieldRatePda", [seed("rate", 1)])))]
    pub field_rate_account: AccountMeta,
    #[codama(account(default_value = pda("ratePda", program = "exchange", [seed("rate", 1)])))]
    pub own_rate_account: AccountMeta,
    #[codama(account(default_value = pda("pricePda", program = "oracle", [seed("price", 1)])))]
    pub price_account: AccountMeta,
    #[codama(account(default_value = pda("linkedPricePda", program = "oracle", [seed("price", 1)])))]
    pub linked_price_account: AccountMeta,
    #[codama(default_value = 1)]
    pub scale: f64,
    #[codama(default_value = 1)]
    pub amount: Rate,
    #[codama(default_value = pda("ratePda", [seed("rate", 1)]))]
    pub derived: Pubkey,
}

#[derive(CodamaInstructions)]
pub enum ExchangeInstruction {
    SetScale(#[codama(default_value = 1)] f32),
    SetFee(#[codama(default_value = 1)] Rate),
}
