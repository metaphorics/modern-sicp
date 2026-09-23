(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program iterative-improve in SICP section
   1.3 exercise 1.46 *)

(** Exercise 1.46: [iterative_improve] abstracts the pattern behind
    both [sqrt] (@ref{1.1.7}) and [fixed_point] (@ref{1.3.3}); addition
    1.46a returns the fixed point's successive guesses as a lazy
    [Seq.t] instead of only the final value. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [iterative_improve good_enough improve guess] returns the first
    [guess'] reached by repeated [improve] for which [good_enough
    guess'] is [true]. *)
val iterative_improve : (float -> bool) -> (float -> float) -> float -> float

(** [sqrt_via_ii x] is [Sec_1_1.Sqrt.sqrt x] (@ref{1.1.7}), rewritten
    in terms of [iterative_improve]. *)
val sqrt_via_ii : float -> float

(** [fixed_point_via_ii f guess] is [Sec_1_3.Fixed_point.fixed_point f
    guess] (@ref{1.3.3}), rewritten in terms of [iterative_improve]. *)
val fixed_point_via_ii : (float -> float) -> float -> float

(** [ex_1_46 ()] is [(sqrt_via_ii 9., fixed_point_via_ii cos 1.)]. *)
val ex_1_46 : unit -> float * float

(** [sqrt_guesses x] is the infinite lazy sequence of successive
    average-damped guesses at [sqrt x], starting from [1.]; this is
    addition 1.46a. *)
val sqrt_guesses : float -> float Seq.t

(** [ex_1_46a ()] is the first 5 elements of [sqrt_guesses 2.],
    forced. *)
val ex_1_46a : unit -> float list
