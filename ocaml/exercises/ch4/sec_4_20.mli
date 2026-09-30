(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.20 *)

(** Exercise 4.20: [let rec] as a derived expression. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [letrec_to_lets e] is the derived form of the [let rec] group [e],
    or an [Invalid_form] error when [e] is not a [let rec]. *)
val letrec_to_lets
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval] is the evaluator that derives every [let rec] with
    [letrec_to_lets] and reports a read of the unassigned marker as an
    error. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [parity binder] is the source of Louis's example, [f 5], whose
    internal group is written [let binder ... and ...]. *)
val parity : string -> string

(** [ex_4_20 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_20 : unit -> string list
