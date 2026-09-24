(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.58 *)

type infix =
  | Num of int
  | Var of string
  | Plus of infix * infix
  | Times of infix * infix

let is_number _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_sum _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let make_product _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let deriv _a0 _a1 = raise Sicp_common.Pending.Pending_solution
let ex_2_58_parenthesized_expr = raise Sicp_common.Pending.Pending_solution
let ex_2_58_parenthesized () = raise Sicp_common.Pending.Pending_solution

type tok =
  | TNum of int
  | TVar of string
  | TPlus
  | TTimes
  | TOpen
  | TClose

let parse_standard _a0 = raise Sicp_common.Pending.Pending_solution
let ex_2_58_standard_tokens = raise Sicp_common.Pending.Pending_solution
let ex_2_58_standard () = raise Sicp_common.Pending.Pending_solution
