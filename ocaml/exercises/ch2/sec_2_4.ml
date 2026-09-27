(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cons in SICP section 2.1
   exercise 2.4 *)

let cons x y m = m x y
let car z = z (fun p _q -> p)
let cdr _z = raise Sicp_common.Pending.Pending_solution
let ex_2_04 _x _y = raise Sicp_common.Pending.Pending_solution
