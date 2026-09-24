(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.7 *)

(** Exercise 3.7: [make-joint] shares one account under a second
    password. *)

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

let make_account _balance _password = raise Sicp_common.Pending.Pending_solution

let make_joint _account _account_password _new_password =
  raise Sicp_common.Pending.Pending_solution
;;

let ex_3_07 () = raise Sicp_common.Pending.Pending_solution

(** Addition 3.7a: a read-only capability over the unpassworded
    account of @ref{3.1.1}, projected out of the full account record. *)

type account =
  { withdraw : int -> withdraw_result
  ; deposit : int -> int
  ; balance : unit -> int
  }

type read_only_account = { balance : unit -> int }

let make_full_account _balance = raise Sicp_common.Pending.Pending_solution
let read_only_capability _account = raise Sicp_common.Pending.Pending_solution
let ex_3_07a () = raise Sicp_common.Pending.Pending_solution
