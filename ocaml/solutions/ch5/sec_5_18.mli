(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.18: register tracing. *)

(** The recursive factorial machine of Figure 5.11. *)
val factorial_controller : string

(** [ex_5_18 ()] traces [n] and [val] through the factorial machine on
    [n = 3]: one report line per traced write, then the answer. *)
val ex_5_18 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
