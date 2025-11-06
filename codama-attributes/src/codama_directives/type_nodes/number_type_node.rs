use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{Endian, NumberFormat, NumberTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for NumberTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("number")?.as_path_list()?;
        let mut format = SetOnce::<NumberFormat>::new("format");
        let mut endian = SetOnce::<Endian>::new("endian").initial_value(Endian::Little);

        pl.each(|ref meta| match meta.path_str().as_str() {
            "format" => {
                let path = meta.as_value()?.as_path()?;
                match NumberFormat::try_from(path.to_string()) {
                    Ok(value) => format.set(value, meta),
                    _ => Err(path.error("invalid format")),
                }
            }
            "endian" => {
                let path = meta.as_value()?.as_path()?;
                match Endian::try_from(path.to_string()) {
                    Ok(value) => endian.set(value, meta),
                    _ => Err(path.error("invalid endian")),
                }
            }
            _ => {
                if let Ok(path) = meta.as_path() {
                    if let Ok(value) = NumberFormat::try_from(path.to_string()) {
                        return format.set(value, meta);
                    }
                    if let Ok(value) = Endian::try_from(path.to_string()) {
                        return endian.set(value, meta);
                    }
                }
                Err(meta.path()?.error("unrecognized attribute"))
            }
        })?;

        Ok(Self::new(format.take(meta)?, endian.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use NumberFormat::{U16, U64};

    #[test]
    fn implicit() {
        assert_type!({ number(u16, le) }, NumberTypeNode::le(U16).into());
        assert_type!({ number(u16, le) }, NumberTypeNode::le(U16).into());
        assert_type!({ number(u64, le) }, NumberTypeNode::le(U64).into());
        assert_type!({ number(u16, be) }, NumberTypeNode::be(U16).into());
        assert_type!({ number(u64, be) }, NumberTypeNode::be(U64).into());
        assert_type!({ number(le, u16) }, NumberTypeNode::le(U16).into());
    }

    #[test]
    fn explicit() {
        assert_type!(
            { number(format = u16, endian = le) },
            NumberTypeNode::le(U16).into()
        );
        assert_type!(
            { number(format = u16, endian = le) },
            NumberTypeNode::le(U16).into()
        );
        assert_type!(
            { number(format = u64, endian = le) },
            NumberTypeNode::le(U64).into()
        );
        assert_type!(
            { number(format = u16, endian = be) },
            NumberTypeNode::be(U16).into()
        );
        assert_type!(
            { number(format = u64, endian = be) },
            NumberTypeNode::be(U64).into()
        );
        assert_type!(
            { number(endian = le, format = u16) },
            NumberTypeNode::le(U16).into()
        );
    }

    #[test]
    fn defaults_to_little_endian() {
        assert_type!({ number(u16) }, NumberTypeNode::le(U16).into());
        assert_type!({ number(format = u16) }, NumberTypeNode::le(U16).into());
    }

    #[test]
    fn missing_format() {
        assert_type_err!({ number(le) }, "format is missing");
    }

    #[test]
    fn format_already_set() {
        assert_type_err!({ number(u8, u16) }, "format is already set");
    }

    #[test]
    fn endian_already_set() {
        assert_type_err!({ number(le, be) }, "endian is already set");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ number(u16, le, unknown) }, "unrecognized attribute");
        assert_type_err!({ number(u16, le, unknown = 42) }, "unrecognized attribute");
        assert_type_err!({ number(u16 = ?what?, le) }, "unrecognized attribute");
    }

    #[test]
    fn expected_a_path() {
        assert_type_err!({ number(42) }, "expected a path");
    }
}
