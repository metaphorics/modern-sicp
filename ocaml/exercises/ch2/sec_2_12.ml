(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-center-width in SICP section
   2.1 exercise 2.12 *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b
let center i = (lower_bound i +. upper_bound i) /. 2.0
let width i = (upper_bound i -. lower_bound i) /. 2.0
let make_center_width c w = make_interval (c -. w) (c +. w)
let make_center_percent _c _p = raise Sicp_common.Pending.Pending_solution
let percent _i = raise Sicp_common.Pending.Pending_solution
let ex_2_12 () = raise Sicp_common.Pending.Pending_solution

let add_interval x y =
  make_interval (lower_bound x +. lower_bound y) (upper_bound x +. upper_bound y)
;;

let add_interval_by_center_width _x _y = raise Sicp_common.Pending.Pending_solution
let ex_2_12a () = raise Sicp_common.Pending.Pending_solution
