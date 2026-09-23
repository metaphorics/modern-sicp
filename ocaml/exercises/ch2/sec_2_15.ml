(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program par2 in SICP section 2.1
   exercise 2.15 *)

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

let add_interval x y =
  make_interval (lower_bound x +. lower_bound y) (upper_bound x +. upper_bound y)
;;

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
  mul_interval x (make_interval (1.0 /. upper_bound y) (1.0 /. lower_bound y))
;;

let par1 r1 r2 = div_interval (mul_interval r1 r2) (add_interval r1 r2)

let par2 r1 r2 =
  let one = make_interval 1.0 1.0 in
  div_interval one (add_interval (div_interval one r1) (div_interval one r2))
;;

let ex_2_15 () = raise Sicp_common.Pending.Pending_solution
