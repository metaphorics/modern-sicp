(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.9 *)

(** Exercise 4.9: iteration constructs as derived expressions. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** An iteration construct. *)
type loop =
  | While of Sicp_common.Ast.expr * Sicp_common.Ast.expr
  (** [While (test, body)] runs [body] as long as [test] is [true],
      testing before each pass, and answers [()]. *)
  | For of string * Sicp_common.Ast.expr * Sicp_common.Ast.expr * Sicp_common.Ast.expr
  (** [For (var, from, upto, body)] runs [body] with [var] bound to each
      integer from [from] up to [upto] inclusive and answers [()].  The
      bounds are evaluated once, [from] after [upto]. *)

(** [loop_to_expr loop] is the recursive procedure and its first call
    equivalent to [loop].  The procedure's names cannot be spelled in
    source, so they never capture a variable of the body. *)
val loop_to_expr : loop -> Sicp_common.Ast.expr

(** [eval_loop eval loop env] evaluates [loop_to_expr loop] with
    [eval]. *)
val eval_loop
  :  Sicp_ch4.Sec_4_1.eval_t
  -> loop
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [summation ()] sums the integers from 1 to 10 into a reference with
    a [While] loop and answers the total. *)
val summation : unit -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [ex_4_09 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_09 : unit -> string list
