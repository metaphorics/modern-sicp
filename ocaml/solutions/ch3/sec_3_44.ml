(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.44 *)

(** Exercise 3.44: Ben's [transfer] withdraws a known [amount] from one
    account and deposits it into another; Louis wants the heavier
    exchange machinery instead. [transfer] only ever needs the amount
    it was given -- it never reads a balance to compute what to move --
    so its two steps do not need a joint lock the way [exchange]'s
    read-then-move does. Concurrent transfers among independently
    serialized accounts still conserve the total. *)

open Sicp_ch3.Sec_3_4.Account

let transfer from_account to_account amount =
  match from_account (Withdraw amount) with
  | Withdrawn (Sicp_ch3.Sec_3_1.Balance _) -> ignore (to_account (Deposit amount))
  | Withdrawn Sicp_ch3.Sec_3_1.Insufficient_funds -> ()
  | _ -> invalid_arg "transfer: not a withdrawal response"
;;

let total_of accounts =
  List.fold_left
    (fun sum account ->
       match account Balance with
       | New_balance n -> sum + n
       | _ -> invalid_arg "total_of: not a balance")
    0
    accounts
;;

let total_preserved_under_concurrent_transfers rounds per_transfer =
  let a1 = make_account 1000 in
  let a2 = make_account 1000 in
  let a3 = make_account 1000 in
  let expected = total_of [ a1; a2; a3 ] in
  let all_ok = ref true in
  for _ = 1 to rounds do
    ignore
      (Sicp_ch3.Sec_3_4.Parallel.parallel
         (fun _ -> transfer a1 a2 per_transfer)
         (fun _ -> transfer a2 a3 per_transfer));
    if total_of [ a1; a2; a3 ] <> expected then all_ok := false
  done;
  !all_ok
;;

let ex_3_44 () =
  let ok = total_preserved_under_concurrent_transfers 300 10 in
  let a1 = make_account 1000 in
  let a2 = make_account 1000 in
  let a3 = make_account 1000 in
  ok, total_of [ a1; a2; a3 ]
;;
