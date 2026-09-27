(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program iterative-improve in SICP section
   1.3 exercise 1.46 *)

(** Exercise 1.46: [iterative_improve]'s [go] tests the *current*
    guess, not the gap to the next one, so it can settle one step
    earlier than [Sec_1_3.Fixed_point.fixed_point]'s own [try_guess]
    (which compares consecutive guesses); the two rewritten searches
    below can therefore land a step apart from the section's originals
    on the same input, which is a property of the two stopping tests,
    not a bug. Addition 1.46a: [sqrt_guesses] builds the same
    average-damped improvement as [sqrt_via_ii] but as an unbounded
    [Seq.t], so a caller can take as many or as few guesses as wanted
    without [iterative_improve] committing to a stopping rule at all. *)

let rec iterative_improve good_enough improve guess =
  if good_enough guess
  then guess
  else iterative_improve good_enough improve (improve guess)
;;

let sqrt_via_ii x =
  let good_enough guess = Float.abs ((guess *. guess) -. x) < 0.001 in
  let improve guess = (guess +. (x /. guess)) /. 2.0 in
  iterative_improve good_enough improve 1.0
;;

let tolerance = 0.00001

let fixed_point_via_ii f guess =
  let good_enough g = Float.abs (g -. f g) < tolerance in
  iterative_improve good_enough f guess
;;

let ex_1_46 () = sqrt_via_ii 9.0, fixed_point_via_ii cos 1.0

let sqrt_guesses x =
  let rec from guess =
    Seq.Cons (guess, fun () -> from ((guess +. (x /. guess)) /. 2.0))
  in
  fun () -> from 1.0
;;

let ex_1_46a () = List.of_seq (Seq.take 5 (sqrt_guesses 2.0))
