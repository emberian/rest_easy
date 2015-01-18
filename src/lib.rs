#![feature(plugin_registrar, plugin)]

#[plugin] #[macro_use]
extern crate syntax;
#[plugin] #[macro_use]
extern crate rustc;


use syntax::ast;
use rustc::lint::{Context, LintPass, LintArray};
use rustc::lint::LintPassObject;
use rustc::plugin::Registry;

declare_lint!(REST_EASY, Warn,
              "Tell you when trans starts.");

struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(REST_EASY)
    }

    fn check_crate(&mut self, ctx: &Context, _: &ast::Crate) {
        ctx.tcx.sess.note("trans starting, rest easy");
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(Box::new(Pass) as LintPassObject);
}
