use crate::KorokVisitor;
use codama_errors::CodamaResult;
use codama_koroks::KorokTrait;

/// Construct an indented debug string representation of the koroks visited.
#[derive(Default)]
pub struct DebugVisitor {
    current_result: String,
    current_indent: usize,
}

impl DebugVisitor {
    const INDENT: &'static str = "|   ";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_result(self) -> String {
        self.current_result
    }

    pub fn clear(&mut self) {
        self.current_result.clear();
        self.current_indent = 0;
    }

    fn write(
        &mut self,
        identifier: &str,
        options: Option<&str>,
        mut korok: codama_koroks::KorokMut,
    ) -> CodamaResult<()> {
        self.write_indent();
        self.current_result.push_str(identifier);
        if let Some(text) = options {
            self.current_result.push_str(&format!(" ({})", text));
        }

        let json = serde_json::to_string(&korok.node())?;
        self.current_result.push_str(&format!(": {}\n", json));

        self.current_indent += 1;
        self.visit_children(&mut korok)?;
        self.current_indent -= 1;
        Ok(())
    }

    fn write_indent(&mut self) {
        self.current_result
            .push_str(&Self::INDENT.repeat(self.current_indent));
    }
}

impl KorokVisitor for DebugVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        self.write("Root", None, korok.into())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.write(
            "Crate",
            Some(&korok.store.path.display().to_string()),
            korok.into(),
        )
    }

    fn visit_item(&mut self, korok: &mut codama_koroks::ItemKorok) -> CodamaResult<()> {
        self.write("Item", None, korok.into())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.write(
            "FileModule",
            Some(&format!(
                "{} -> {}",
                korok.ast.ident,
                korok.store.path.display(),
            )),
            korok.into(),
        )
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.write("Module", Some(&korok.ast.ident.to_string()), korok.into())
    }

    fn visit_struct(&mut self, korok: &mut codama_koroks::StructKorok) -> CodamaResult<()> {
        self.write("Struct", Some(&korok.ast.ident.to_string()), korok.into())
    }

    fn visit_enum(&mut self, korok: &mut codama_koroks::EnumKorok) -> CodamaResult<()> {
        self.write("Enum", Some(&korok.ast.ident.to_string()), korok.into())
    }

    fn visit_enum_variant(
        &mut self,
        korok: &mut codama_koroks::EnumVariantKorok,
    ) -> CodamaResult<()> {
        self.write(
            "EnumVariant",
            Some(&korok.ast.ident.to_string()),
            korok.into(),
        )
    }

    fn visit_const(&mut self, korok: &mut codama_koroks::ConstKorok) -> CodamaResult<()> {
        let ident = match korok.ast {
            codama_koroks::ConstAst::Item(item) => &item.ident,
            codama_koroks::ConstAst::ImplItem(item) => &item.ident,
        };

        self.write("Const", Some(&ident.to_string()), korok.into())
    }

    fn visit_unsupported_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedItemKorok,
    ) -> CodamaResult<()> {
        self.write("UnsupportedItem", None, korok.into())
    }

    fn visit_impl_item(&mut self, korok: &mut codama_koroks::ImplItemKorok) -> CodamaResult<()> {
        self.write("ImplItem", None, korok.into())
    }

    fn visit_unsupported_impl_item(
        &mut self,
        korok: &mut codama_koroks::UnsupportedImplItemKorok,
    ) -> CodamaResult<()> {
        self.write("UnsupportedImplItem", None, korok.into())
    }

    fn visit_fields(&mut self, korok: &mut codama_koroks::FieldsKorok) -> CodamaResult<()> {
        self.write("Fields", None, korok.into())
    }

    fn visit_field(&mut self, korok: &mut codama_koroks::FieldKorok) -> CodamaResult<()> {
        let ident = korok
            .ast
            .ident
            .as_ref()
            .map_or("None".to_string(), |i| i.to_string());

        self.write("Field", Some(&ident), korok.into())
    }
}
