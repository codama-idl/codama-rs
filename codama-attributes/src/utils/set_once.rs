use std::fmt::Display;

pub trait IntoSynError {
    fn get_syn_error(&self, msg: impl Display) -> syn::Error;
}

impl IntoSynError for &syn::meta::ParseNestedMeta<'_> {
    fn get_syn_error(&self, msg: impl Display) -> syn::Error {
        self.error(msg)
    }
}

impl IntoSynError for &syn::Attribute {
    fn get_syn_error(&self, msg: impl Display) -> syn::Error {
        syn::Error::new_spanned(self, msg)
    }
}

pub struct SetOnce<T, U: IntoSynError> {
    value: Option<T>,
    ident: &'static str,
    tokens: U,
    is_set: bool,
}

impl<T, U: IntoSynError> SetOnce<T, U> {
    pub fn new(ident: &'static str, tokens: U) -> Self {
        Self {
            value: None,
            ident,
            tokens,
            is_set: false,
        }
    }

    // pub fn initial_value(mut self, value: T) -> Self {
    //     self.value = Some(value);
    //     self
    // }

    pub fn set(&mut self, value: T) -> syn::Result<()> {
        if self.is_set {
            return Err(self
                .tokens
                .get_syn_error(format!("{} is already set", self.ident)));
        }
        self.is_set = true;
        self.value = Some(value);
        Ok(())
    }

    // pub fn option(&self) -> &Option<T> {
    //     &self.value
    // }

    pub fn take(&mut self) -> syn::Result<T> {
        match self.value.take() {
            Some(value) => Ok(value),
            None => Err(self
                .tokens
                .get_syn_error(format!("{} is missing", self.ident))),
        }
    }
}
