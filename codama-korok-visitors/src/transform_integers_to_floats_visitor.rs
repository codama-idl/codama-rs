use crate::KorokVisitor;
use codama_errors::CodamaResult;
use codama_koroks::RootKorok;
use codama_nodes::{
    CamelCaseString, ConstantPdaSeedValue, ConstantValueNode, CountNode, DiscriminatorNode,
    FloatValueNode, HasTransforms, InstructionInputValueNode, InstructionNode, Node, PdaNode,
    PdaSeedNode, PdaSeedValueValue, PdaValueNode, PdaValuePda, ProgramLinkNode, ProgramNode,
    RegisteredContextualValueNode, RootNode, StructFieldTypeNode, TransformNode, TypeNode,
    ValueNode,
};

/// Turns each integer value whose type resolves to a float type into a float value with the same digits.
/// It covers:
/// - field defaults
/// - constants in types and programs
/// - constant discriminators of accounts, instructions and events
/// - constant PDA seeds
/// - the PDA seed values of instruction account defaults and `provides`.
///
/// Types resolve through defined type links and PDA links, which are complete only on the combined root node.
/// A dangling link, an unknown program, PDA or seed, and a cycle of links keep the integer.
/// Values inside other values (array, set, some, map, struct, tuple, enum payload, injected fallback)
/// and display values keep their kind.
#[derive(Default)]
pub struct TransformIntegersToFloatsVisitor;

impl TransformIntegersToFloatsVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for TransformIntegersToFloatsVisitor {
    fn visit_root(&mut self, korok: &mut RootKorok) -> CodamaResult<()> {
        if let Some(Node::Root(root)) = &mut korok.node {
            transform_integers_to_floats(root);
        }
        Ok(())
    }
}

fn transform_integers_to_floats(root: &mut RootNode) {
    // The pass changes values only, never types or identifiers, so links resolve in this copy.
    let programs: Vec<ProgramNode> = std::iter::once(&root.program)
        .chain(&root.additional_programs)
        .cloned()
        .collect();
    let targets = std::iter::once(&mut root.program).chain(&mut root.additional_programs);

    for (program, lookup) in targets.zip(&programs) {
        let scope = Scope {
            programs: &programs,
            program: lookup,
        };
        scope.walk_program(program);
    }
}

/// The programs of the root, and the program that links without a `program` point into.
#[derive(Clone, Copy)]
struct Scope<'a> {
    programs: &'a [ProgramNode],
    program: &'a ProgramNode,
}

impl<'a> Scope<'a> {
    fn walk_program(self, program: &mut ProgramNode) {
        for account in &mut program.accounts {
            self.walk_type(&mut account.data);
            self.walk_discriminators(&mut account.discriminators);
        }
        for defined_type in &mut program.defined_types {
            self.walk_type(&mut defined_type.r#type);
        }
        for event in &mut program.events {
            self.walk_type(&mut event.data);
            self.walk_discriminators(&mut event.discriminators);
        }
        for pda in &mut program.pdas {
            self.walk_pda(pda);
        }
        for constant in &mut program.constants {
            self.walk_typed_value(&mut constant.r#type, &mut constant.value);
        }
        for instruction in &mut program.instructions {
            self.walk_instruction(instruction);
        }
    }

    /// Walks a type tree, so field defaults and constants at any depth take their types.
    fn walk_type(self, r#type: &mut TypeNode) {
        self.walk_transforms(r#type.transforms_mut());
        match r#type {
            TypeNode::Array(node) => {
                self.walk_type(&mut node.item);
                self.walk_count(&mut node.count);
            }
            TypeNode::Boolean(node) => self.walk_transforms(&mut node.size.transforms),
            TypeNode::DateTime(node) => self.walk_transforms(&mut node.number.transforms),
            TypeNode::Duration(node) => self.walk_transforms(&mut node.number.transforms),
            TypeNode::Enum(node) => {
                self.walk_transforms(&mut node.size.transforms);
                for variant in &mut node.variants {
                    if let Some(data) = &mut *variant.data {
                        self.walk_type(data);
                    }
                }
            }
            TypeNode::FixedPoint(node) => self.walk_transforms(&mut node.number.transforms),
            TypeNode::Map(node) => {
                self.walk_type(&mut node.key);
                self.walk_type(&mut node.value);
                self.walk_count(&mut node.count);
            }
            TypeNode::Option(node) => {
                self.walk_type(&mut node.item);
                self.walk_transforms(&mut node.prefix.transforms);
            }
            TypeNode::RemainderOption(node) => self.walk_type(&mut node.item),
            TypeNode::Set(node) => {
                self.walk_type(&mut node.item);
                self.walk_count(&mut node.count);
            }
            TypeNode::Struct(node) => {
                for field in &mut node.fields {
                    self.walk_field(field);
                }
            }
            TypeNode::Tuple(node) => {
                for item in &mut node.items {
                    self.walk_type(item);
                }
            }
            TypeNode::ZeroableOption(node) => {
                self.walk_type(&mut node.item);
                if let Some(zero_value) = &mut node.zero_value {
                    self.walk_constant(zero_value);
                }
            }
            TypeNode::Bytes(_)
            | TypeNode::Float(_)
            | TypeNode::Integer(_)
            | TypeNode::Link(_)
            | TypeNode::PublicKey(_)
            | TypeNode::String(_) => {}
        }
    }

    fn walk_pda(self, pda: &mut PdaNode) {
        for seed in &mut pda.seeds {
            match seed {
                PdaSeedNode::Constant(seed) => {
                    self.walk_type(&mut seed.r#type);
                    if let ConstantPdaSeedValue::Integer(integer) = seed.value.as_ref() {
                        *seed.value = self
                            .value_node_for_type(integer.clone().into(), &seed.r#type)
                            .into();
                    }
                }
                PdaSeedNode::Variable(seed) => self.walk_type(&mut seed.r#type),
            }
        }
    }

    fn walk_instruction(self, instruction: &mut InstructionNode) {
        for account in &mut instruction.accounts {
            if let Some(InstructionInputValueNode::PdaValue(value)) = &mut *account.default_value {
                self.walk_pda_value(value);
            }
        }
        if let Some(data) = &mut *instruction.data {
            self.walk_type(data);
        }
        if let Some(return_data) = &mut *instruction.return_data {
            self.walk_type(return_data);
        }
        self.walk_discriminators(&mut instruction.discriminators);
        for provided in &mut instruction.provides {
            if let Node::ContextualValue(RegisteredContextualValueNode::Pda(value)) =
                &mut *provided.node
            {
                self.walk_pda_value(value);
            }
        }
        for sub_instruction in &mut instruction.sub_instructions {
            self.walk_instruction(sub_instruction);
        }
    }

    fn walk_typed_value(self, r#type: &mut TypeNode, value: &mut ValueNode) {
        self.walk_type(r#type);
        *value = self.value_node_for_type(value.clone(), r#type);
    }

    fn walk_count(self, count: &mut CountNode) {
        match count {
            CountNode::Prefixed(node) => self.walk_transforms(&mut node.prefix.transforms),
            CountNode::Sentinel(node) => self.walk_constant(&mut node.sentinel),
            CountNode::Fixed(_) | CountNode::Remainder(_) => {}
        }
    }

    fn walk_field(self, field: &mut StructFieldTypeNode) {
        self.walk_type(&mut field.r#type);
        if let Some(value) = &mut *field.default_value {
            *value = self.value_node_for_type(value.clone(), &field.r#type);
        }
    }

    /// A constant value takes its own type.
    fn walk_constant(self, constant: &mut ConstantValueNode) {
        self.walk_typed_value(&mut constant.r#type, &mut constant.value);
    }

    fn walk_discriminators(self, discriminators: &mut [DiscriminatorNode]) {
        for discriminator in discriminators {
            if let DiscriminatorNode::Constant(node) = discriminator {
                self.walk_constant(&mut node.constant);
            }
        }
    }

    /// Seed values take the type of the variable seed with the same identifier, in the program of the PDA.
    fn walk_pda_value(self, value: &mut PdaValueNode) {
        if let PdaValuePda::Pda(pda) = &mut *value.pda {
            self.walk_pda(pda);
        }
        let Some((scope, pda)) = self.resolve_pda(&value.pda) else {
            return;
        };
        for seed in &mut value.seeds {
            let PdaSeedValueValue::Integer(integer) = seed.value.as_ref() else {
                continue;
            };
            let r#type = pda.seeds.iter().find_map(|candidate| match candidate {
                PdaSeedNode::Variable(candidate) if candidate.identifier == seed.identifier => {
                    Some(&candidate.r#type)
                }
                _ => None,
            });
            if let Some(r#type) = r#type {
                *seed.value = scope
                    .value_node_for_type(integer.clone().into(), r#type)
                    .into();
            }
        }
    }

    fn walk_transforms(self, transforms: &mut [TransformNode]) {
        for transform in transforms {
            match transform {
                TransformNode::HiddenPrefix(node) => {
                    for constant in &mut node.prefix {
                        self.walk_constant(constant);
                    }
                }
                TransformNode::HiddenSuffix(node) => {
                    for constant in &mut node.suffix {
                        self.walk_constant(constant);
                    }
                }
                TransformNode::Sentinel(node) => self.walk_constant(&mut node.sentinel),
                TransformNode::SizePrefix(node) => {
                    self.walk_transforms(&mut node.prefix.transforms)
                }
                TransformNode::FixedSize(_)
                | TransformNode::PostOffset(_)
                | TransformNode::PreOffset(_) => {}
            }
        }
    }

    /// An integer value whose type resolves to a float type becomes a float value with the same digits.
    /// Other values, and values whose type does not resolve, stay as written.
    fn value_node_for_type(self, value: ValueNode, r#type: &TypeNode) -> ValueNode {
        match (value, self.resolve_type(r#type)) {
            (ValueNode::Integer(integer), Some(TypeNode::Float(_))) => FloatValueNode {
                value: integer.value.into(),
                plugins: integer.plugins,
            }
            .into(),
            (value, _) => value,
        }
    }

    /// The scope of a link: the first root program with the linked identifier, or this program.
    fn get_scope_for_program_link(self, program: Option<&ProgramLinkNode>) -> Option<Self> {
        let Some(program) = program else {
            return Some(self);
        };
        let program = self
            .programs
            .iter()
            .find(|candidate| candidate.identifier == program.identifier)?;
        Some(Self { program, ..self })
    }

    /// The type behind a chain of defined type links. A dangling link or a cycle gives `None`.
    fn resolve_type<'b>(self, mut r#type: &'b TypeNode) -> Option<&'b TypeNode>
    where
        'a: 'b,
    {
        let mut scope = self;
        let mut followed: Vec<(&CamelCaseString, &CamelCaseString)> = Vec::new();
        while let TypeNode::Link(link) = r#type {
            scope = scope.get_scope_for_program_link(link.program.as_ref())?;
            let key = (&scope.program.identifier, &link.identifier);
            if followed.contains(&key) {
                return None;
            }
            followed.push(key);
            let defined_type = scope
                .program
                .defined_types
                .iter()
                .find(|candidate| candidate.identifier == link.identifier)?;
            r#type = &defined_type.r#type;
        }
        Some(r#type)
    }

    /// The PDA of a PDA value, and the scope its seed types resolve in.
    fn resolve_pda<'b>(self, pda: &'b PdaValuePda) -> Option<(Scope<'b>, &'b PdaNode)>
    where
        'a: 'b,
    {
        match pda {
            PdaValuePda::Pda(pda) => Some((self, pda)),
            PdaValuePda::PdaLink(link) => {
                let scope = self.get_scope_for_program_link(link.program.as_ref())?;
                let pda = scope
                    .program
                    .pdas
                    .iter()
                    .find(|candidate| candidate.identifier == link.identifier)?;
                Some((scope, pda))
            }
        }
    }
}
