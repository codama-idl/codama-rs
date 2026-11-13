use crate::{
    AccountDataValueNode, AccountValueNode, DataValueNode, Identifier, InstructionDisplayNode,
    Path, Text,
};
use codama_errors::{CodamaError, CodamaResult};

impl InstructionDisplayNode {
    pub fn new<T>(intent: T) -> Self
    where
        T: Into<Text>,
    {
        Self {
            intent: Some(intent.into()),
            ..Default::default()
        }
    }

    pub fn interpolated<T>(interpolated_intent: T) -> Self
    where
        T: Into<Text>,
    {
        Self {
            interpolated_intent: Some(interpolated_intent.into()),
            ..Default::default()
        }
    }
}

/// A part of an interpolated intent: literal text, or the contextual value that a placeholder refers to.
///
/// - `${data.config.fees[0]}` is a `dataValueNode` with the path `config.fees[0]`.
/// - `${accounts.mint}` is an `accountValueNode`, the address of the `mint` account.
/// - `${accounts.mint.data.decimals}` is an `accountDataValueNode` of the `mint` account with the path
///   `decimals`.
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolatedIntentPart {
    Text(String),
    Data(DataValueNode),
    Account(AccountValueNode),
    AccountData(AccountDataValueNode),
}

const PLACEHOLDER_START: &str = "${";
const PLACEHOLDER_END: &str = "}";

/// Parses an interpolated intent into its literal text and the contextual values its placeholders refer
/// to, like `parseInterpolatedIntent` in the JavaScript library.
///
/// Placeholders start with the `data` root, followed by a path into the instruction data, or the
/// `accounts` root, followed by an account identifier and, optionally, `.data` and a path into the data
/// of that account. Text outside placeholders is kept as is, and empty text is omitted.
///
/// Fails with `InvalidInterpolatedIntentPlaceholder` on the first placeholder that does not follow this
/// grammar, such as `${accounts.mint.lamports}`, or that is not closed.
///
/// ```
/// use codama_nodes::{parse_interpolated_intent, DataValueNode, InterpolatedIntentPart};
///
/// let parts = parse_interpolated_intent("Transfer ${data.amount}").unwrap();
/// assert_eq!(
///     parts,
///     [
///         InterpolatedIntentPart::Text("Transfer ".to_string()),
///         InterpolatedIntentPart::Data(DataValueNode::new("amount")),
///     ]
/// );
/// ```
pub fn parse_interpolated_intent(template: &str) -> CodamaResult<Vec<InterpolatedIntentPart>> {
    let mut parts = vec![];
    let mut cursor = 0;
    while let Some(offset) = template[cursor..].find(PLACEHOLDER_START) {
        let start = cursor + offset;
        if start > cursor {
            parts.push(InterpolatedIntentPart::Text(
                template[cursor..start].to_string(),
            ));
        }
        let content_start = start + PLACEHOLDER_START.len();
        let Some(end) = template[content_start..]
            .find(PLACEHOLDER_END)
            .map(|offset| content_start + offset)
        else {
            return Err(invalid_placeholder(&template[start..], template));
        };
        let placeholder = &template[start..end + PLACEHOLDER_END.len()];
        parts.push(parse_placeholder(
            &template[content_start..end],
            placeholder,
            template,
        )?);
        cursor = end + PLACEHOLDER_END.len();
    }
    if cursor < template.len() {
        parts.push(InterpolatedIntentPart::Text(template[cursor..].to_string()));
    }
    Ok(parts)
}

/// Prints the parts of an interpolated intent back into its template, the inverse of
/// [`parse_interpolated_intent`].
pub fn print_interpolated_intent(parts: &[InterpolatedIntentPart]) -> String {
    parts
        .iter()
        .map(|part| match part {
            InterpolatedIntentPart::Text(text) => text.clone(),
            InterpolatedIntentPart::Data(value) => {
                format!("${{data{}}}", print_path_continuation(&value.path))
            }
            InterpolatedIntentPart::Account(value) => format!("${{accounts.{}}}", value.identifier),
            InterpolatedIntentPart::AccountData(value) => {
                let path = value.path.as_ref().map(print_path_continuation);
                format!(
                    "${{accounts.{}.data{}}}",
                    value.account,
                    path.unwrap_or_default()
                )
            }
        })
        .collect()
}

fn parse_placeholder(
    content: &str,
    placeholder: &str,
    template: &str,
) -> CodamaResult<InterpolatedIntentPart> {
    let invalid = || invalid_placeholder(placeholder, template);
    if let Some(continuation) = content.strip_prefix("data") {
        let path = parse_path_continuation(continuation).ok_or_else(invalid)?;
        return Ok(InterpolatedIntentPart::Data(DataValueNode::new(path)));
    }
    let rest = content.strip_prefix("accounts.").ok_or_else(invalid)?;
    let identifier_end = rest
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(rest.len());
    let account = rest[..identifier_end]
        .parse::<Identifier>()
        .map_err(|_| invalid())?;
    match &rest[identifier_end..] {
        "" => Ok(InterpolatedIntentPart::Account(AccountValueNode::new(
            account,
        ))),
        ".data" => Ok(InterpolatedIntentPart::AccountData(
            AccountDataValueNode::new(account),
        )),
        attribute => {
            let continuation = attribute.strip_prefix(".data").ok_or_else(invalid)?;
            let path = parse_path_continuation(continuation).ok_or_else(invalid)?;
            Ok(InterpolatedIntentPart::AccountData(AccountDataValueNode {
                path: Some(path),
                ..AccountDataValueNode::new(account)
            }))
        }
    }
}

/// The path of a path continuation, such as `amount` for `.amount` or `[0]` for `[0]`.
fn parse_path_continuation(continuation: &str) -> Option<Path> {
    match continuation.strip_prefix('.') {
        // A field segment, so `.[0]` is not a continuation.
        Some(path) if !path.starts_with('[') => path.parse().ok(),
        Some(_) => None,
        None if continuation.starts_with('[') => continuation.parse().ok(),
        None => None,
    }
}

fn print_path_continuation(path: &Path) -> String {
    match path.as_str().starts_with('[') {
        true => path.to_string(),
        false => format!(".{path}"),
    }
}

fn invalid_placeholder(placeholder: &str, template: &str) -> CodamaError {
    CodamaError::InvalidInterpolatedIntentPlaceholder {
        placeholder: placeholder.to_string(),
        template: template.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = InstructionDisplayNode::new("Transfer");
        assert_eq!(node.intent, Some(Text::from("Transfer")));
        assert_eq!(node.interpolated_intent, None);
    }

    #[test]
    fn interpolated() {
        let node = InstructionDisplayNode::interpolated("Send {amount} to {to}");
        assert_eq!(node.intent, None);
        assert_eq!(
            node.interpolated_intent,
            Some(Text::from("Send {amount} to {to}"))
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionDisplayNode {
            intent: Some("Transfer".into()),
            interpolated_intent: Some("Send {amount} to {to}".into()),
            plugins: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionDisplayNode","intent":"Transfer","interpolatedIntent":"Send {amount} to {to}"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionDisplayNode","intent":"Transfer"}"#;
        let node: InstructionDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, InstructionDisplayNode::new("Transfer"));
    }

    fn parse(template: &str) -> Vec<InterpolatedIntentPart> {
        parse_interpolated_intent(template).unwrap()
    }

    fn text(value: &str) -> InterpolatedIntentPart {
        InterpolatedIntentPart::Text(value.to_string())
    }

    fn data(path: &str) -> InterpolatedIntentPart {
        InterpolatedIntentPart::Data(DataValueNode::new(path.parse::<Path>().unwrap()))
    }

    fn account(identifier: &str) -> InterpolatedIntentPart {
        InterpolatedIntentPart::Account(AccountValueNode::new(
            identifier.parse::<Identifier>().unwrap(),
        ))
    }

    fn account_data(identifier: &str, path: Option<&str>) -> InterpolatedIntentPart {
        InterpolatedIntentPart::AccountData(AccountDataValueNode {
            path: path.map(|path| path.parse().unwrap()),
            ..AccountDataValueNode::new(identifier.parse::<Identifier>().unwrap())
        })
    }

    #[test]
    fn parses_text_and_placeholders() {
        assert_eq!(parse("Close the account"), [text("Close the account")]);
        assert_eq!(parse(""), []);
        assert_eq!(
            parse("Transfer ${data.amount}"),
            [text("Transfer "), data("amount")]
        );
        assert_eq!(parse("${data.config.fees[0]}"), [data("config.fees[0]")]);
        assert_eq!(parse("${data[0].amount}"), [data("[0].amount")]);
        assert_eq!(
            parse("to ${accounts.destination}"),
            [text("to "), account("destination")]
        );
        assert_eq!(
            parse("${accounts.mint.data.decimals}"),
            [account_data("mint", Some("decimals"))]
        );
        assert_eq!(
            parse("${accounts.pool.data[0]}"),
            [account_data("pool", Some("[0]"))]
        );
        assert_eq!(parse("${accounts.mint.data}"), [account_data("mint", None)]);
        assert_eq!(parse("${data.a}${data.b}"), [data("a"), data("b")]);
        assert_eq!(
            parse("Pay $5 {now} ${data.amount}"),
            [text("Pay $5 {now} "), data("amount")]
        );
    }

    #[test]
    fn fails_on_invalid_placeholders() {
        for (template, placeholder) in [
            ("Send ${amount}", "${amount}"),
            ("Send ${data}", "${data}"),
            ("Send ${data.[0]}", "${data.[0]}"),
            ("Send ${data.a..b}", "${data.a..b}"),
            (
                "Send ${accounts.mint.lamports}",
                "${accounts.mint.lamports}",
            ),
            (
                "Send ${accounts.mint.database}",
                "${accounts.mint.database}",
            ),
            (
                "Send ${accounts.mint.meta.decimals}",
                "${accounts.mint.meta.decimals}",
            ),
            ("Send ${accounts}", "${accounts}"),
            ("Send ${ data.amount }", "${ data.amount }"),
            ("Send ${data.amount", "${data.amount"),
        ] {
            let error = parse_interpolated_intent(template).unwrap_err();
            assert_eq!(
                error.to_string(),
                format!("Invalid placeholder [{placeholder}] in interpolated intent [{template}]. Placeholders are `${{data.<path>}}`, `${{accounts.<account>}}` or `${{accounts.<account>.data[.<path>]}}`."),
                "{template}"
            );
        }
    }

    #[test]
    fn prints_parts_back_into_their_template() {
        let parts = [
            text("Transfer "),
            data("amount"),
            text(" to "),
            account("destination"),
            text(" of "),
            account_data("mint", Some("decimals")),
        ];
        assert_eq!(
            print_interpolated_intent(&parts),
            "Transfer ${data.amount} to ${accounts.destination} of ${accounts.mint.data.decimals}"
        );
        assert_eq!(
            print_interpolated_intent(&[data("[0].amount"), account_data("pool", Some("[1]"))]),
            "${data[0].amount}${accounts.pool.data[1]}"
        );
        assert_eq!(
            print_interpolated_intent(&[account_data("mint", None)]),
            "${accounts.mint.data}"
        );
        let template =
            "Swap ${data[0].amount} of ${accounts.pool.data.mints[1]} for ${accounts.owner}, ${data.x}$";
        assert_eq!(print_interpolated_intent(&parse(template)), template);
    }
}
