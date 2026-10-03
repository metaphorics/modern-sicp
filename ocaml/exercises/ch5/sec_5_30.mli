(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 5.30: errors signaled inside the evaluator -- runtime failures of primitives and operators become condition codes routed to signal-error, and admission rejects unbound variables and wrong operand counts. *)

(** Exercise 5.30: the caught failures and one clean computation. *)
val ex_5_30 : unit -> (string list, Sicp_common.Eval_error.t) result
