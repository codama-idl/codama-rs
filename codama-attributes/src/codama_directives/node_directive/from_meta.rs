use codama_syn_helpers::Meta;

pub trait FromMeta
where
    Self: Sized,
{
    fn from_meta(meta: &Meta) -> syn::Result<Self>;
}
