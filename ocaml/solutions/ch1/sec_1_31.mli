(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program product in SICP section 1.3
   exercise 1.31 *)

(** Reference solution of exercise 1.31. [product] generates an
    iterative process (part b): the accumulation is a plain
    tail-recursive fold, the same shape as exercise 1.30's
    [sum_iterative]. *)

val product : (float -> float) -> float -> (float -> float) -> float -> float
val factorial : int -> float
val pi_approx : int -> float

(** [ex_1_31 ()] is [(720., 3.143160705532257)]: [factorial 6] exact,
    and [pi_approx 1000] converging toward pi from above, slowly, as
    the book's own [pi_sum] does from below. *)
val ex_1_31 : unit -> float * float
