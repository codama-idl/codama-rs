use crate::utils::FromMeta;
use codama_syn_helpers::{extensions::ToTokensExtension, Meta};

pub trait MetaConsumer: Sized {
    fn new(metas: Vec<Meta>) -> Self;
    fn metas(&self) -> &[Meta];
    fn metas_mut(&mut self) -> &mut Vec<Meta>;

    fn consume_metas(
        mut self,
        mut logic: impl FnMut(&mut Self, Meta) -> syn::Result<Option<Meta>>,
    ) -> syn::Result<Self> {
        let metas = std::mem::take(self.metas_mut());
        let metas = metas.into_iter().try_fold(Vec::new(), |mut acc, meta| {
            if let Some(meta) = logic(&mut self, meta)? {
                acc.push(meta);
            }
            syn::Result::Ok(acc)
        })?;

        *self.metas_mut() = metas;
        Ok(self)
    }

    fn assert_fully_consumed(self) -> syn::Result<Self> {
        self.metas()
            .iter()
            .try_for_each(|meta| Err(meta.error("unrecognized attribute")))?;
        Ok(self)
    }
}

impl<T: MetaConsumer> FromMeta for T {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        Ok(Self::new(meta.as_path_list()?.parse_metas()?))
    }
}
