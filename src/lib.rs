#![feature(quote, plugin_registrar, rustc_private, plugin)]
#![allow(unused_variables, unused_imports)]

extern crate syntax;
extern crate rustc;

use syntax::ext::base::{ItemModifier, ExtCtxt, Modifier};
use syntax::codemap::{Span, Spanned};
use syntax::ptr::P;
use syntax::ast::{Item, MetaItem, Expr, ExprLit, ExprIf, ExprUnary, UnNot, Lit, LitBool, Expr_};
use syntax::parse::token::intern;
use syntax::fold::{Folder};
use rustc::plugin::Registry;

struct HeckleExpander;

impl ItemModifier for HeckleExpander {
    fn expand(&self, ecx: &mut ExtCtxt, span: Span, meta_item: &MetaItem, item: P<Item>) -> P<Item> {
        // let mut fld = InvertBooleanMutation::new(ecx);
        let mut fld = InvertIfExprCondMutation::new(ecx);
        fld.fold_item(item).pop().unwrap()
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let expander = Modifier(Box::new(HeckleExpander));
    reg.register_syntax_extension(intern("heckle"), expander);
}

trait Mutation<'a, 'b:'a> {
    fn new(ecx: &'a mut ExtCtxt<'b>) -> Self;
}

struct InvertBooleanMutation<'a, 'b:'a> {
    ecx: &'a mut ExtCtxt<'b>
}

impl<'a, 'b:'a> Mutation<'a, 'b> for InvertBooleanMutation<'a, 'b> {
    fn new(ecx: &'a mut ExtCtxt<'b>) -> Self {
        InvertBooleanMutation {
            ecx: ecx
        }
    }
}

impl<'a, 'b> InvertBooleanMutation<'a, 'b> {
    fn invert_boolean(&self, expr: P<Expr>) -> P<Expr> {
        match (*expr).node {
            ExprLit(ref spanned) => match spanned.node {
                LitBool(value) => quote_expr!(self.ecx, !$value),
                _ => quote_expr!(self.ecx, $expr)
            },

            _ => quote_expr!(self.ecx, $expr)
       }
    }
}

impl<'a, 'b> Folder for InvertBooleanMutation<'a, 'b> {
    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        self.invert_boolean(e)
    }
}

struct InvertIfExprCondMutation<'a, 'b:'a> {
    ecx: &'a mut ExtCtxt<'b>
}

impl<'a, 'b:'a> Mutation<'a, 'b> for InvertIfExprCondMutation<'a, 'b> {
    fn new(ecx: &'a mut ExtCtxt<'b>) -> Self {
        InvertIfExprCondMutation {
            ecx: ecx
        }
    }
}

impl<'a, 'b:'a> Folder for InvertIfExprCondMutation<'a, 'b> {
    fn fold_expr(&mut self, expr: P<Expr>) -> P<Expr> {
        expr.clone().and_then(|e| match e.node {
            ExprIf(cond, thn, Some(els)) => {
                let new_thn = self.fold_block(thn);
                let new_els = self.fold_expr(els);
                quote_expr!(self.ecx, if !$cond { $new_thn } else { $new_els })
            },
            ExprIf(cond, thn, None) => {
                let new_thn = self.fold_block(thn);
                quote_expr!(self.ecx, if !$cond { $new_thn })
            }

            _ => quote_expr!(self.ecx, $expr)
        })
    }
}

