use codama_macros::codama;

#[codama(type = enum)]
pub struct EmptyTest;

#[codama(type = enum(variant("pending"), variant("done")))]
pub struct EmptyVariantsTest;

#[codama(type = enum(variant("point", struct(field("x", number(u8))))))]
pub struct StructVariantTest;

#[codama(type = enum(variant("pair", tuple(number(u8)))))]
pub struct TupleVariantTest;

#[codama(type = enum(variant("a", discriminator = 3)))]
pub struct DiscriminatorTest;

#[codama(type = enum(variant("a"), size = number(u16)))]
pub struct ExplicitSizeTest;

fn main() {}
