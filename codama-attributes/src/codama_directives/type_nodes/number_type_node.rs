use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{Endianness, NumberDisplayNode, NumberFormat, NumberTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for NumberTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("number")?.as_path_list()?;
        let mut format = SetOnce::<NumberFormat>::new("format");
        let mut endian = SetOnce::<Endianness>::new("endian").initial_value(Endianness::Le);
        let mut display = SetOnce::<NumberDisplayNode>::new("display");

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
                match Endianness::try_from(path.to_string()) {
                    Ok(value) => endian.set(value, meta),
                    _ => Err(path.error("invalid endian")),
                }
            }
            "display" => display.set(NumberDisplayNode::from_meta(meta.as_value()?)?, meta),
            _ => {
                if let Ok(path) = meta.as_path() {
                    if let Ok(value) = NumberFormat::try_from(path.to_string()) {
                        return format.set(value, meta);
                    }
                    if let Ok(value) = Endianness::try_from(path.to_string()) {
                        return endian.set(value, meta);
                    }
                }
                Err(meta.path()?.error("unrecognized attribute"))
            }
        })?;

        Ok(NumberTypeNode {
            format: format.take(meta)?,
            endian: endian.take(meta)?,
            display: Box::new(display.option()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        AmountNumberDisplayNode, DateTimeNumberDisplayNode, NumberValueNode, StringValueNode,
    };
    use NumberFormat::{I64, U16, U64};

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
    fn amount_display() {
        assert_type!(
            { number(u64, display = amount(decimals = 9, unit = "SOL")) },
            NumberTypeNode {
                display: Box::new(Some(NumberDisplayNode::Amount(AmountNumberDisplayNode {
                    decimals: Box::new(Some(NumberValueNode::new(9u64).into())),
                    unit: Box::new(Some(StringValueNode::new("SOL").into())),
                }))),
                ..NumberTypeNode::le(U64)
            }
            .into()
        );
    }

    #[test]
    fn date_time_display() {
        assert_type!(
            { number(i64, display = date_time) },
            NumberTypeNode {
                display: Box::new(Some(NumberDisplayNode::DateTime(
                    DateTimeNumberDisplayNode::default()
                ))),
                ..NumberTypeNode::le(I64)
            }
            .into()
        );
        assert_type!(
            { number(i64, display = date_time(ticks_per_second = 1_000)) },
            NumberTypeNode {
                display: Box::new(Some(NumberDisplayNode::DateTime(
                    DateTimeNumberDisplayNode {
                        ticks_per_second: Some(1_000),
                    }
                ))),
                ..NumberTypeNode::le(I64)
            }
            .into()
        );
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
    fn display_already_set() {
        assert_type_err!(
            { number(u64, display = amount, display = date_time) },
            "display is already set"
        );
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
