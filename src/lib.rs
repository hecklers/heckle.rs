#![feature(quote, plugin_registrar, rustc_private, plugin, convert)]
#![allow(unused_variables, unused_imports)]

extern crate syntax;
extern crate rustc;
extern crate rand;

use syntax::ext::base::{ItemModifier, ExtCtxt, Modifier};
use syntax::codemap::{Span, Spanned};
use syntax::ptr::P;
use syntax::ast::{Item, MetaItem, Expr, ExprLit, ExprIf, ExprWhile, ExprUnary, UnNot, Lit, LitBool, LitStr};
use syntax::parse::token::intern;
use syntax::fold::{Folder};
use rustc::plugin::Registry;

use rand::{thread_rng, Rng};

struct HeckleExpander;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let expander = Modifier(Box::new(HeckleExpander));
    reg.register_syntax_extension(intern("heckle"), expander);
}

impl ItemModifier for HeckleExpander {
    fn expand(&self, ecx: &mut ExtCtxt, span: Span, meta_item: &MetaItem, item: P<Item>) -> P<Item> {
        let name = option_env!("HECKLE_MUTATION").unwrap_or("invert_boolean");
        match name {
            "invert_boolean" => {
                let mut mutation = InvertBooleanMutation::new(ecx);
                mutation.fold_item(item).pop().unwrap()
            },
            "random_string" => {
                let mut mutation = RandomStringMutation::new(ecx);
                mutation.fold_item(item).pop().unwrap()
            },
            "invert_while_cond" => {
                let mut mutation = InvertWhileExprCondMutation::new(ecx);
                mutation.fold_item(item).pop().unwrap()
            },
            "invert_if_cond" => {
                let mut mutation = InvertIfExprCondMutation::new(ecx);
                mutation.fold_item(item).pop().unwrap()
            },
            _ => item
        }
    }
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
                quote_expr!(self.ecx, if !($cond) { $new_thn } else { $new_els })
            },
            ExprIf(cond, thn, None) => {
                let new_thn = self.fold_block(thn);
                quote_expr!(self.ecx, if !($cond) { $new_thn })
            }

            _ => quote_expr!(self.ecx, $expr)
        })
    }
}

struct RandomStringMutation<'a, 'b:'a> {
    ecx: &'a mut ExtCtxt<'b>
}

impl<'a, 'b:'a> Mutation<'a, 'b> for RandomStringMutation<'a, 'b> {
    fn new(ecx: &'a mut ExtCtxt<'b>) -> Self {
        RandomStringMutation {
            ecx: ecx
        }
    }
}

impl<'a, 'b> RandomStringMutation<'a, 'b> {
    fn mutate_string(&self, expr: P<Expr>) -> P<Expr> {
        let random_string: String = thread_rng().gen_ascii_chars().take(10).collect();
        let s = random_string.as_str();

        match (*expr).node {
            ExprLit(ref spanned) => match spanned.node {
                LitStr(_, str_ty) => quote_expr!(self.ecx, $s),
                _ => quote_expr!(self.ecx, $expr)
            },

            _ => quote_expr!(self.ecx, $expr)
        }
    }
}



impl<'a, 'b> Folder for RandomStringMutation<'a, 'b> {
    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        self.mutate_string(e)
    }
}

struct InvertWhileExprCondMutation<'a, 'b:'a> {
    ecx: &'a mut ExtCtxt<'b>
}

impl<'a, 'b:'a> Mutation<'a,'b> for InvertWhileExprCondMutation<'a, 'b> {
    fn new(ecx: &'a mut ExtCtxt<'b>) -> Self {
        InvertWhileExprCondMutation { ecx: ecx }
    }
}

impl<'a, 'b> Folder for InvertWhileExprCondMutation<'a, 'b> {
    fn fold_expr(&mut self, expr: P<Expr>) -> P<Expr> {
        expr.clone().and_then(|e| match e.node {
            ExprWhile(cond, block, None) => {
                let new_block = self.fold_block(block);
                quote_expr!(self.ecx, while !$cond { $new_block })
            },
            _ => quote_expr!(self.ecx, $expr)
        })
    }
}
