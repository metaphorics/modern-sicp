(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.17: the trace announces each instruction's labels. *)

(** [trace_line m i] renders one traced instruction: the labels the
    assembler retained for it, then the instruction itself. *)
val trace_line : Sec_5_15.Monitor.machine -> int -> string

(** [ex_5_17 ()] traces the Fibonacci machine on [n = 3] with the
    labels printed and the count undisturbed. *)
val ex_5_17 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
