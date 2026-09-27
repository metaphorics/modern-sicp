(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.9: a while construct as a derived expression. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** A while shape: the test and the body of one while form. *)
type shape = Sicp_common.Ast.expr * Sicp_common.Ast.expr list

(** [while_to_combination shape] lowers one while shape to a
      definition and a call. *)
val while_to_combination
  :  shape
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval_while shape env] lowers and evaluates one while. *)
val eval_while
  :  shape
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_09 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_09 : unit -> string list
