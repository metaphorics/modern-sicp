(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program width in SICP section 2.1
   exercise 2.9 *)

type interval = float * float

let make_interval a b : interval = a, b
let lower_bound ((a, _) : interval) = a
let upper_bound ((_, b) : interval) = b
let width _i = raise Sicp_common.Pending.Pending_solution
let add_interval _x _y = raise Sicp_common.Pending.Pending_solution
let mul_interval _x _y = raise Sicp_common.Pending.Pending_solution
let ex_2_09 () = raise Sicp_common.Pending.Pending_solution
