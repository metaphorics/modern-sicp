(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.7 and this edition's 5.7a: the Exercise 5.4 machines on
    the section's simulator, and each simulated run paired with a
    direct host computation as its oracle. *)

(** [ex_5_07 ()] runs the recursive and iterative exponentiation
    machines of Exercise 5.4 on (2, 10) and (3, 5). *)
val ex_5_07 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result

(** [ex_5_07a ()] is this edition's addition: for every [n] in 0..6
    the simulated Fibonacci and factorial runs appear beside their
    direct host computations, [ok] when they agree. *)
val ex_5_07a : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
