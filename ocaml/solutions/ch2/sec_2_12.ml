(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-center-width in SICP section
   2.1 exercise 2.12 *)

(** [make_center_percent c p] restates [make_center_width] with the
    width computed from the percentage: [p] percent of [c] is [c *. p
    /. 100.0]. [percent] inverts that: [width i] divided by the
    center's magnitude, scaled back up to a percentage.

    Addition 2.12a: exercise 2.9 already proves [width (add_interval x
    y) = width x +. width y]; [center (add_interval x y) = center x +.
    center y] holds by the same kind of algebra. [add_interval_by_center_width]
    restates the sum directly from those two facts, comparing the
    center-percent interface against the endpoint interface Alyssa
    started from. *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b
let center i = (lower_bound i +. upper_bound i) /. 2.0
let width i = (upper_bound i -. lower_bound i) /. 2.0
let make_center_width c w = make_interval (c -. w) (c +. w)
let make_center_percent c p = make_center_width c (c *. p /. 100.0)
let percent i = width i /. Float.abs (center i) *. 100.0

let ex_2_12 () =
  let i = make_center_percent 6.8 10.0 in
  center i, percent i
;;

let add_interval x y =
  make_interval (lower_bound x +. lower_bound y) (upper_bound x +. upper_bound y)
;;

let add_interval_by_center_width x y =
  make_center_width (center x +. center y) (width x +. width y)
;;

let ex_2_12a () =
  let close a b = Float.abs (a -. b) < 1e-9 in
  let agrees (x, y) =
    let via_endpoints = add_interval x y in
    let via_center_width = add_interval_by_center_width x y in
    close (lower_bound via_endpoints) (lower_bound via_center_width)
    && close (upper_bound via_endpoints) (upper_bound via_center_width)
  in
  List.for_all
    agrees
    [ make_interval 6.12 7.48, make_interval 4.465 4.935
    ; make_interval (-3.0) 5.0, make_interval 2.0 2.0
    ; make_center_percent 100.0 1.0, make_center_percent 200.0 2.0
    ]
;;
