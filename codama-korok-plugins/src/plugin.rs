use codama_errors::CodamaResult;
use codama_korok_visitors::KorokVisitable;

pub trait KorokPlugin {
    fn run(
        &self,
        visitable: &mut dyn KorokVisitable,
        next: &dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>,
    ) -> CodamaResult<()>;
}

/// Reduce all plugins into a single function that runs them in sequence.
///
/// For instance, imagine we have a list of plugins [A, B, C] implemented as:
///
/// ```rust
/// use codama_errors::CodamaResult;
/// use codama_korok_plugins::KorokPlugin;
/// use codama_korok_visitors::KorokVisitable;
///
/// struct LoggingPluging;
/// impl KorokPlugin for LoggingPluging {
///     fn run(&self, visitable: &mut dyn KorokVisitable, next: &dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>) -> CodamaResult<()> {
///         println!("Plugin X - before");
///         next(visitable)?;
///         println!("Plugin X - after");
///         Ok(())
///     }
/// }
/// ```
///
/// Where `X` is `A`, `B`, or `C`. The `resolve_plugins` function will return a function that
/// prints the following:
///
/// ```text
/// Plugin C - before
/// Plugin B - before
/// Plugin A - before
/// Plugin A - after
/// Plugin B - after
/// Plugin C - after
/// ```
pub fn resolve_plugins<'a>(
    plugins: &'a [Box<dyn KorokPlugin + 'a>],
) -> Box<dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()> + 'a> {
    // We fold from the left to ensure that any code before the
    // `next` call is run before the previous plugin on the list.
    plugins.iter().fold(
        // Base case: a no-op `next` function.
        Box::new(|_: &mut dyn KorokVisitable| Ok(()))
            as Box<dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>>,
        // Wrap each plugin with a closure that calls the next plugin in the chain.
        |next, plugin| {
            Box::new(move |visitable: &mut dyn KorokVisitable| plugin.run(visitable, &next))
                as Box<dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>>
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_korok_visitors::KorokVisitor;
    use std::{cell::RefCell, sync::Arc};

    struct LoggingPluging {
        id: String,
        logs: Arc<RefCell<Vec<String>>>,
    }
    impl LoggingPluging {
        fn new(id: &str, logs: Arc<RefCell<Vec<String>>>) -> Self {
            Self {
                id: id.into(),
                logs,
            }
        }
    }
    impl KorokPlugin for LoggingPluging {
        fn run(
            &self,
            visitable: &mut dyn KorokVisitable,
            next: &dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()>,
        ) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - before", self.id));
            next(visitable)?;
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - after", self.id));
            Ok(())
        }
    }

    struct MockVisitable;
    impl KorokVisitable for MockVisitable {
        fn accept(&mut self, _visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
            Ok(())
        }
        fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
            Vec::new()
        }
    }

    #[test]
    fn test_resolve_plugins() -> CodamaResult<()> {
        let logs = Arc::new(RefCell::new(Vec::new()));
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![
            Box::new(LoggingPluging::new("A", logs.clone())),
            Box::new(LoggingPluging::new("B", logs.clone())),
        ];

        let run_plugins = resolve_plugins(&plugins);
        run_plugins(&mut MockVisitable)?;

        assert_eq!(
            logs.borrow().as_slice(),
            &[
                "Plugin B - before",
                "Plugin A - before",
                "Plugin A - after",
                "Plugin B - after",
            ]
        );
        Ok(())
    }
}
