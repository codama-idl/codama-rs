---
default: major
---

#### Walk koroks with `walk_*` functions, like `syn::visit_mut`

Every `KorokVisitor` method now defaults to the matching `walk_*` function, such as `walk_struct(visitor, korok)`, which a visitor handling a korok itself calls to keep walking. `KorokVisitor::visit_children` and `KorokVisitable::get_children`, which walked a parallel list of children, are removed; `walk_korok_mut` walks the children of any `KorokMut`. `KorokMut` gains the `Impl` variant it lacked, so `UniformVisitor` and `DebugVisitor` visit `impl` blocks, and `ComposeVisitor` forwards every method to its visitors, including `visit_const` and `visit_type_alias`, which it used to walk on its own. The `KorokPlugin` trait and its phases are documented.
