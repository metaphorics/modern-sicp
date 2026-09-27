(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program product in SICP section 1.3
   exercise 1.31 *)

(** Exercise 1.31: [product], the multiplicative analog of [sum];
    [factorial] and a Wallis-product approximation to pi built on it.
    The stubs raise [Sicp_common.Pending.Pending_solution] until they
    are solved. *)

(** [product term a next b] is [term a * term (next a) * ... * term k]
    for the largest [k] the [next] chain reaches without passing [b];
    the empty product (when [a > b]) is 1. *)
val product : (float -> float) -> float -> (float -> float) -> float -> float

(** [factorial n] is [product] applied to the identity term over
    [1 .. n]. *)
val factorial : int -> float

(** [pi_approx n] is 4 times the [n]-term Wallis product
    [2*4*4*6*6*8*.../3*3*5*5*7*7*...], via [product]. *)
val pi_approx : int -> float

(** [ex_1_31 ()] is [(factorial 6, pi_approx 1000)]. *)
val ex_1_31 : unit -> float * float
