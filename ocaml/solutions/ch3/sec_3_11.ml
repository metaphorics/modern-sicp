(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.11 *)

(** Exercise 3.11 (and the edition's addition 3.11a): where an
    account's local state lives, and what identity and equality say
    about two accounts. [ex_3_11] replays the statement's interactions
    and reports whether two accounts are distinct objects; [ex_3_11a]
    runs the identity-versus-equality checklist on [==] and [=]. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

type account =
  { withdraw : int -> withdraw_result
  ; deposit : int -> int
  }

let make_account balance =
  let balance = ref balance in
  let withdraw amount =
    if !balance >= amount
    then (
      balance := !balance - amount;
      Balance !balance)
    else Insufficient_funds
  in
  let deposit amount =
    balance := !balance + amount;
    !balance
  in
  { withdraw; deposit }
;;

let ex_3_11 () =
  let acc = make_account 50 in
  let acc2 = make_account 100 in
  let deposited = acc.deposit 40 in
  let withdrawn = acc.withdraw 60 in
  let acc2_balance = acc2.deposit 0 in
  let records_distinct = not (acc == acc2) in
  deposited, withdrawn, acc2_balance, records_distinct
;;

let ex_3_11a () =
  let acc = make_account 100 in
  let twin = make_account 100 in
  let alias = acc in
  let distinct_objects = acc == twin in
  let alias_same_object = acc == alias in
  (* Structural comparison on records of closures cannot decide
     equality between two separately created accounts: it raises. *)
  let structural_compare_raises =
    match acc = twin with
    | _ -> false
    | exception Invalid_argument _ -> true
  in
  let from_acc = acc.withdraw 20 in
  let from_twin = twin.withdraw 20 in
  let same_behavior = from_acc = from_twin in
  let still_distinct = not (acc == twin) in
  ( distinct_objects
  , alias_same_object
  , structural_compare_raises
  , same_behavior
  , still_distinct )
;;
