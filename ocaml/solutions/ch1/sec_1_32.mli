(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program accumulate in SICP section 1.3
   exercise 1.32 *)

(** Reference solution of exercise 1.32. [accumulate] generates an
    iterative process (part b), the same tail-recursive fold as
    exercises 1.30 and 1.31. *)

val accumulate
  :  (float -> float -> float)
  -> float
  -> (float -> float)
  -> float
  -> (float -> float)
  -> float
  -> float

val sum_via_accumulate : (float -> float) -> float -> (float -> float) -> float -> float

val product_via_accumulate
  :  (float -> float)
  -> float
  -> (float -> float)
  -> float
  -> float

(** [ex_1_32 ()] is [(55., 720.)]. *)
val ex_1_32 : unit -> float * float
