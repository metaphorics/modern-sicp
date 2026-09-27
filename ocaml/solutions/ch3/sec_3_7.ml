(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.7 *)

(** Exercise 3.7: [make-joint] shares one account under a second
    password, following @ref{Exercise 3.3}'s password-protected
    account. *)

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

let make_joint account account_password new_password =
  fun given_password message ->
  if String.equal given_password new_password
  then account account_password message
  else Incorrect_password
;;

let ex_3_07 () =
  let peter_acc = make_account 100 "open-sesame" in
  let paul_acc = make_joint peter_acc "open-sesame" "rosebud" in
  let withdrawal = paul_acc "rosebud" (Withdraw 40) in
  let peters_view = peter_acc "open-sesame" (Deposit 0) in
  withdrawal, peters_view
;;

(** Addition 3.7a: a read-only capability over the unpassworded
    account of @ref{3.1.1}, projected out of the full account record. *)

type account =
  { withdraw : int -> withdraw_result
  ; deposit : int -> int
  ; balance : unit -> int
  }

type read_only_account = { balance : unit -> int }

let make_full_account balance =
  let balance_cell = ref balance in
  let withdraw amount =
    if !balance_cell >= amount
    then (
      balance_cell := !balance_cell - amount;
      Balance !balance_cell)
    else Insufficient_funds
  in
  let deposit amount =
    balance_cell := !balance_cell + amount;
    !balance_cell
  in
  { withdraw; deposit; balance = (fun () -> !balance_cell) }
;;

let read_only_capability (account : account) : read_only_account =
  { balance = account.balance }
;;

let ex_3_07a () =
  let acc = make_full_account 100 in
  let viewer = read_only_capability acc in
  let before = viewer.balance () in
  let (_ : int) = acc.deposit 50 in
  let after = viewer.balance () in
  before, after
;;
