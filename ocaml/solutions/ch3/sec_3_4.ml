(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.4 *)

(** Exercise 3.4: @ref{Exercise 3.3}'s account with a second local
    state variable, a consecutive-bad-password counter, gating
    [call-the-cops]. *)

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

let make_account balance password =
  let balance = ref balance in
  let bad_password_count = ref 0 in
  fun given_password message ->
    if String.equal given_password password
    then (
      bad_password_count := 0;
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
    else (
      incr bad_password_count;
      if !bad_password_count > 7 then Police_called else Incorrect_password)
;;

let ex_3_04 () =
  let acc = make_account 100 "secret-password" in
  List.init 8 (fun _ -> acc "wrong-password" (Withdraw 10))
;;
