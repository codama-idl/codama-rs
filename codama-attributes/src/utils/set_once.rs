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

pub struct SetOnce<T> {
    value: Option<T>,
    ident: &'static str,
    is_set: bool,
}

impl<T> SetOnce<T> {
    pub fn new(ident: &'static str) -> Self {
        Self {
            value: None,
            ident,
            is_set: false,
        }
    }

    // pub fn initial_value(mut self, value: T) -> Self {
    //     self.value = Some(value);
    //     self
    // }

    pub fn set<U: IntoSynError>(&mut self, value: T, tokens: U) -> syn::Result<()> {
        if self.is_set {
            return Err(tokens.get_syn_error(format!("{} is already set", self.ident)));
        }
        self.is_set = true;
        self.value = Some(value);
        Ok(())
    }

    // pub fn option(&self) -> &Option<T> {
    //     &self.value
    // }

    pub fn take<U: IntoSynError>(&mut self, tokens: U) -> syn::Result<T> {
        match self.value.take() {
            Some(value) => Ok(value),
            None => Err(tokens.get_syn_error(format!("{} is missing", self.ident))),
        }
    }
}
