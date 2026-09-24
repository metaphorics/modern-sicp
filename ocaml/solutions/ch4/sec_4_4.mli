(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.4: and and or as special forms and as derived expressions. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [eval] carries the direct clauses; [and_to_if] and [or_to_if]
      are the derived-expression rewrites. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

val and_to_if
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

val or_to_if
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [ex_4_04 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_04 : unit -> string list
