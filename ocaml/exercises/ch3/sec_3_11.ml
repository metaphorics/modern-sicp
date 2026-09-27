(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.11 *)

(** Exercise 3.11 (and the edition's addition 3.11a): where an
    account's local state lives, and what identity and equality say
    about two accounts. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

type account =
  { withdraw : int -> withdraw_result
  ; deposit : int -> int
  }

let make_account _balance = raise Sicp_common.Pending.Pending_solution
let ex_3_11 () = raise Sicp_common.Pending.Pending_solution
let ex_3_11a () = raise Sicp_common.Pending.Pending_solution
