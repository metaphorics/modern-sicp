(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 5.30: errors signaled inside the evaluator -- unbound variables and primitive failures become condition codes routed to signal-error. *)

(** Exercise 5.30: the caught failures and one clean computation. *)
val ex_5_30 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
