(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.56 *)

type expr =
  | Const of int
  | Var of string
  | Sum of expr * expr
  | Prod of expr * expr
  | Power of expr * expr

let is_number _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let same_variable _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_sum _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_product _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let base _a0 = raise Sicp_common.Pending.Pending_solution
let exponent _a0 = raise Sicp_common.Pending.Pending_solution
let make_exponentiation _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let deriv _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_56 () = raise Sicp_common.Pending.Pending_solution
