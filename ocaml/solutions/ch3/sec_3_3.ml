(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.3 *)

(** Exercise 3.3: a password-protected account. Scheme's two-level
    application [((acc password op) amount)] flattens into one
    three-argument curried call, since OCaml never needs a returned
    procedure just to carry [amount]. *)

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

let make_account balance password =
  let balance = ref balance in
  fun given_password message ->
    if String.equal given_password password
    then (
      match message with
      | Withdraw amount ->
        if !balance >= amount
        then (
          balance := !balance - amount;
          Balance_response (Balance !balance))
        else Balance_response Insufficient_funds
      | Deposit amount ->
        balance := !balance + amount;
        Deposit_response !balance)
    else Incorrect_password
;;

let ex_3_03 () =
  let acc = make_account 100 "secret-password" in
  let first = acc "secret-password" (Withdraw 40) in
  let second = acc "some-other-password" (Deposit 50) in
  first, second
;;
