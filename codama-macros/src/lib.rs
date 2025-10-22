use proc_macro::TokenStream;
#[cfg(not(target_os = "solana"))]
mod attributes;
#[cfg(not(target_os = "solana"))]
mod derives;

fn codama_derive(input: TokenStream) -> TokenStream {
    #[cfg(not(target_os = "solana"))]
    {
        derives::codama_derive(input)
    }
    #[cfg(target_os = "solana")]
    {
        input
    }
}

fn codama_attribute(attr: TokenStream, input: TokenStream) -> TokenStream {
    #[cfg(not(target_os = "solana"))]
    {
        attributes::codama_attribute(attr, input)
    }
    #[cfg(target_os = "solana")]
    {
        let _ = attr;
        input
    }
}

#[proc_macro_derive(CodamaAccount, attributes(codama))]
pub fn codama_account_derive(input: TokenStream) -> TokenStream {
    codama_derive(input)
}

#[proc_macro_derive(CodamaAccounts, attributes(codama))]
pub fn codama_accounts_derive(input: TokenStream) -> TokenStream {
    codama_derive(input)
}

#[proc_macro_derive(CodamaErrors, attributes(codama))]
pub fn codama_errors_derive(input: TokenStream) -> TokenStream {
    codama_derive(input)
}

#[proc_macro_derive(CodamaInstruction, attributes(codama))]
pub fn codama_instruction_derive(input: TokenStream) -> TokenStream {
    codama_derive(input)
}

#[proc_macro_derive(CodamaInstructions, attributes(codama))]
pub fn codama_instructions_derive(input: TokenStream) -> TokenStream {
    codama_derive(input)
}

#[proc_macro_derive(CodamaPda, attributes(codama))]
pub fn codama_pda_derive(input: TokenStream) -> TokenStream {
    codama_derive(input)
}

#[proc_macro_derive(CodamaType, attributes(codama))]
pub fn codama_type_derive(input: TokenStream) -> TokenStream {
    codama_derive(input)
}

#[proc_macro_attribute]
pub fn codama(attr: TokenStream, input: TokenStream) -> TokenStream {
    codama_attribute(attr, input)
}
