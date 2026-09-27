(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.7: let* as nested lets. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** A let* shape: the bindings and the body of one let* form. *)
type shape = (string * Sicp_common.Ast.expr) list * Sicp_common.Ast.expr list

(** [let_star_to_nested_lets shape] lowers one let* shape to nested
      lets of the typed subset. *)
val let_star_to_nested_lets
  :  shape
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval_shape shape env] lowers [shape] and evaluates the result. *)
val eval_shape
  :  shape
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_07 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_07 : unit -> string list
