// SPDX-License-Identifier: GPL-3.0-only

//! Format-string checking for the three admitted macros (grammar §4):
//! Rust's compile-time checked format strings, restricted to the
//! `{}` display and `{:?}` debug placeholders plus `{{` and `}}`
//! escapes. Everything else is `Unsupported`, never a runtime failure.

use crate::host::ast::{self, FormatKind};
use crate::host::check::Checker;
use crate::host::diag::{Diag, Span};
use crate::host::hir::{FormatSpec, HirExpr, HirExprKind};

impl Checker {
    /// Checks a `format!`, `print!`, or `println!` invocation.
    pub(crate) fn check_format(
        &mut self,
        kind: FormatKind,
        fmt: &str,
        args: &[ast::Expr],
        span: Span,
    ) -> Result<HirExpr, Diag> {
        let spec = parse_spec(fmt, span)?;
        if spec.debug.len() != args.len() {
            return Err(Diag::type_error(
                span,
                format!(
                    "the format string names {} arguments but {} were given",
                    spec.debug.len(),
                    args.len()
                ),
            ));
        }
        let mut checked = Vec::with_capacity(args.len());
        for (arg, debug) in args.iter().zip(&spec.debug) {
            let want = self.fresh_cell(false);
            let expr = self.check_expr(arg, Some(&want))?;
            let got = self.deep(&expr.ty);
            let admitted = if *debug {
                self.debug_able(&got)
            } else {
                display_able(&got)
            };
            if !admitted {
                return Err(Diag::type_error(
                    arg.span,
                    if *debug {
                        format!("`{got:?}` has no admitted `Debug` rendering")
                    } else {
                        format!("`{got:?}` has no admitted `Display` rendering")
                    },
                ));
            }
            checked.push(expr);
        }
        let ty = match kind {
            FormatKind::Format => crate::host::hir::HostTy::String,
            FormatKind::Print | FormatKind::Println => crate::host::hir::HostTy::Unit,
        };
        Ok(HirExpr {
            kind: HirExprKind::Format {
                kind,
                spec,
                args: checked,
            },
            ty,
            diverges: false,
            span,
        })
    }

    fn debug_able(&self, ty: &crate::host::hir::HostTy) -> bool {
        use crate::host::hir::HostTy;
        match ty {
            HostTy::Infer(_) => true,
            HostTy::Struct(id) | HostTy::Enum(id) => {
                self.item_has_derive(*id, ast::DeriveName::Debug)
            }
            HostTy::Ref(_, inner)
            | HostTy::Option(inner)
            | HostTy::Box(inner)
            | HostTy::Vec(inner)
            | HostTy::HashMap(inner)
            | HostTy::Array(inner, _) => self.debug_able(inner),
            HostTy::Result(ok, err) => self.debug_able(ok) && self.debug_able(err),
            HostTy::Tuple(left, right) => self.debug_able(left) && self.debug_able(right),
            other => display_able(other) || matches!(other, HostTy::Unit),
        }
    }
}

/// Whether a type has an admitted display rendering.
fn display_able(ty: &crate::host::hir::HostTy) -> bool {
    use crate::host::hir::HostTy;
    match ty {
        HostTy::Ref(_, inner) => display_able(inner),
        HostTy::I64
        | HostTy::Usize
        | HostTy::Bool
        | HostTy::Str
        | HostTy::String
        | HostTy::Infer(_) => true,
        _ => false,
    }
}

/// Parses the admitted format-string surface into alternating literal
/// pieces and placeholder kinds.
fn parse_spec(fmt: &str, span: Span) -> Result<FormatSpec, Diag> {
    let mut pieces = Vec::new();
    let mut debug = Vec::new();
    let mut literal = String::new();
    let mut chars = fmt.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                literal.push('{');
            }
            '{' => {
                let mut close = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some(c) => close.push(c),
                        None => {
                            return Err(Diag::unsupported(span, "unterminated format placeholder"));
                        }
                    }
                }
                let is_debug = match close.as_str() {
                    "" => false,
                    "?" => true,
                    _ => {
                        return Err(Diag::unsupported(
                            span,
                            format!("format specifier `{{{close}}}` is outside the subset"),
                        ));
                    }
                };
                pieces.push(std::mem::take(&mut literal));
                debug.push(is_debug);
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                literal.push('}');
            }
            '}' => {
                return Err(Diag::unsupported(span, "stray `}` in a format string"));
            }
            other => literal.push(other),
        }
    }
    pieces.push(literal);
    Ok(FormatSpec { pieces, debug })
}
