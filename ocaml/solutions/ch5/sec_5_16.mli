(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.16: instruction tracing, on and off. *)

(** [ex_5_16 ()] runs the GCD machine on (12, 8) with tracing on --
    one line per executed instruction -- then again with tracing off
    and shows the silence. *)
val ex_5_16 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
