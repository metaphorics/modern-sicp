(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.43 *)

(** Exercise 3.43: three accounts start at 10, 20, and 30; processes
    run concurrent exchanges. Sequential runs only ever permute who
    holds which of the original three amounts, so the multiset never
    changes. The plain exchange, serialized only per account, can
    still interleave two exchanges' reads and writes and leave a
    balance that is not one of the original three -- but each
    withdraw-then-deposit pair still moves exactly the amount it
    computed from one account to the other, so the sum survives even
    then. These accounts move balances without an insufficient-funds
    check, matching the text's own simplification that its [deposit]
    message accepts negative amounts. Their dispatch answers raw,
    exactly the way the text's [make-account-and-serializer] exposes
    its procedures: the exported serializer is the only guard, and
    wrapping [exchange] in both accounts' serializers makes the whole
    exchange one atomic step. *)

open Sicp_ch3.Sec_3_4

type raw_account =
  { dispatch : Account.account
  ; serializer : Serializers.serializer
  }

let make_raw_account initial =
  let balance = ref initial in
  let serializer = Serializers.make_serializer () in
  let dispatch m =
    match m with
    | Account.Withdraw amount ->
      balance := !balance - amount;
      Account.Withdrawn (Sicp_ch3.Sec_3_1.Balance !balance)
    | Account.Deposit amount ->
      balance := !balance + amount;
      Account.New_balance !balance
    | Account.Balance -> Account.New_balance !balance
    | Account.Serializer -> Account.Serialized serializer
  in
  { dispatch; serializer }
;;

let balance_of account =
  match account.dispatch Account.Balance with
  | Account.New_balance n -> n
  | _ -> invalid_arg "balance_of: not a balance"
;;

let exchange account1 account2 =
  let difference = balance_of account1 - balance_of account2 in
  ignore (account1.dispatch (Account.Withdraw difference));
  ignore (account2.dispatch (Account.Deposit difference))
;;

let serialized_exchange account1 account2 =
  account1.serializer.protect (fun () ->
    account2.serializer.protect (fun () -> exchange account1 account2))
;;

let three_accounts balances =
  match balances with
  | [ b1; b2; b3 ] -> make_raw_account b1, make_raw_account b2, make_raw_account b3
  | _ -> invalid_arg "three_accounts: exactly three starting balances"
;;

let one_round exchange_op (a1, a2, a3) =
  ignore (Parallel.parallel (fun () -> exchange_op a1 a2) (fun () -> exchange_op a1 a3))
;;

let multiset_preserved_by_serialized_exchange rounds balances =
  let expected = List.sort compare balances in
  let accounts = three_accounts balances in
  let all_ok = ref true in
  let a1, a2, a3 = accounts in
  for _ = 1 to rounds do
    one_round serialized_exchange accounts;
    let now = List.sort compare [ balance_of a1; balance_of a2; balance_of a3 ] in
    if now <> expected then all_ok := false
  done;
  !all_ok
;;

let sum_preserved_by_plain_exchange rounds balances =
  let expected = List.fold_left ( + ) 0 balances in
  let accounts = three_accounts balances in
  let all_ok = ref true in
  let a1, a2, a3 = accounts in
  for _ = 1 to rounds do
    one_round exchange accounts;
    let now = balance_of a1 + balance_of a2 + balance_of a3 in
    if now <> expected then all_ok := false
  done;
  !all_ok
;;

let ex_3_43 () =
  let serialized_ok = multiset_preserved_by_serialized_exchange 200 [ 10; 20; 30 ] in
  let sum_ok = sum_preserved_by_plain_exchange 200 [ 10; 20; 30 ] in
  let violations =
    let expected = List.sort compare [ 10; 20; 30 ] in
    let a1, a2, a3 = three_accounts [ 10; 20; 30 ] in
    let count = ref 0 in
    for _ = 1 to 200 do
      one_round exchange (a1, a2, a3);
      let now = List.sort compare [ balance_of a1; balance_of a2; balance_of a3 ] in
      if now <> expected then incr count
    done;
    !count
  in
  serialized_ok, sum_ok, violations
;;
