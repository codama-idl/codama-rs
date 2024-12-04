use codama_korok_visitors::KorokVisitable;

pub trait KorokPlugin {
    fn run(&self, visitable: &mut dyn KorokVisitable, next: &dyn Fn(&mut dyn KorokVisitable));
}

/// Reduce all plugins into a single function that runs them in sequence.
///
/// For instance, imagine we have a list of plugins [A, B, C] implemented as:
///
/// ```rust
/// fn run(&self, visitable: &mut dyn KorokVisitable, next: &dyn Fn(&mut dyn KorokVisitable)) {
///     println!("Plugin X - before");
///     next(visitable);
///     println!("Plugin X - after");
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
pub fn resolve_plugins(plugins: Vec<Box<dyn KorokPlugin>>) -> Box<dyn Fn(&mut dyn KorokVisitable)> {
    // We fold from the right to ensure that any code before the
    // `next` call is run before the previous plugin on the list.
    plugins.into_iter().rfold(
        // Base case: a no-op `next` function.
        Box::new(|_: &mut dyn KorokVisitable| {}) as Box<dyn Fn(&mut dyn KorokVisitable)>,
        // Wrap each plugin with a closure that calls the next plugin in the chain.
        |next, plugin| {
            Box::new(move |visitable: &mut dyn KorokVisitable| {
                plugin.run(visitable, &next);
            }) as Box<dyn Fn(&mut dyn KorokVisitable)>
        },
    )
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use codama_korok_visitors::KorokVisitor;

    struct LoggingPluging {
        id: String,
        logs: RefCell<Vec<String>>,
    }
    impl LoggingPluging {
        fn new(id: &str, logs: RefCell<Vec<String>>) -> Self {
            Self {
                id: id.into(),
                logs,
            }
        }
    }
    impl KorokPlugin for LoggingPluging {
        fn run(&self, visitable: &mut dyn KorokVisitable, next: &dyn Fn(&mut dyn KorokVisitable)) {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - before", self.id));
            next(visitable);
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - after", self.id));
        }
    }

    struct MockVisitable;
    impl KorokVisitable for MockVisitable {
        fn accept(&mut self, _visitor: &mut dyn KorokVisitor) {}
        fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
            Vec::new()
        }
    }

    #[test]
    fn test_resolve_plugins() {
        let logs = RefCell::new(Vec::new());
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![
            Box::new(LoggingPluging::new("A", logs.clone())),
            Box::new(LoggingPluging::new("B", logs.clone())),
        ];

        let run_plugins = resolve_plugins(plugins);
        run_plugins(&mut MockVisitable);

        assert_eq!(
            logs.borrow().as_slice(),
            &[
                "Plugin B - before",
                "Plugin A - before",
                "Plugin A - after",
                "Plugin B - after",
            ]
        );
    }
}
