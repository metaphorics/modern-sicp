(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fixed-point in SICP section 1.3
   exercise 1.36 *)

(** Exercise 1.36: [fixed_point_traced] is the section's [fixed_point]
    with a [print_endline] of every guess before testing it, and a
    step counter; the guess of [1.] the book warns against would divide
    by [log 1.] = [0.], so [x_to_the_x_eq_1000] starts from [2.]
    instead, as the exercise allows. *)

let tolerance = 0.00001

let fixed_point_traced f first_guess =
  let close_enough v1 v2 = Float.abs (v1 -. v2) < tolerance in
  let rec try_guess guess steps =
    Printf.printf "%.17g\n" guess;
    if steps > 100_000
    then failwith "fixed_point_traced: did not converge"
    else (
      let next = f guess in
      if close_enough guess next then next, steps + 1 else try_guess next (steps + 1))
  in
  try_guess first_guess 0
;;

let average_damp f x = (x +. f x) /. 2.0

let x_to_the_x_eq_1000 damped =
  let raw x = Float.log 1000.0 /. Float.log x in
  let transform = if damped then average_damp raw else raw in
  fixed_point_traced transform 2.0
;;

let ex_1_36 () = x_to_the_x_eq_1000 false, x_to_the_x_eq_1000 true
