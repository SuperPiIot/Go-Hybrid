use std::collections::HashMap;

// Adicionar uma estrutura para representar regiões transacionais
pub struct TransactionalRegion {
    start: usize,
    end: usize,
}

impl Checker {
    pub fn check_transactional_region(&mut self, region: &TransactionalRegion) {
        // Verificar que todas as operações dentro da região são transacionais
        for stmt in &self.file.stmts[region.start..region.end] {
            match stmt {
                Stmt::Local(l) => self.check_local(l),
                Stmt::Assign { lhs, rhs, .. } => {
                    if let Expr::Ident { name, .. } = lhs.as_ref() {
                        if !self.transactional_cells.contains_key(name) {
                            self.emit_diagnostic(Diagnostic::new(
                                Severity::Error,
                                format!("assignment to non-transactional cell: {}", name),
                                stmt.span(),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}