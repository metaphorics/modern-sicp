(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program width in SICP section 2.1
   exercise 2.9 *)

(** [width (add_interval x y) = width x +. width y] always, by
    algebra: [add_interval]'s bounds are [lower_bound x +. lower_bound
    y] and [upper_bound x +. upper_bound y], so its width is
    [((ux +. uy) -. (lx +. ly)) /. 2 = (ux -. lx) /. 2 +. (uy -. ly) /.
    2], the sum of the two widths, independent of where the intervals
    are centered. Subtraction is the same argument with [y]'s sign
    flipped. Multiplication has no such formula: [ex_2_09] exhibits
    two width-1 intervals whose products with the same third interval
    have different widths, so the product's width depends on more
    than just the two argument widths. *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b
let width i = (upper_bound i -. lower_bound i) /. 2.0

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

let ex_2_09 () =
  let a1 = make_interval 1.0 3.0 in
  (* width 1 *)
  let a2 = make_interval 5.0 7.0 in
  (* width 1, same width as a1 *)
  let b = make_interval 10.0 12.0 in
  width (mul_interval a1 b), width (mul_interval a2 b)
;;
