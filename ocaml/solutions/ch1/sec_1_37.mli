(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cont-frac in SICP section 1.3
   exercise 1.37 *)

(** Reference solution of exercise 1.37. [cont_frac] generates an
    iterative process (part b): building the fraction from the
    innermost term outward is a plain right-to-left fold. *)

val cont_frac : (int -> float) -> (int -> float) -> int -> float
val smallest_k_for_4_decimal_places : unit -> int

(** [ex_1_37 ()] is [(0.6180555555555556, 11)]. *)
val ex_1_37 : unit -> float * int
