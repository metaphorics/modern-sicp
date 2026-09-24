(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.10: a new syntax for the same evaluator. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** One term of the alternative surface syntax: a self-evaluating
      literal, a name, a function written [fn], a [do] sequence, or a
      call written [term ! args]. *)
type term =
  | Lit of Sicp_common.Value.t
  | Name of string
  | Fn of string list * term
  | Do of term list
  | Call of term * term list

(** [from_new_syntax term] translates one term into the standard
      typed AST; [eval] is the unmodified base evaluator. *)
val from_new_syntax : term -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_10 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_10 : unit -> string list
