use codama_nodes::test_utils::id;
use codama_nodes::{
    walk_struct_type, AccountNode, ArrayTypeNode, BooleanTypeNode, BooleanValueNode, BytesTypeNode,
    ConstantNode, DateTimeTypeNode, DefinedTypeLinkNode, DefinedTypeNode, Docs, DurationTypeNode,
    EnumTypeNode, EnumVariantTypeNode, ErrorNode, EventNode, FixedCountNode, FixedPointTypeNode,
    FixedSizeTransformNode, FloatTypeNode, HasKind, HasTransforms, InstructionAccountNode,
    InstructionByteDeltaNode, InstructionNode, InstructionRemainingAccountsNode,
    InstructionStatusNode, IntegerTypeNode, IntegerValueNode, MapTypeNode, Namespace, Node,
    NodePath, NodeTrait, NodeVisitor, OptionTypeNode, PathSegment, PayerValueNode, PdaLinkNode,
    PdaNode, PluginNode, ProgramLinkNode, ProgramNode, ProvidedNode, PublicKeyTypeNode,
    RegisteredTypeNode, RemainderOptionTypeNode, RootNode, SetTypeNode, SizeDiscriminatorNode,
    StringDisplayNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, Text, TextNode,
    TupleTypeNode, TypeNode, VariablePdaSeedNode, ZeroableOptionTypeNode, F64, U64,
};
use pretty_assertions::assert_eq;
use serde_json::Value;
use PathSegment::{Attribute, Item};

fn namespace(value: &str) -> Namespace {
    value.parse().unwrap()
}

#[derive(Default)]
struct Recorder {
    visits: Vec<(&'static str, Vec<PathSegment>)>,
}

impl NodeVisitor for Recorder {
    fn enter_node<'a>(&mut self, kind: &'static str, path: &NodePath<'a>) {
        self.visits
            .push((kind, path.steps().iter().map(|step| step.segment).collect()));
    }
}

fn record(walk: impl FnOnce(&mut Recorder)) -> Vec<(&'static str, Vec<PathSegment>)> {
    let mut recorder = Recorder::default();
    walk(&mut recorder);
    recorder.visits
}

#[test]
fn struct_type_node() {
    let node = StructTypeNode::new(vec![
        StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64)),
        StructFieldTypeNode::new(id("label"), StringTypeNode::utf8()),
    ]);
    assert_eq!(
        record(|visitor| visitor.visit_struct_type(&node, &NodePath::default())),
        vec![
            ("structTypeNode", vec![]),
            ("structFieldTypeNode", vec![Item("fields", 0)]),
            (
                "integerTypeNode",
                vec![Item("fields", 0), Attribute("type")]
            ),
            ("structFieldTypeNode", vec![Item("fields", 1)]),
            ("stringTypeNode", vec![Item("fields", 1), Attribute("type")]),
        ]
    );
}

#[test]
fn option_type_node_with_prefix() {
    let node =
        OptionTypeNode::new(StringTypeNode::utf8()).add_transform(FixedSizeTransformNode::new(4));
    assert_eq!(
        record(|visitor| visitor.visit_option_type(&node, &NodePath::default())),
        vec![
            ("optionTypeNode", vec![]),
            ("stringTypeNode", vec![Attribute("item")]),
            ("integerTypeNode", vec![Attribute("prefix")]),
            ("fixedSizeTransformNode", vec![Item("transforms", 0)]),
        ]
    );
}

#[test]
fn pda_link_node_with_program() {
    let node = PdaLinkNode::new_from_program(id("vault"), ProgramLinkNode::new(id("token")));
    assert_eq!(
        record(|visitor| visitor.visit_pda_link(&node, &NodePath::default())),
        vec![
            ("pdaLinkNode", vec![]),
            ("programLinkNode", vec![Attribute("program")]),
        ]
    );
}

#[test]
fn text_docs_and_plugins() {
    let docs = TextNode {
        content: "A config.".to_string(),
        plugins: vec![PluginNode::new(namespace("i18n.es"))],
    };
    let node = DefinedTypeNode {
        docs: Docs::from(Text::from(docs)),
        plugins: vec![PluginNode::new(namespace("anchor"))],
        ..DefinedTypeNode::new(id("config"), StringTypeNode::utf8())
    };
    assert_eq!(
        record(|visitor| visitor.visit_defined_type(&node, &NodePath::default())),
        vec![
            ("definedTypeNode", vec![]),
            ("textNode", vec![Attribute("docs")]),
            ("pluginNode", vec![Attribute("docs"), Item("plugins", 0)]),
            ("stringTypeNode", vec![Attribute("type")]),
            ("pluginNode", vec![Item("plugins", 0)]),
        ]
    );
}

type Step = (PathSegment, &'static str, Option<String>);

#[derive(Default)]
struct FirstNode(Option<(&'static str, Vec<Step>)>);

impl NodeVisitor for FirstNode {
    fn enter_node<'a>(&mut self, kind: &'static str, path: &NodePath<'a>) {
        self.0.get_or_insert_with(|| {
            let steps = path.steps().iter().map(|step| {
                let identifier = step.identifier.map(ToString::to_string);
                (step.segment, step.kind, identifier)
            });
            (kind, steps.collect())
        });
    }
}

const SEGMENT: PathSegment = Item("items", 2);
const NAME: &str = "name";

fn assert_dispatch<T: HasKind>(unnamed: &[T], named: &[T], visit: impl Fn(&mut FirstNode, &T)) {
    let unnamed = unnamed.iter().map(|node| (node, None));
    let named = named.iter().map(|node| (node, Some(NAME.to_string())));
    let (visited, expected): (Vec<_>, Vec<_>) = unnamed
        .chain(named)
        .map(|(node, identifier)| {
            let mut visitor = FirstNode::default();
            visit(&mut visitor, node);
            let step = (SEGMENT, node.kind(), identifier);
            (visitor.0, Some((node.kind(), vec![step])))
        })
        .unzip();
    assert_eq!(visited, expected);
}

fn standalone_type_nodes() -> Vec<TypeNode> {
    vec![
        ArrayTypeNode::new(StringTypeNode::utf8(), FixedCountNode::new(2)).into(),
        BooleanTypeNode::default().into(),
        BytesTypeNode::new().into(),
        DateTimeTypeNode::new(IntegerTypeNode::le(U64)).into(),
        DurationTypeNode::new(IntegerTypeNode::le(U64)).into(),
        EnumTypeNode::new(vec![]).into(),
        FixedPointTypeNode::new(IntegerTypeNode::le(U64), 2).into(),
        FloatTypeNode::le(F64).into(),
        IntegerTypeNode::le(U64).into(),
        MapTypeNode::remainder(StringTypeNode::utf8(), StringTypeNode::utf8()).into(),
        OptionTypeNode::new(StringTypeNode::utf8()).into(),
        PublicKeyTypeNode::new().into(),
        RemainderOptionTypeNode::new(StringTypeNode::utf8()).into(),
        SetTypeNode::new(StringTypeNode::utf8(), FixedCountNode::new(2)).into(),
        StringTypeNode::utf8().into(),
        StructTypeNode::new(vec![]).into(),
        TupleTypeNode::new(vec![]).into(),
        ZeroableOptionTypeNode::new(StringTypeNode::utf8()).into(),
    ]
}

#[test]
fn walk_type_joins_the_node_inside_each_variant() {
    let named = [TypeNode::from(DefinedTypeLinkNode::new(id(NAME)))];
    assert_dispatch(&standalone_type_nodes(), &named, |visitor, node| {
        visitor.visit_type(node, &NodePath::default(), SEGMENT)
    });
}

#[test]
fn walk_registered_type_joins_the_node_inside_each_variant() {
    let unnamed: Vec<RegisteredTypeNode> = standalone_type_nodes()
        .into_iter()
        .map(|node| node.try_into().unwrap())
        .collect();
    let named: Vec<RegisteredTypeNode> = vec![
        EnumVariantTypeNode::new(id(NAME)).into(),
        StructFieldTypeNode::new(id(NAME), IntegerTypeNode::le(U64)).into(),
    ];
    assert_dispatch(&unnamed, &named, |visitor, node| {
        visitor.visit_registered_type(node, &NodePath::default(), SEGMENT)
    });
}

#[test]
fn walk_node_joins_the_node_inside_each_variant() {
    let unnamed: Vec<Node> = vec![
        PayerValueNode::new().into(),
        FixedCountNode::new(2).into(),
        SizeDiscriminatorNode::new(8).into(),
        StringDisplayNode::default().into(),
        FixedSizeTransformNode::new(4).into(),
        StringTypeNode::utf8().into(),
        BooleanValueNode::new(true).into(),
        InstructionByteDeltaNode::new(IntegerValueNode::new(1), false).into(),
        InstructionStatusNode::default().into(),
        PluginNode::new(namespace("anchor")).into(),
        RootNode::default().into(),
        TextNode::default().into(),
    ];
    let instruction = InstructionNode {
        identifier: id(NAME),
        ..InstructionNode::default()
    };
    let remaining_accounts = InstructionRemainingAccountsNode {
        identifier: id(NAME),
        ..InstructionRemainingAccountsNode::default()
    };
    let named: Vec<Node> = vec![
        ProgramLinkNode::new(id(NAME)).into(),
        VariablePdaSeedNode::new(id(NAME), PublicKeyTypeNode::new()).into(),
        AccountNode::new(id(NAME), StructTypeNode::new(vec![])).into(),
        ConstantNode::new(id(NAME), IntegerTypeNode::le(U64), IntegerValueNode::new(1)).into(),
        DefinedTypeNode::new(id(NAME), StringTypeNode::utf8()).into(),
        ErrorNode::new(id(NAME), 1, "Overflow.").into(),
        EventNode::new(id(NAME), StructTypeNode::new(vec![])).into(),
        instruction.into(),
        InstructionAccountNode::new(id(NAME), true, true).into(),
        remaining_accounts.into(),
        PdaNode::new(id(NAME), vec![]).into(),
        ProgramNode::new(id(NAME), "").into(),
        ProvidedNode::new(id(NAME), PayerValueNode::new()).into(),
    ];
    assert_dispatch(&unnamed, &named, |visitor, node| {
        visitor.visit_node(node, &NodePath::default(), SEGMENT)
    });
}

#[derive(Default)]
struct DataPaths(Vec<String>);

impl NodeVisitor for DataPaths {
    fn visit_struct_type(&mut self, node: &StructTypeNode, path: &NodePath) {
        self.0.push(path.to_string());
        walk_struct_type(self, node, path);
    }
}

#[test]
fn path_names_the_nodes_with_an_identifier() {
    let data = StructTypeNode::new(vec![StructFieldTypeNode::new(
        id("amount"),
        StructTypeNode::new(vec![]),
    )]);
    let instruction = InstructionNode {
        identifier: id("transfer"),
        data: Box::new(Some(data.into())),
        ..InstructionNode::default()
    };
    let program = ProgramNode {
        instructions: vec![instruction],
        ..ProgramNode::new(id("token"), "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
    };
    let mut visitor = DataPaths::default();
    visitor.visit_root(&RootNode::new(program), &NodePath::default());
    assert_eq!(
        visitor.0,
        vec![
            "program \"token\" > instruction \"transfer\" > data",
            "program \"token\" > instruction \"transfer\" > data > structFieldType \"amount\" > type",
        ]
    );
}

#[derive(Default)]
struct FieldSkipper(Vec<&'static str>);

impl NodeVisitor for FieldSkipper {
    fn enter_node<'a>(&mut self, kind: &'static str, _: &NodePath<'a>) {
        self.0.push(kind);
    }

    fn visit_struct_field_type<'a>(&mut self, node: &'a StructFieldTypeNode, _: &NodePath<'a>) {
        self.0.push(node.kind());
    }
}

#[test]
fn an_override_without_walk_skips_the_subtree() {
    let field = StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64));
    let node = StructTypeNode::new(vec![field]).add_transform(FixedSizeTransformNode::new(32));
    let mut visitor = FieldSkipper::default();
    visitor.visit_struct_type(&node, &NodePath::default());
    assert_eq!(
        visitor.0,
        vec![
            "structTypeNode",
            "structFieldTypeNode",
            "fixedSizeTransformNode"
        ]
    );
}

fn e2e_idl(name: &str) -> String {
    let path = format!(
        "{}/../codama/tests/{name}/mod.rs",
        env!("CARGO_MANIFEST_DIR")
    );
    let source = std::fs::read_to_string(path).unwrap();
    let start = source.find("r#\"").unwrap() + "r#\"".len();
    let end = start + source[start..].find("\"#").unwrap();
    source[start..end].to_string()
}

fn json_kinds<'a>(value: &'a Value, kinds: &mut Vec<&'a str>) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(kind)) = map.get("kind") {
                kinds.push(kind);
            }
            map.values().for_each(|value| json_kinds(value, kinds));
        }
        Value::Array(items) => items.iter().for_each(|item| json_kinds(item, kinds)),
        _ => {}
    }
}

#[test]
fn walks_every_node_of_the_e2e_idls() {
    for name in [
        "float_values",
        "membership",
        "memo",
        "program_override",
        "system",
    ] {
        let json = e2e_idl(name);
        let root = RootNode::from_json(&json).unwrap();
        let visits = record(|recorder| {
            let visitor: &mut dyn NodeVisitor = recorder;
            visitor.visit_root(&root, &NodePath::default());
        });
        let mut visited: Vec<&str> = visits.into_iter().map(|(kind, _)| kind).collect();
        let value: Value = serde_json::from_str(&json).unwrap();
        let mut expected = Vec::new();
        json_kinds(&value, &mut expected);
        visited.sort();
        expected.sort();
        assert_eq!(visited, expected, "{name}");
    }
}
