(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cont-frac in SICP section 1.3
   exercise 1.38 *)

(** Reference solution of exercise 1.38. *)

val euler_d : int -> float
val e_approx : int -> float

(** [ex_1_38 ()] is [2.718281828459045], equal to [Float.exp 1.] at
    the precision OCaml's float prints. *)
val ex_1_38 : unit -> float
