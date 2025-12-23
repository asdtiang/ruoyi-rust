use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{parse_quote, visit, visit_mut, Expr};


pub struct ReplacePoolSt {
    pub tx: syn::Ident,
}

impl VisitMut for ReplacePoolSt {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Macro(expr_macro) = node {
            if expr_macro.mac.path.is_ident("pool") {
                // 替换节点
                let tx=self.tx.clone();
                *node = parse_quote!(#tx);
            }
        }
        // 继续正常遍历
        visit_mut::visit_expr_mut(self, node);
    }
}

pub struct HasPoolSt ;

impl Visit<'_>  for HasPoolSt {
    fn visit_expr(&mut self, node: &Expr) {
        if let Expr::Macro(expr_macro) = node {
            if expr_macro.mac.path.is_ident("pool") {
                panic!("函数里面还有pool!()引用")
            }
        }
        // 继续正常遍历
        visit::visit_expr(self, node);
    }
}
