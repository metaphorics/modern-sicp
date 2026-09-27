(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.13 *)

(** Write [x = c1 (1 +/- p1/100)] and [y = c2 (1 +/- p2/100)] for
    positive centers [c1, c2] and small tolerances [p1, p2]. Their
    product's extremes are [c1 c2 (1 +/- p1/100) (1 +/- p2/100) = c1
    c2 (1 +/- (p1 + p2)/100 +/- p1 p2 / 10000)]. The cross term [p1 p2
    / 10000] is a product of two small numbers, negligible next to
    [(p1 + p2)/100], leaving [percent (mul_interval x y) ~= percent x
    +. percent y]: the tolerances add. *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b
let center i = (lower_bound i +. upper_bound i) /. 2.0
let width i = (upper_bound i -. lower_bound i) /. 2.0

let make_center_percent c p =
  make_interval (c -. (c *. p /. 100.0)) (c +. (c *. p /. 100.0))
;;

let percent i = width i /. Float.abs (center i) *. 100.0

let mul_interval x y =
  let p1 = lower_bound x *. lower_bound y in
  let p2 = lower_bound x *. upper_bound y in
  let p3 = upper_bound x *. lower_bound y in
  let p4 = upper_bound x *. upper_bound y in
  make_interval
    (Float.min (Float.min p1 p2) (Float.min p3 p4))
    (Float.max (Float.max p1 p2) (Float.max p3 p4))
;;

let ex_2_13 () =
  let x = make_center_percent 10.0 1.0 in
  let y = make_center_percent 20.0 2.0 in
  percent (mul_interval x y), percent x +. percent y
;;
