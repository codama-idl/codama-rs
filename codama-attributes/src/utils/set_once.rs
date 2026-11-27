use codama_syn_helpers::extensions::*;
use quote::ToTokens;

pub(crate) struct SetOnce<T> {
    value: Option<T>,
    ident: &'static str,
    is_set: bool,
}

impl<T> SetOnce<T> {
    pub(crate) fn new(ident: &'static str) -> Self {
        Self {
            value: None,
            ident,
            is_set: false,
        }
    }

    pub(crate) fn is_set(&self) -> bool {
        self.is_set
    }

    pub(crate) fn initial_value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set<U: ToTokens>(&mut self, value: T, tokens: U) -> syn::Result<()> {
        if self.is_set {
            return Err(tokens.error(format!("{} is already set", self.ident)));
        }
        self.is_set = true;
        self.value = Some(value);
        Ok(())
    }

    pub(crate) fn option(self) -> Option<T> {
        self.value
    }

    pub(crate) fn option_ref(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub(crate) fn take<U: ToTokens>(mut self, tokens: U) -> syn::Result<T> {
        match self.value.take() {
            Some(value) => Ok(value),
            None => Err(tokens.error(format!("{} is missing", self.ident))),
        }
    }
}
