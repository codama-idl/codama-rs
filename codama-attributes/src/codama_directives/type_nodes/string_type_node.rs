use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{BytesEncoding, StringTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for StringTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("string")?;
        let mut encoding: SetOnce<BytesEncoding> = SetOnce::new("encoding");
        if meta.is_path_or_empty_list() {
            return Ok(StringTypeNode::utf8());
        }

        meta.as_path_list()?
            .each(|ref meta| match meta.path_str().as_str() {
                "encoding" => encoding.set(BytesEncoding::from_meta(meta)?, meta),
                _ => {
                    if let Ok(value) = BytesEncoding::from_meta(meta) {
                        return encoding.set(value, meta);
                    }
                    Err(meta.error("unrecognized attribute"))
                }
            })?;

        Ok(StringTypeNode::new(encoding.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_type, assert_type_err};

    #[test]
    fn default() {
        assert_type!({ string }, StringTypeNode::utf8().into());
        assert_type!({ string() }, StringTypeNode::utf8().into());
    }

    #[test]
    fn implicit() {
        assert_type!({ string(utf8) }, StringTypeNode::utf8().into());
        assert_type!({ string(base16) }, StringTypeNode::base16().into());
        assert_type!({ string(base58) }, StringTypeNode::base58().into());
        assert_type!({ string(base64) }, StringTypeNode::base64().into());
    }

    #[test]
    fn explicit() {
        assert_type!({ string(encoding = utf8) }, StringTypeNode::utf8().into());
        assert_type!(
            { string(encoding = base16) },
            StringTypeNode::base16().into()
        );
        assert_type!(
            { string(encoding = base58) },
            StringTypeNode::base58().into()
        );
        assert_type!(
            { string(encoding = base64) },
            StringTypeNode::base64().into()
        );
    }

    #[test]
    fn invalid_encoding() {
        assert_type_err!(
            { string(encoding = unrecognized) },
            "expected one of: base16, base58, base64, utf8"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ string(unrecognized) }, "unrecognized attribute");
        assert_type_err!({ string(foo = 42) }, "unrecognized attribute");
    }
}
