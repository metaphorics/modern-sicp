(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.8 *)

(** Exercise 4.8: named [let]. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** A named [let]: the procedure name, the bindings, and the body.  The
    body may refer to the name and to every binding name. *)
type named_let =
  { name : string
  ; bindings : (string * Sicp_common.Ast.expr) list
  ; body : Sicp_common.Ast.expr
  }

(** The two [let] forms [let_to_combination] accepts: an ordinary
    non-recursive [let] of the checked syntax, or a named [let]. *)
type let_form =
  | Plain of Sicp_common.Ast.expr
  | Named of named_let

(** [let_to_combination form] extends [Sec_4_6.let_to_combination] to
    named [let]: the procedure is bound recursively and applied to the
    inits, which run in the enclosing scope.  A named [let] with no
    bindings takes one unit parameter applied to [()]. *)
val let_to_combination
  :  let_form
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval_let_form form env] evaluates [let_to_combination form] with
    [Sec_4_6.eval]. *)
val eval_let_form
  :  let_form
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [fib ()] is the book's iterative Fibonacci procedure,
    [fun n -> ] a named [let] [fib_iter] over [a = 1], [b = 0],
    [count = n]. *)
val fib : unit -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [ex_4_08 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_08 : unit -> (string list, Sicp_common.Eval_error.t) result
