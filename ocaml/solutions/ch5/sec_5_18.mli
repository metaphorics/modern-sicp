(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.18: register tracing. *)

(** [ex_5_18 ()] traces [n] and [val] through the Figure 5.11 factorial
    machine on [n = 3]: one report line per traced write -- the input
    load, the assignments, and the restores -- then the answer. *)
val ex_5_18 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
