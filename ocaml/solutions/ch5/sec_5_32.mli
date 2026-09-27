(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.32: the evaluator's symbol-operator fast path. *)

(** The modified application fragments. *)
val ev_application_fast : string

val controller : string
val symbol_operator_op : string * Sicp_ch5.Sec_5_4.op
val run : string -> (string list, Sicp_ch5.Sec_5_4.error) result
val ex_5_32 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
