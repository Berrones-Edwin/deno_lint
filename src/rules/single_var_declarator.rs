// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.
use super::{Context, LintRule, DUMMY_NODE};
use crate::ProgramRef;
use deno_ast::swc::ast::VarDecl;
use deno_ast::swc::visit::noop_visit_type;
use deno_ast::swc::visit::Node;
use deno_ast::swc::visit::Visit;
use derive_more::Display;

pub struct SingleVarDeclarator;

const CODE: &str = "single-var-declarator";

#[derive(Display)]
enum SingleVarDeclaratorMessage {
  #[display(fmt = "Multiple variable declarators are not allowed")]
  Unexpected,
}

impl LintRule for SingleVarDeclarator {
  fn new() -> Box<Self> {
    Box::new(SingleVarDeclarator)
  }

  fn code(&self) -> &'static str {
    CODE
  }

  fn lint_program<'view>(
    &self,
    context: &mut Context<'view>,
    program: ProgramRef<'view>,
  ) {
    let mut visitor = SingleVarDeclaratorVisitor::new(context);
    match program {
      ProgramRef::Module(m) => visitor.visit_module(m, &DUMMY_NODE),
      ProgramRef::Script(s) => visitor.visit_script(s, &DUMMY_NODE),
    }
  }

  #[cfg(feature = "docs")]
  fn docs(&self) -> &'static str {
    include_str!("../../docs/rules/single_var_declarator.md")
  }
}

struct SingleVarDeclaratorVisitor<'c, 'view> {
  context: &'c mut Context<'view>,
}

impl<'c, 'view> SingleVarDeclaratorVisitor<'c, 'view> {
  fn new(context: &'c mut Context<'view>) -> Self {
    Self { context }
  }
}

impl<'c, 'view> Visit for SingleVarDeclaratorVisitor<'c, 'view> {
  noop_visit_type!();

  fn visit_var_decl(&mut self, var_decl: &VarDecl, _parent: &dyn Node) {
    if var_decl.decls.len() > 1 {
      self.context.add_diagnostic(
        var_decl.span,
        CODE,
        SingleVarDeclaratorMessage::Unexpected,
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn single_var_declarator_invalid() {
    assert_lint_err! {
      SingleVarDeclarator,
      r#"const a1 = "a", b1 = "b", c1 = "c";"#: [
      {
        col: 0,
        message: SingleVarDeclaratorMessage::Unexpected,
      }],
      r#"let a2 = "a", b2 = "b", c2 = "c";"#: [
      {
        col: 0,
        message: SingleVarDeclaratorMessage::Unexpected,
      }],
      r#"var a3 = "a", b3 = "b", c3 = "c";"#: [
      {
        col: 0,
        message: SingleVarDeclaratorMessage::Unexpected,
      }],
    }
  }
}
