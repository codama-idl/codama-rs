/// Strips a leading identifier `[A-Za-z_][A-Za-z0-9_]*` and returns the rest.
pub(super) fn strip_identifier(value: &str) -> Option<&str> {
    let first_ok =
        value.starts_with(|character: char| character.is_ascii_alphabetic() || character == '_');
    let end = value
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(value.len());
    first_ok.then(|| &value[end..])
}

/// Strips a leading index `"[" (0|[1-9][0-9]*) "]"` and returns the rest.
pub(super) fn strip_index(value: &str) -> Option<&str> {
    let (integer, rest) = value.strip_prefix('[')?.split_once(']')?;
    let is_digits = !integer.is_empty() && integer.bytes().all(|byte| byte.is_ascii_digit());
    let is_integer = is_digits && (integer == "0" || !integer.starts_with('0'));
    is_integer.then_some(rest)
}
