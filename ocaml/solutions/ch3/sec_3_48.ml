(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.48 *)

(** Exercise 3.48: number the accounts and always protect the
    lower-numbered one first. Two exchanges that would otherwise
    deadlock -- one exchanging accounts 1 and 2, the other exchanging
    2 and 1 -- now both try to enter account 1's serializer first, so
    the second simply waits for the first to finish instead of each
    holding one lock and waiting for the other's. The numbered
    accounts answer raw, the way the text's
    [make-account-and-serializer] exposes its procedures, so the
    exported serializer is the only guard and the ordered double
    protection never re-enters itself. *)

open Sicp_ch3.Sec_3_4

type numbered_account =
  { id : int
  ; dispatch : Account.account
  ; serializer : Serializers.serializer
  }

let make_numbered_account id initial =
  let balance = ref initial in
  let serializer = Serializers.make_serializer () in
  let dispatch m =
    match m with
    | Account.Withdraw amount ->
      if !balance >= amount
      then (
        balance := !balance - amount;
        Account.Withdrawn (Sicp_ch3.Sec_3_1.Balance !balance))
      else Account.Withdrawn Sicp_ch3.Sec_3_1.Insufficient_funds
    | Account.Deposit amount ->
      balance := !balance + amount;
      Account.New_balance !balance
    | Account.Balance -> Account.New_balance !balance
    | Account.Serializer -> Account.Serialized serializer
  in
  { id; dispatch; serializer }
;;

let balance_of account =
  match account.dispatch Account.Balance with
  | Account.New_balance n -> n
  | _ -> invalid_arg "balance_of: not a balance"
;;

let raw_exchange account1 account2 =
  let difference = balance_of account1 - balance_of account2 in
  ignore (account1.dispatch (Account.Withdraw difference));
  ignore (account2.dispatch (Account.Deposit difference))
;;

let ordered_serialized_exchange account1 account2 =
  let first, second =
    if account1.id <= account2.id then account1, account2 else account2, account1
  in
  first.serializer.protect (fun () ->
    second.serializer.protect (fun () -> raw_exchange account1 account2))
;;

let survives_reversed_concurrent_exchanges rounds =
  let a1 = make_numbered_account 1 10 in
  let a2 = make_numbered_account 2 20 in
  let a3 = make_numbered_account 3 30 in
  let expected = List.sort compare [ 10; 20; 30 ] in
  let all_completed = ref true in
  for _ = 1 to rounds do
    ignore
      (Parallel.parallel
         (fun () -> ordered_serialized_exchange a1 a2)
         (fun () -> ordered_serialized_exchange a2 a1));
    let now = List.sort compare [ balance_of a1; balance_of a2; balance_of a3 ] in
    if now <> expected then all_completed := false
  done;
  !all_completed
;;

let ex_3_48 () =
  let ok = survives_reversed_concurrent_exchanges 500 in
  let a1 = make_numbered_account 1 10 in
  let a2 = make_numbered_account 2 20 in
  let a3 = make_numbered_account 3 30 in
  ordered_serialized_exchange a1 a2;
  ok, List.sort compare [ balance_of a1; balance_of a2; balance_of a3 ]
;;
