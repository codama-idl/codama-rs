pub struct SetOnce<'a, T> {
    value: Option<T>,
    ident: &'static str,
    meta: &'a syn::meta::ParseNestedMeta<'a>,
    is_set: bool,
}

impl<'a, T> SetOnce<'a, T> {
    pub fn new(ident: &'static str, meta: &'a syn::meta::ParseNestedMeta<'a>) -> Self {
        Self {
            value: None,
            ident,
            meta,
            is_set: false,
        }
    }

    // pub fn initial_value(mut self, value: T) -> Self {
    //     self.value = Some(value);
    //     self
    // }

    pub fn set(&mut self, value: T) -> syn::Result<()> {
        if self.is_set {
            return Err(self.meta.error(format!("{} is already set", self.ident)));
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
            None => Err(self.meta.error(format!("{} is missing", self.ident))),
        }
    }
}
