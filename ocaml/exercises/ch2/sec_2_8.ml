(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sub-interval in SICP section 2.1
   exercise 2.8 *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b
let sub_interval _x _y = raise Sicp_common.Pending.Pending_solution
let ex_2_08 () = raise Sicp_common.Pending.Pending_solution
