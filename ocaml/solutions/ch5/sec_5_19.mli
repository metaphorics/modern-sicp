(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.19: breakpoints with proceed and cancel. *)

(** [show_stop] renders how a run ended. *)
val show_stop : Sec_5_15.stop -> string

(** [ex_5_19 ()] installs the book's breakpoint before gcd's assignment
    to [a], stops at it on every pass, proceeds to the answer, cancels
    the breakpoint, and runs through. *)
val ex_5_19 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
