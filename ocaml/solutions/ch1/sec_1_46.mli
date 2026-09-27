(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program iterative-improve in SICP section
   1.3 exercise 1.46 *)

(** Reference solution of exercise 1.46 and addition 1.46a. *)

val iterative_improve : (float -> bool) -> (float -> float) -> float -> float
val sqrt_via_ii : float -> float
val fixed_point_via_ii : (float -> float) -> float -> float

(** [ex_1_46 ()] is [(3.00009155413138, 0.7390893414033927)]. *)
val ex_1_46 : unit -> float * float

val sqrt_guesses : float -> float Seq.t

(** [ex_1_46a ()] is
    [[1.; 1.5; 1.4166666666666665; 1.4142156862745097; 1.4142135623746899]]. *)
val ex_1_46a : unit -> float list
