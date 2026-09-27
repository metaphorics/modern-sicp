(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.46: fib stack ratios across the three machines. *)

val fib_source : string
val special_controller : string
val special_at : int -> (int * int, Sicp_ch5.Sec_5_4.error) result
val compiled_at : int -> (int * int, Sicp_ch5.Sec_5_5.error) result
val interpreted_at : int -> (int * int, Sicp_ch5.Sec_5_4.error) result
val ex_5_46 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
