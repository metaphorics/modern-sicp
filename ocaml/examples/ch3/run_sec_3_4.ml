(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_3_4 = Sicp_ch3.Sec_3_4
module S = Sec_3_4.Shared_withdraw
module R = Sec_3_4.X_race
module A = Sec_3_4.Account
module E = Sec_3_4.Exchange
module M = Sec_3_4.Mutex_impl

let show_withdraw_result = function
  | Sicp_ch3.Sec_3_1.Balance b -> Printf.sprintf "Balance %d" b
  | Sicp_ch3.Sec_3_1.Insufficient_funds -> "Insufficient_funds"
;;

let show_response = function
  | A.Withdrawn wr -> show_withdraw_result wr
  | A.New_balance n -> string_of_int n
  | A.Serialized _ -> "Serialized <serializer>"
;;

let expect_response r shown = Replay.expect (show_response r) shown
let expect_int n = Replay.expect (string_of_int n)
let expect_bool b shown = Replay.expect (string_of_bool b) shown

let () =
  (* The 3.1.1 withdrawal recalled: successive calls, different values. *)
  Replay.expect (show_withdraw_result (S.withdraw 25)) "Balance 75";
  Replay.expect (show_withdraw_result (S.withdraw 25)) "Balance 50";
  (* The increment/square race: one unserialized run lands somewhere in
     the five-outcome set, a serialized run only in {101, 121}. *)
  let saw_unserialized = List.mem (R.run_unserialized ()) R.possible_values in
  expect_bool saw_unserialized "true";
  let saw_serialized = List.mem (R.run_serialized ()) R.serialized_values in
  expect_bool saw_serialized "true";
  (* The serialized bank account, driven by messages. *)
  let acc = A.make_account 100 in
  expect_response (acc (A.Withdraw 50)) "Balance 50";
  expect_response (acc (A.Withdraw 60)) "Insufficient_funds";
  expect_response (acc (A.Deposit 40)) "90";
  expect_response (acc A.Balance) "90";
  (* Concurrent deposits through one serializer conserve every unit. *)
  expect_int (A.concurrent_deposits acc 200) "290";
  (* The account that exports its serializer. Exchanging swaps the two
     balances: account1 loses the difference, account2 gains it. *)
  let a1 = E.make_account_and_serializer 20 in
  let a2 = E.make_account_and_serializer 10 in
  E.exchange a1 a2;
  expect_int (E.balance_of a1) "10";
  expect_int (E.balance_of a2) "20";
  expect_int (E.deposit a1 5) "15";
  E.serialized_exchange a1 a2;
  expect_int (E.balance_of a1) "20";
  expect_int (E.balance_of a2) "15";
  (* The section's own mutex: protected increments from two domains
     lose none of their updates. *)
  let counter = ref 0 in
  let s = M.make_serializer () in
  ignore
    (Sec_3_4.Parallel.parallel
       (fun _ ->
          for _ = 1 to 1000 do
            s.protect (fun () -> incr counter)
          done)
       (fun _ ->
          for _ = 1 to 1000 do
            s.protect (fun () -> incr counter)
          done));
  expect_int !counter "2000"
;;
