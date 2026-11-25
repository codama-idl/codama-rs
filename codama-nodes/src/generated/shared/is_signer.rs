#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum IsSigner {
    True,
    False,
    Either,
}
