use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CalleeSlotSource<'c> {
    Arg(&'c Expr),
    OuterBinding(String),
}

// Adicionar após a definição de CalleeSlotSource
enum TransactionalSlotSource<'c> {
    Arg(&'c Expr),
    OuterBinding(String),
}

// Adicionar função para resolver transações
impl<'a> Checker<'a> {
    fn resolve_transaction(&self, st: &FnState, cr: &CallRef) -> Option<String> {
        match cr.callee {
            Expr::Field { base, name, .. } => {
                if let Some(root) = base.root_ident() {
                    let recv = self.shape_of_expr(st, base);
                    if let TypeShape::Transactional(_) = recv {
                        return Some(root.to_string());
                    }
                }
            }
            _ => {}
        }
        None
    }
}