use crate::{LogLevel, ValidationItem};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::NodeRef;
use codama_visitors_core::{NodePath, Visitor, VisitorExt};

/// Wraps a visitor that lists [`ValidationItem`]s so it fails when the visited node has items of
/// `throw_level` or above, like `throwValidatorItemsVisitor` in the JavaScript library.
///
/// The error is [`CodamaError::FailedToValidateNode`], whose message lists the number of items of each
/// level and then every item, from the most to the least severe. As it maps the value of the visited
/// node, wrap a visitor with it last.
///
/// ```
/// use codama_nodes::{ProgramNode, RootNode};
/// use codama_validators::{get_validation_items_visitor, throw_validator_items_visitor, LogLevel};
/// use codama_visitors_core::visit;
///
/// let root = RootNode::new(ProgramNode {
///     version: String::new(),
///     ..ProgramNode::new("token", "")
/// });
/// let visitor = throw_validator_items_visitor(get_validation_items_visitor(), LogLevel::Error);
/// let error = visit(&visitor, &root).unwrap_err();
/// assert_eq!(
///     error.to_string(),
///     concat!(
///         "Failed to validate node [rootNode] [errors: 1, warns: 1].\n",
///         "- [error] Program has no public key. at [programNode]token\n",
///         "- [warn] Program has no version. at [programNode]token",
///     )
/// );
/// ```
pub fn throw_validator_items_visitor<'a, V>(
    visitor: V,
    throw_level: LogLevel,
) -> impl Visitor<'a, CodamaResult<()>>
where
    V: Visitor<'a, Vec<ValidationItem<'a>>>,
{
    visitor.map(move |items: Vec<ValidationItem<'a>>, node: NodeRef<'a>| {
        let Some(max_level) = items.iter().map(|item| item.level).max() else {
            return Ok(());
        };
        if max_level < throw_level {
            return Ok(());
        }
        let mut items = items;
        // A stable sort keeps the order of the items of each level.
        items.sort_by(|a, b| b.level.cmp(&a.level));
        let mut histogram: Vec<(LogLevel, usize)> = vec![];
        for item in &items {
            match histogram.last_mut() {
                Some((level, count)) if *level == item.level => *count += 1,
                _ => histogram.push((item.level, 1)),
            }
        }
        Err(CodamaError::FailedToValidateNode {
            path: NodePath::from(vec![node]).to_string(),
            histogram: histogram
                .iter()
                .map(|(level, count)| format!("{level}s: {count}"))
                .collect::<Vec<_>>()
                .join(", "),
            items: items.iter().map(ToString::to_string).collect(),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation_item;
    use codama_nodes::{ProgramNode, RootNode};
    use codama_visitors_core::{visit, MergeVisitor};

    /// A visitor reporting the given items about the program of the root it visits.
    fn reporting<'a>(
        levels: &'static [LogLevel],
        root: &'a RootNode,
    ) -> impl Visitor<'a, Vec<ValidationItem<'a>>> {
        MergeVisitor::new(
            move |node: NodeRef<'a>, _: Vec<Vec<ValidationItem<'a>>>| match node {
                NodeRef::Root(_) => levels
                    .iter()
                    .map(|level| {
                        let path =
                            NodePath::from(vec![NodeRef::from(root), NodeRef::from(&root.program)]);
                        validation_item(*level, format!("Something at {level} level."), path)
                    })
                    .collect(),
                _ => vec![],
            },
        )
    }

    #[test]
    fn fails_with_the_path_of_the_validated_node() {
        let root = RootNode::new(ProgramNode::new("myProgram", "1111"));
        let visitor =
            throw_validator_items_visitor(reporting(&[LogLevel::Error], &root), LogLevel::Error);
        let error = visit(&visitor, &root).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Failed to validate node [rootNode] [errors: 1].\n- [error] Something at error level. at [programNode]myProgram"
        );
    }

    #[test]
    fn does_not_fail_on_items_below_the_given_level() {
        let root = RootNode::new(ProgramNode::new("myProgram", "1111"));
        let levels = &[LogLevel::Debug, LogLevel::Info, LogLevel::Warn];
        let visitor = throw_validator_items_visitor(reporting(levels, &root), LogLevel::Error);
        assert!(visit(&visitor, &root).is_ok());
        assert!(visit(
            &throw_validator_items_visitor(reporting(&[], &root), LogLevel::Debug),
            &root
        )
        .is_ok());
    }

    #[test]
    fn lists_the_items_from_the_most_to_the_least_severe() {
        let root = RootNode::new(ProgramNode::new("myProgram", "1111"));
        let levels = &[
            LogLevel::Info,
            LogLevel::Error,
            LogLevel::Info,
            LogLevel::Warn,
        ];
        let visitor = throw_validator_items_visitor(reporting(levels, &root), LogLevel::Warn);
        let error = visit(&visitor, &root).unwrap_err();
        let CodamaError::FailedToValidateNode {
            histogram, items, ..
        } = error
        else {
            panic!("expected a validation failure");
        };
        assert_eq!(histogram, "errors: 1, warns: 1, infos: 2");
        assert_eq!(
            items.iter().map(|item| &item[..7]).collect::<Vec<_>>(),
            ["[error]", "[warn] ", "[info] ", "[info] "]
        );
    }
}
