(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise
   SICP section 2.1 exercise 2.16 *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b

let sub_interval x y =
  make_interval (lower_bound x -. upper_bound y) (upper_bound x -. lower_bound y)
;;

let ex_2_16 () = raise Sicp_common.Pending.Pending_solution
