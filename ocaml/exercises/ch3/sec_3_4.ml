(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.4 *)

(** Exercise 3.4: a password-protected account that calls the cops
    after seven consecutive incorrect passwords. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

type message =
  | Withdraw of int
  | Deposit of int

type response =
  | Balance_response of withdraw_result
  | Deposit_response of int
  | Incorrect_password
  | Police_called

let make_account _balance _password = raise Sicp_common.Pending.Pending_solution
let ex_3_04 () = raise Sicp_common.Pending.Pending_solution
