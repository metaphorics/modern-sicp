(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.8: named let. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [named_let_to_combination name bindings body] lowers one named
      let to a definition and a call. *)
val named_let_to_combination
  :  string
  -> (string * Sicp_common.Ast.expr) list
  -> Sicp_common.Ast.expr list
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval_named name bindings body env] lowers and evaluates one
      named let. *)
val eval_named
  :  string
  -> (string * Sicp_common.Ast.expr) list
  -> Sicp_common.Ast.expr list
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_08 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_08 : unit -> string list
