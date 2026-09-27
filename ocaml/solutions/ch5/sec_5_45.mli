(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.45: stack ratios of compiled, interpreted, and
    special-purpose factorial runs. *)

val parse_stats : string -> (int * int) option
val compiled_at : int -> (int * int, Sicp_ch5.Sec_5_5.error) result
val interpreted_at : int -> (int * int, Sicp_ch5.Sec_5_4.error) result
val special_at : int -> (int * int, Sicp_ch5.Sec_5_4.error) result
val ex_5_45 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
