(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.43 *)

(** Exercise 3.43: three accounts start at 10, 20, and 30; processes
    run concurrent exchanges. Sequential runs only ever permute who
    holds which of the original three amounts, so the multiset never
    changes. The plain exchange can still interleave two exchanges'
    reads and writes. Over accounts whose every dispatch step is
    serialized -- the sum probe's accounts -- each withdraw and
    deposit still moves exactly the amount it computed, so the sum
    survives even then. Over the raw accounts, whose dispatch answers
    with no guard at all the way the text's
    [make-account-and-serializer] exposes its procedures, two
    exchanges can lose an update between them, and then even the sum
    goes wrong; the edition observes that as a count and never asserts
    it. These accounts move balances without an insufficient-funds
    check, matching the text's own simplification that its [deposit]
    message accepts negative amounts. *)

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

let make_serialized_account initial =
  let account = make_raw_account initial in
  let serializer = account.serializer in
  { dispatch = (fun m -> serializer.protect (fun () -> account.dispatch m)); serializer }
;;

let three_accounts_with maker balances =
  match balances with
  | [ b1; b2; b3 ] -> maker b1, maker b2, maker b3
  | _ -> invalid_arg "three_accounts_with: exactly three starting balances"
;;

let one_round exchange_op (a1, a2, a3) =
  ignore (Parallel.parallel (fun _ -> exchange_op a1 a2) (fun _ -> exchange_op a1 a3))
;;

let multiset_preserved_by_serialized_exchange rounds balances =
  let expected = List.sort compare balances in
  let accounts = three_accounts_with make_raw_account balances in
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
  let accounts = three_accounts_with make_serialized_account balances in
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
    let a1, a2, a3 = three_accounts_with make_raw_account [ 10; 20; 30 ] in
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
