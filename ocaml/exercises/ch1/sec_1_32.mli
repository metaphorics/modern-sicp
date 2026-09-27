(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program accumulate in SICP section 1.3
   exercise 1.32 *)

(** Exercise 1.32: [accumulate] generalizes [sum] and [product] with an
    explicit combiner and null value. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [accumulate combiner null_value term a next b] combines [term a],
    [term (next a)], ..., left to right, into [null_value] with
    [combiner]; the empty range (when [a > b]) is [null_value]. *)
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

(** [ex_1_32 ()] is [(sum_via_accumulate identity 1. inc 10.,
    product_via_accumulate identity 1. inc 6.)]. *)
val ex_1_32 : unit -> float * float
