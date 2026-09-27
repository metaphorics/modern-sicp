(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.73 *)

type expr =
  | Const of int
  | Var of string
  | Compound of string * expr list

let operator _a0 = raise Sicp_common.Pending.Pending_solution
let operands _a0 = raise Sicp_common.Pending.Pending_solution

type deriv_rule = (expr -> string -> expr) -> expr list -> string -> expr
type table = unit

let make_table () = raise Sicp_common.Pending.Pending_solution
let install_sum_rule _a0 = raise Sicp_common.Pending.Pending_solution
let install_product_rule _a0 = raise Sicp_common.Pending.Pending_solution
let install_expt_rule _a0 = raise Sicp_common.Pending.Pending_solution
let deriv _a0 _a1 _a2 = raise Sicp_common.Pending.Pending_solution
let make_sum _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_product _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_expt _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_73 () = raise Sicp_common.Pending.Pending_solution
