(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sum in SICP section 1.3
   exercise 1.30 *)

(** Reference solution of exercise 1.30. *)

val sum_iterative : (float -> float) -> float -> (float -> float) -> float -> float

(** [ex_1_30 ()] is [55.]: the sum of the integers 1 through 10. *)
val ex_1_30 : unit -> float
