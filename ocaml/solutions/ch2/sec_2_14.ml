(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program par1 in SICP section 2.1
   exercise 2.14 *)

(** [div_interval] treats [a] in [A / A] as two independent
    quantities that merely happen to share the same bounds, so the
    result is not the exact [[1, 1]] the algebra [A / A = 1] promises;
    its percent tolerance is nonzero, though small. [par1] and [par2]
    likewise diverge, because [par1] uses [r1] and [r2] twice each
    (once directly, once inside the other's product), while [par2]
    uses each only once. *)

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

let ex_2_14 () =
  let a = make_center_percent 100.0 1.0 in
  let b = make_center_percent 200.0 1.0 in
  let r1 = make_center_percent 6.8 10.0 in
  let r2 = make_center_percent 4.7 5.0 in
  ( percent (div_interval a a)
  , percent (div_interval a b)
  , percent (par1 r1 r2)
  , percent (par2 r1 r2) )
;;
