use codama_syn_helpers::{extensions::*, Meta};

pub(super) fn bare_path(meta: &Meta) -> Option<String> {
    meta.as_expr()
        .ok()?
        .as_path()
        .ok()
        .map(|path| path.to_string())
}
