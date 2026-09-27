(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.10 *)

(** Exercise 3.10: the lifetime of the ref cell that make_withdraw
    allocates and its closure captures. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

let make_withdraw _balance = raise Sicp_common.Pending.Pending_solution

type withdraw_with_cell =
  { withdraw : int -> withdraw_result
  ; balance_cell : int ref
  }

let make_withdraw_traced _initial = raise Sicp_common.Pending.Pending_solution
let ex_3_10 () = raise Sicp_common.Pending.Pending_solution
