(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program mul-interval in SICP section 2.1
   exercise 2.11 *)

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

let mul_interval _x _y = raise Sicp_common.Pending.Pending_solution
let ex_2_11 () = raise Sicp_common.Pending.Pending_solution
