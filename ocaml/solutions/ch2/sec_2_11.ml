(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program mul-interval in SICP section 2.1
   exercise 2.11 *)

(** The nine cases split on whether [x]'s bounds are both non-negative
    ([xl >= 0]), both non-positive ([xu <= 0]), or straddle zero, and
    likewise for [y]; each combination picks the two products (one
    multiplication doubled to a min and a max in the straddle/straddle
    case) that must bound every [a * b] for [a] in [x], [b] in [y]. *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b

let mul_interval_naive x y =
  let p1 = lower_bound x *. lower_bound y in
  let p2 = lower_bound x *. upper_bound y in
  let p3 = upper_bound x *. lower_bound y in
  let p4 = upper_bound x *. upper_bound y in
  make_interval
    (Float.min (Float.min p1 p2) (Float.min p3 p4))
    (Float.max (Float.max p1 p2) (Float.max p3 p4))
;;

let mul_interval x y =
  let xl = lower_bound x
  and xu = upper_bound x
  and yl = lower_bound y
  and yu = upper_bound y in
  if xl >= 0.0 && yl >= 0.0
  then make_interval (xl *. yl) (xu *. yu)
  else if xl >= 0.0 && yu <= 0.0
  then make_interval (xu *. yl) (xl *. yu)
  else if xl >= 0.0
  then make_interval (xu *. yl) (xu *. yu)
  else if xu <= 0.0 && yl >= 0.0
  then make_interval (xl *. yu) (xu *. yl)
  else if xu <= 0.0 && yu <= 0.0
  then make_interval (xu *. yu) (xl *. yl)
  else if xu <= 0.0
  then make_interval (xl *. yu) (xl *. yl)
  else if yl >= 0.0
  then make_interval (xl *. yu) (xu *. yu)
  else if yu <= 0.0
  then make_interval (xu *. yl) (xl *. yl)
  else make_interval (Float.min (xl *. yu) (xu *. yl)) (Float.max (xl *. yl) (xu *. yu))
;;

let ex_2_11 () =
  let cases =
    [ make_interval 2.0 4.0, make_interval 3.0 5.0 (* +, + *)
    ; make_interval 2.0 4.0, make_interval (-5.0) (-3.0) (* +, - *)
    ; make_interval 2.0 4.0, make_interval (-3.0) 5.0 (* +, 0 *)
    ; make_interval (-4.0) (-2.0), make_interval 3.0 5.0 (* -, + *)
    ; make_interval (-4.0) (-2.0), make_interval (-5.0) (-3.0) (* -, - *)
    ; make_interval (-4.0) (-2.0), make_interval (-3.0) 5.0 (* -, 0 *)
    ; make_interval (-2.0) 4.0, make_interval 3.0 5.0 (* 0, + *)
    ; make_interval (-2.0) 4.0, make_interval (-5.0) (-3.0) (* 0, - *)
    ; make_interval (-2.0) 4.0, make_interval (-3.0) 5.0 (* 0, 0 *)
    ]
  in
  List.for_all (fun (x, y) -> mul_interval x y = mul_interval_naive x y) cases
;;
