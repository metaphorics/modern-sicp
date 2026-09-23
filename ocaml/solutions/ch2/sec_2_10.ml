(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program div-interval in SICP section 2.1
   exercise 2.10 *)

(** OCaml, like Scheme, never raises on a float division by zero, so
    Ben's condition has to be checked explicitly: a divisor spans zero
    when its lower bound is at most 0 and its upper bound is at least
    0 (an endpoint exactly at 0 already makes the reciprocal
    infinite). *)

type interval = float * float
type interval_error = Spans_zero of float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b

let mul_interval x y =
  let p1 = lower_bound x *. lower_bound y in
  let p2 = lower_bound x *. upper_bound y in
  let p3 = upper_bound x *. lower_bound y in
  let p4 = upper_bound x *. upper_bound y in
  make_interval
    (Float.min (Float.min p1 p2) (Float.min p3 p4))
    (Float.max (Float.max p1 p2) (Float.max p3 p4))
;;

let div_interval x y =
  if lower_bound y <= 0.0 && upper_bound y >= 0.0
  then Error (Spans_zero (lower_bound y, upper_bound y))
  else Ok (mul_interval x (make_interval (1.0 /. upper_bound y) (1.0 /. lower_bound y)))
;;

let ex_2_10 () = div_interval (make_interval 1.0 2.0) (make_interval (-1.0) 1.0)
