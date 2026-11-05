use crate::{
    resolve_type_path, walk_account, walk_account_bump_value, walk_account_data_value,
    walk_account_value, walk_data_value, walk_event, walk_instruction, AccountBumpValueNode,
    AccountDataValueNode, AccountLinkResolution, AccountNode, AccountValueNode, DataValueNode,
    DiscriminatorNode, EventNode, Identifier, InstructionAccountNode, InstructionNode, Linkables,
    NodePath, NodeVisitor, Path, PathError, PathSegment, TypeNode, TypePathNode,
};

/// Node visitor that lists the path errors of a root node in `errors`, one message per error.
/// A contextual value must name an account of its closest instruction, and its data path must resolve.
/// A field discriminator must point to a struct field of the data of its owner, without crossing an account link.
/// The rules are those of the JS validator (`getContextualValueItems` and `visitFieldDiscriminator`).
/// A path that crosses a dangling link is not reported, as `ValidateLinksVisitor` reports the link.
pub struct ValidatePathsVisitor<'a> {
    linkables: &'a Linkables<'a>,
    pub errors: Vec<String>,
}

impl<'a> ValidatePathsVisitor<'a> {
    pub fn new(linkables: &'a Linkables<'a>) -> Self {
        Self {
            linkables,
            errors: Vec::new(),
        }
    }

    /// Adds one error per field discriminator whose path does not resolve to a struct field of `data`.
    /// Identification only reads the bytes being identified, so the path must not cross an account link.
    fn check_field_discriminators(
        &mut self,
        discriminators: &[DiscriminatorNode],
        data: Option<&TypeNode>,
        path: &NodePath,
    ) {
        let program = path.find_closest_identifier("programNode");
        for (index, discriminator) in discriminators.iter().enumerate() {
            let DiscriminatorNode::Field(discriminator) = discriminator else {
                continue;
            };
            let discriminator_path =
                path.join(PathSegment::Item("discriminators", index), discriminator);
            let field_path = &discriminator.path;
            let Some(data) = data else {
                let message = no_discriminator_data_message(field_path, &discriminator_path);
                self.errors.push(message);
                continue;
            };
            let Some(program) = program else {
                continue;
            };
            let resolved = resolve_type_path(
                self.linkables,
                program,
                data,
                field_path,
                AccountLinkResolution::Forbid,
            );
            let message = match resolved {
                // dangling links are reported by `ValidateLinksVisitor`, so we skip them here,
                Ok(TypePathNode::Field(_)) | Err(PathError::DanglingLink) => continue,
                Ok(TypePathNode::Type(_)) => {
                    not_a_struct_field_message(field_path, &discriminator_path)
                }
                Err(PathError::ForbiddenAccountLink) => {
                    forbidden_account_link_message(field_path, &discriminator_path)
                }
                Err(PathError::Unresolvable { segment, .. }) => {
                    unresolvable_segment_message(&segment, field_path, &discriminator_path)
                }
            };
            self.errors.push(message);
        }
    }
}

impl NodeVisitor for ValidatePathsVisitor<'_> {
    fn visit_account<'a>(&mut self, node: &'a AccountNode, path: &NodePath<'a>) {
        self.check_field_discriminators(&node.discriminators, Some(&node.data), path);
        walk_account(self, node, path);
    }

    fn visit_event<'a>(&mut self, node: &'a EventNode, path: &NodePath<'a>) {
        self.check_field_discriminators(&node.discriminators, Some(&node.data), path);
        walk_event(self, node, path);
    }

    fn visit_instruction<'a>(&mut self, node: &'a InstructionNode, path: &NodePath<'a>) {
        self.check_field_discriminators(&node.discriminators, (*node.data).as_ref(), path);
        let mut contextual_values = ContextualValuesVisitor {
            validator: self,
            instruction: node,
            program: path.find_closest_identifier("programNode"),
        };
        walk_instruction(&mut contextual_values, node, path);
    }
}

/// Walks one instruction and checks its contextual values against it.
/// The walker lends each node for one call only, so each instruction gets a new visitor that holds it.
/// A sub-instruction gets its own visitor, so a value resolves against its closest instruction, as in JS.
struct ContextualValuesVisitor<'v, 'l> {
    validator: &'v mut ValidatePathsVisitor<'l>,
    instruction: &'v InstructionNode,
    program: Option<&'v Identifier>,
}

impl<'v> ContextualValuesVisitor<'v, '_> {
    /// The account of the instruction with that exact identifier. Remaining accounts do not count, as in JS.
    fn find_instruction_account(
        &self,
        identifier: &Identifier,
    ) -> Option<&'v InstructionAccountNode> {
        let instruction = self.instruction;
        instruction
            .accounts
            .iter()
            .find(|account| &account.identifier == identifier)
    }

    /// The account of the instruction with that exact identifier, else an error.
    fn check_instruction_account(
        &mut self,
        identifier: &Identifier,
        path: &NodePath,
    ) -> Option<&'v InstructionAccountNode> {
        let account = self.find_instruction_account(identifier);
        if account.is_none() {
            let message = unknown_account_message(identifier, &self.instruction.identifier, path);
            self.validator.errors.push(message);
        }
        account
    }

    fn check_data_value(&mut self, node: &DataValueNode, path: &NodePath) {
        let instruction = self.instruction;
        let Some(data) = &*instruction.data else {
            let message = no_instruction_data_message(&node.path, &instruction.identifier, path);
            self.validator.errors.push(message);
            return;
        };
        if let Some(program) = self.program {
            self.check_data_path(program, data, &node.path, path);
        }
    }

    /// An `accountDataValueNode` reads the data of the account that its instruction account links to.
    fn check_account_data_value(&mut self, node: &AccountDataValueNode, path: &NodePath) {
        let Some(account) = self.check_instruction_account(&node.account, path) else {
            return;
        };
        let Some(account_link) = &account.account_link else {
            let message = no_account_link_message(&node.account, path);
            self.validator.errors.push(message);
            return;
        };
        let Some(data_path) = &node.path else {
            return;
        };
        let linkables = self.validator.linkables;
        let Some(program) = linkables.linked_program(account_link.program.as_ref(), self.program)
        else {
            return;
        };
        let Some(linked_account) = program.account(&account_link.identifier) else {
            return;
        };
        self.check_data_path(program.identifier(), &linked_account.data, data_path, path);
    }

    /// Adds an error when a segment of `data_path` does not apply to `data`, a type of `program`.
    /// Contextual values read account state, so a `.data` segment resolves into the data of a linked account.
    fn check_data_path(
        &mut self,
        program: &Identifier,
        data: &TypeNode,
        data_path: &Path,
        path: &NodePath,
    ) {
        let resolved = resolve_type_path(
            self.validator.linkables,
            program,
            data,
            data_path,
            AccountLinkResolution::Allow,
        );
        if let Err(PathError::Unresolvable { segment, .. }) = resolved {
            let message = unresolvable_segment_message(&segment, data_path, path);
            self.validator.errors.push(message);
        }
    }
}

impl NodeVisitor for ContextualValuesVisitor<'_, '_> {
    fn visit_instruction<'a>(&mut self, node: &'a InstructionNode, path: &NodePath<'a>) {
        self.validator.visit_instruction(node, path);
    }

    fn visit_account_value<'a>(&mut self, node: &'a AccountValueNode, path: &NodePath<'a>) {
        self.check_instruction_account(&node.identifier, path);
        walk_account_value(self, node, path);
    }

    /// The bump of an account follows the rule of the account itself.
    fn visit_account_bump_value<'a>(
        &mut self,
        node: &'a AccountBumpValueNode,
        path: &NodePath<'a>,
    ) {
        self.check_instruction_account(&node.identifier, path);
        walk_account_bump_value(self, node, path);
    }

    fn visit_account_data_value<'a>(
        &mut self,
        node: &'a AccountDataValueNode,
        path: &NodePath<'a>,
    ) {
        self.check_account_data_value(node, path);
        walk_account_data_value(self, node, path);
    }

    fn visit_data_value<'a>(&mut self, node: &'a DataValueNode, path: &NodePath<'a>) {
        self.check_data_value(node, path);
        walk_data_value(self, node, path);
    }
}

fn unknown_account_message(
    account: &Identifier,
    instruction: &Identifier,
    path: &NodePath,
) -> String {
    format!("Account \"{account}\" is not an account of instruction \"{instruction}\" in {path}.")
}

fn no_account_link_message(account: &Identifier, path: &NodePath) -> String {
    format!("Account \"{account}\" has no account link, so its data cannot be read in {path}.")
}

fn no_instruction_data_message(
    data_path: &Path,
    instruction: &Identifier,
    path: &NodePath,
) -> String {
    format!(
        "Data path \"{data_path}\" cannot be resolved as instruction \"{instruction}\" has no data in {path}."
    )
}

fn unresolvable_segment_message(segment: &str, type_path: &Path, path: &NodePath) -> String {
    format!("Cannot resolve segment \"{segment}\" of path \"{type_path}\" in {path}.")
}

fn no_discriminator_data_message(field_path: &Path, path: &NodePath) -> String {
    format!(
        "Field discriminator path \"{field_path}\" cannot be resolved as there is no data in {path}."
    )
}

fn not_a_struct_field_message(field_path: &Path, path: &NodePath) -> String {
    format!("Field discriminator path \"{field_path}\" must point to a struct field in {path}.")
}

fn forbidden_account_link_message(field_path: &Path, path: &NodePath) -> String {
    format!("Field discriminator path \"{field_path}\" must not cross an account link in {path}.")
}
