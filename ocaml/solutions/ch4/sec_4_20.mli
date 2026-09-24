(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.20: letrec as a derived expression. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** A letrec shape: the bindings and the body of one letrec form. *)
type shape = (string * Sicp_common.Ast.expr) list * Sicp_common.Ast.expr list

(** [letrec_to_lets shape] lowers one letrec shape to unassigned lets
      and assignments. *)
val letrec_to_lets : shape -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [eval_shape shape env] lowers [shape] and evaluates the result
      with the scan-out evaluator of 4.16. *)
val eval_shape
  :  shape
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_4_20 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_20 : unit -> string list
