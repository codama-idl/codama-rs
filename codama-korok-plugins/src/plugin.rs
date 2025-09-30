use codama_errors::CodamaResult;
use codama_korok_visitors::KorokVisitable;

pub trait KorokPlugin {
    fn on_initialized(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }

    fn on_fields_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }

    fn on_program_items_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }

    fn on_root_node_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }
}

pub type ResolvePluginsResult<'a> = Box<dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()> + 'a>;

/// Combine all plugins into a single function that runs them in sequence.
pub fn resolve_plugins<'a>(plugins: &'a [Box<dyn KorokPlugin + 'a>]) -> ResolvePluginsResult<'a> {
    Box::new(move |visitable: &mut dyn KorokVisitable| {
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_initialized(visitable))?;
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_fields_set(visitable))?;
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_program_items_set(visitable))?;
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_root_node_set(visitable))?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_korok_visitors::KorokVisitor;
    use std::{cell::RefCell, rc::Rc};

    struct LoggingPluging {
        id: String,
        logs: Rc<RefCell<Vec<String>>>,
    }
    impl LoggingPluging {
        fn new(id: &str, logs: Rc<RefCell<Vec<String>>>) -> Self {
            Self {
                id: id.into(),
                logs,
            }
        }
    }
    impl KorokPlugin for LoggingPluging {
        fn on_initialized(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - initialized", self.id));
            Ok(())
        }
        fn on_fields_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - on_fields_set", self.id));
            Ok(())
        }
        fn on_program_items_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - on_program_items_set", self.id));
            Ok(())
        }
        fn on_root_node_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - on_root_node_set", self.id));
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
        let logs = Rc::new(RefCell::new(Vec::new()));
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![
            Box::new(LoggingPluging::new("A", logs.clone())),
            Box::new(LoggingPluging::new("B", logs.clone())),
        ];

        let run_plugins = resolve_plugins(&plugins);
        run_plugins(&mut MockVisitable)?;

        assert_eq!(
            logs.borrow().as_slice(),
            &[
                "Plugin A - initialized",
                "Plugin B - initialized",
                "Plugin A - on_fields_set",
                "Plugin B - on_fields_set",
                "Plugin A - on_program_items_set",
                "Plugin B - on_program_items_set",
                "Plugin A - on_root_node_set",
                "Plugin B - on_root_node_set",
            ]
        );
        Ok(())
    }
}
