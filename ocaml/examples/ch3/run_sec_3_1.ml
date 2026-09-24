(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_3_1 = Sicp_ch3.Sec_3_1
open Sec_3_1

let show_withdraw_result = function
  | Balance b -> Printf.sprintf "Balance %d" b
  | Insufficient_funds -> "Insufficient_funds"
;;

let expect_withdraw r shown = Replay.expect (show_withdraw_result r) shown
let expect_int n = Replay.expect (string_of_int n)

let () =
  (* Global_withdraw: a top-level [ref], mutated across four calls. *)
  expect_withdraw (Global_withdraw.withdraw 25) "Balance 75";
  expect_withdraw (Global_withdraw.withdraw 25) "Balance 50";
  expect_withdraw (Global_withdraw.withdraw 60) "Insufficient_funds";
  expect_withdraw (Global_withdraw.withdraw 15) "Balance 35";
  (* Make_withdraw: two independent objects, W1 and W2. *)
  let w1 = Make_withdraw.make_withdraw 100 in
  let w2 = Make_withdraw.make_withdraw 100 in
  expect_withdraw (w1 50) "Balance 50";
  expect_withdraw (w2 70) "Balance 30";
  expect_withdraw (w2 40) "Insufficient_funds";
  expect_withdraw (w1 40) "Balance 10";
  (* Make_account: withdraw and deposit through one record of closures. *)
  let acc = Make_account.make_account 100 in
  expect_withdraw (acc.withdraw 50) "Balance 50";
  expect_withdraw (acc.withdraw 60) "Insufficient_funds";
  expect_int (acc.deposit 40) "90";
  expect_withdraw (acc.withdraw 60) "Balance 30";
  (* Make_simplified_withdraw: no insufficient-funds check. *)
  let w = Make_simplified_withdraw.make_simplified_withdraw 25 in
  expect_int (w 20) "5";
  expect_int (w 10) "-5";
  (* Make_decrementer: no accumulated effect across calls. *)
  let d = Make_decrementer.make_decrementer 25 in
  expect_int (d 20) "5";
  expect_int (d 10) "15";
  (* Sameness and change: W1 and W2 built from the same expression are
     still two distinct objects, each with its own local state. *)
  let w1' = Make_simplified_withdraw.make_simplified_withdraw 25 in
  let w2' = Make_simplified_withdraw.make_simplified_withdraw 25 in
  expect_int (w1' 20) "5";
  expect_int (w1' 20) "-15";
  expect_int (w2' 20) "5"
;;
