(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_3_2 = Sicp_ch3.Sec_3_2
open Sec_3_2

let show_withdraw_result = function
  | Balance b -> Printf.sprintf "Balance %d" b
  | Insufficient_funds -> "Insufficient_funds"
;;

let expect_withdraw r shown = Replay.expect (show_withdraw_result r) shown
let expect_int n = Replay.expect (string_of_int n)

let () =
  (* 3.2.2: the walkthrough of [f 5] through square and sum_of_squares. *)
  Replay.expect (string_of_int (F_and_sum_of_squares.f 5)) "136";
  (* 3.2.3: one withdrawal processor and its first call. *)
  let w1 = Local_state.make_withdraw 100 in
  expect_withdraw (w1 50) "Balance 50";
  (* 3.2.3: one cell shared by bump and peek. *)
  let bump, peek = Local_state.make_shared_counter () in
  expect_int (bump ()) "1";
  expect_int (bump ()) "2";
  expect_int (peek ()) "2";
  (* 3.2.3: separate cells for separate one-closure counters. *)
  let c1 = Local_state.make_counter () in
  let c2 = Local_state.make_counter () in
  expect_int (c1 ()) "1";
  expect_int (c1 ()) "2";
  expect_int (c2 ()) "1";
  (* 3.2.4: the block-structured sqrt of Figure 3.11 converges from
     guess 1.0 and stops once guess *. guess is within 0.001 of x. *)
  Replay.expect_float (Internal_definitions.sqrt 2.0) "1.4142156862745097"
;;
