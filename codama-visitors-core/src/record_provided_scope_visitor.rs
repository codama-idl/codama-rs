use crate::{ProvidedScope, Visitor};
use codama_nodes::NodeRef;

/// Wraps a visitor so that a [`ProvidedScope`] holds the `provides` of the instructions enclosing the
/// node being visited.
///
/// Created by [`VisitorExt::record_provided_scope`](crate::VisitorExt::record_provided_scope). Each
/// `instructionNode` with `provides` opens a frame for the duration of its visit, so the injections
/// within it, and within its sub-instructions, resolve against the instructions that enclose them.
///
/// Adapters that wrap this one run outside of the frame, so wrap a visitor with it after the extensions
/// that resolve values from the scope, as they need the frame of the instruction being visited.
/// Otherwise, their injections resolve as if no instruction provided them.
#[derive(Debug, Clone, Copy)]
pub struct RecordProvidedScopeVisitor<'s, 'a, V> {
    visitor: V,
    scope: &'s ProvidedScope<'a>,
}

impl<'s, 'a, V> RecordProvidedScopeVisitor<'s, 'a, V> {
    /// Wraps `visitor` to record into `scope`.
    /// Prefer [`VisitorExt::record_provided_scope`](crate::VisitorExt::record_provided_scope).
    pub fn new(visitor: V, scope: &'s ProvidedScope<'a>) -> Self {
        Self { visitor, scope }
    }
}

impl<'a, T, V> Visitor<'a, T> for RecordProvidedScopeVisitor<'_, 'a, V>
where
    V: Visitor<'a, T>,
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        let NodeRef::Instruction(instruction) = node else {
            return self.visitor.visit(node, recurse);
        };
        if instruction.provides.is_empty() {
            return self.visitor.visit(node, recurse);
        }
        self.scope.push(&instruction.provides);
        let _pop = PopFrameOnDrop(self.scope);
        self.visitor.visit(node, recurse)
    }
}

/// Closes the innermost frame when dropped, so a frame is closed even when its visit panics.
struct PopFrameOnDrop<'s, 'a>(&'s ProvidedScope<'a>);

impl Drop for PopFrameOnDrop<'_, '_> {
    fn drop(&mut self) {
        self.0.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::{visit, ProvidedScope, VisitorExt, VoidVisitor};
    use codama_nodes::{
        Identifier, InjectedValueNode, InstructionNode, IntegerValueNode, NodeRef, ProvidedNode,
        StructTypeNode, ValueNode,
    };
    use std::cell::RefCell;

    fn decimals() -> InjectedValueNode {
        InjectedValueNode::new("decimals")
    }

    #[test]
    fn opens_the_frame_of_an_instruction_for_the_duration_of_its_visit() {
        let instruction = InstructionNode {
            identifier: "myInstruction".into(),
            provides: vec![ProvidedNode::new("decimals", IntegerValueNode::new(6))],
            ..InstructionNode::default()
        };
        let injected = decimals();
        let scope = ProvidedScope::default();
        let resolved = RefCell::new(vec![]);
        let visitor = VoidVisitor
            .tap(|node| {
                if let NodeRef::Instruction(_) = node {
                    resolved
                        .borrow_mut()
                        .push(scope.resolve::<ValueNode>(&injected).unwrap());
                }
            })
            .record_provided_scope(&scope);
        visit(&visitor, &instruction);
        assert_eq!(*resolved.borrow(), [Some(IntegerValueNode::new(6).into())]);
        assert_eq!(scope.get(&Identifier::from("decimals")), None);
    }

    #[test]
    fn lets_a_sub_instruction_shadow_its_parent() {
        let instruction = InstructionNode {
            identifier: "parent".into(),
            provides: vec![ProvidedNode::new("decimals", IntegerValueNode::new(6))],
            sub_instructions: vec![InstructionNode {
                identifier: "child".into(),
                provides: vec![ProvidedNode::new("decimals", IntegerValueNode::new(9))],
                ..InstructionNode::default()
            }],
            ..InstructionNode::default()
        };
        let injected = decimals();
        let scope = ProvidedScope::default();
        let resolved = RefCell::new(vec![]);
        let visitor = VoidVisitor
            .extend(|node, visit_scope| {
                visit_scope.next(node);
                if let NodeRef::Instruction(instruction) = node {
                    let value = scope.resolve::<ValueNode>(&injected).unwrap();
                    resolved
                        .borrow_mut()
                        .push((instruction.identifier.to_string(), value));
                }
            })
            .record_provided_scope(&scope);
        visit(&visitor, &instruction);
        assert_eq!(
            *resolved.borrow(),
            [
                ("child".to_string(), Some(IntegerValueNode::new(9).into())),
                ("parent".to_string(), Some(IntegerValueNode::new(6).into())),
            ]
        );
    }

    #[test]
    fn only_opens_frames_for_instructions_that_provide_values() {
        let scope = ProvidedScope::default();
        let depths = RefCell::new(vec![]);
        let visitor = VoidVisitor
            .tap(|_| depths.borrow_mut().push(scope.depth()))
            .record_provided_scope(&scope);
        let r#struct = StructTypeNode::new(vec![]);
        let instruction = InstructionNode {
            identifier: "myInstruction".into(),
            data: Box::new(Some(StructTypeNode::new(vec![]).into())),
            ..InstructionNode::default()
        };
        visit(&visitor, &r#struct);
        visit(&visitor, &instruction);
        assert!(depths.borrow().iter().all(|depth| *depth == 0));
    }
}
