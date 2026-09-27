(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.6: the redundant save/restore pair, removed and proved
    by counts. *)

(** [ex_5_06 ()] runs the original and modified Fibonacci machines on
    10 -- the answers agree -- and reports the transcription counts of
    [fib 6] before and after, steps and saves. *)
val ex_5_06 : unit -> (string list, Sicp_ch5.Sec_5_1.error) result

(** The Figure 5.12 controller with the pair removed. *)
val fib_modified_controller : string
