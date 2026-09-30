(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 5.24: the literal match (the subset's cond) as a basic controller form; and Exercise 5.24a, added by this edition: chains of && and || as basic controller forms. *)

(** Exercise 5.24: the literal-match sessions through the basic-form evaluator. *)
val ex_5_24 : unit -> (string list, Sicp_common.Eval_error.t) result

(** Exercise 5.24a, added by this edition: the && and || chain sessions through the basic-form evaluator. *)
val ex_5_24a : unit -> (string list, Sicp_common.Eval_error.t) result
