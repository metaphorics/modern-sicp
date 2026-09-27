(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cubic in SICP section 1.3
   exercise 1.40 *)

(** Exercise 1.40: [cubic a b c] is a curried function of one remaining
    argument, [x^3 + a x^2 + b x + c], the shape [newtons_method]
    expects. [deriv], [newton_transform], and [newtons_method] repeat
    the section's own definitions locally, per this chapter's
    self-contained-per-file convention. *)

let cubic a b c x = (x *. x *. x) +. (a *. x *. x) +. (b *. x) +. c
let dx = 0.00001
let deriv g x = (g (x +. dx) -. g x) /. dx
let newton_transform g x = x -. (g x /. deriv g x)
let tolerance = 0.00001

let newtons_method g guess =
  let close_enough v1 v2 = Float.abs (v1 -. v2) < tolerance in
  let rec try_guess guess n =
    if n > 100_000
    then failwith "newtons_method: did not converge"
    else (
      let next = newton_transform g guess in
      if close_enough guess next then next else try_guess next (n + 1))
  in
  try_guess guess 0
;;

(* (x - 1)(x - 2)(x - 3) = x^3 - 6x^2 + 11x - 6 *)
let ex_1_40 () = newtons_method (cubic (-6.0) 11.0 (-6.0)) 1.0
