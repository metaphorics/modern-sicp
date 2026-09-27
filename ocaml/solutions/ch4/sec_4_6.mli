(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.6: let as a derived expression. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [let_to_combination exp] is the equivalent application of a
      lambda. *)
val let_to_combination
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval] adds the let clause to the standard dispatch. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_06 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_06 : unit -> string list
