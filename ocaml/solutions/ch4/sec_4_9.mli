(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.9 *)

(** Exercise 4.9: iteration constructs as derived expressions.

    The subset rejects OCaml's native [while] and [for] loops, so the
    two constructs here are extension nodes beside the checked syntax.
    Each derives into a recursive procedure: iteration gains
    convenience, not computational power. *)

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

(** [ex_4_09 ()] runs the summation, a [For] loop printing the squares
    from 1 to 5, a [While] loop whose test is false at once, and then
    the native [while] and [for] loops, which admission rejects.  Each
    line is the printed output followed by the value or the
    diagnostic. *)
val ex_4_09 : unit -> string list
