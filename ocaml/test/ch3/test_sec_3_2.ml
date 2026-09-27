(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 3.2. [sicp_ch3_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module Sec_3_9 = Sicp_ch3_solutions.Sec_3_9
module Sec_3_10 = Sicp_ch3_solutions.Sec_3_10
module Sec_3_11 = Sicp_ch3_solutions.Sec_3_11

let ex_3_09_statement_runs () =
  let (fact_result, fact_depth), (iter_result, iter_depth) = Sec_3_9.ex_3_09 () in
  Alcotest.(check int) "recursive factorial 6" 720 fact_result;
  Alcotest.(check int) "iterative factorial 6" 720 iter_result;
  Alcotest.(check int) "recursive depth is 6" 6 fact_depth;
  Alcotest.(check int) "iterative depth is constant" 1 iter_depth
;;

let ex_3_09_traced_agrees_deeper () =
  let n = 40 in
  let plain_rec = Sec_3_9.factorial n in
  let plain_iter = Sec_3_9.factorial_iter n in
  let rec_value, rec_depth = Sec_3_9.factorial_traced n in
  let iter_value, iter_depth = Sec_3_9.factorial_iter_traced n in
  Alcotest.(check int) "the two plain versions agree" plain_rec plain_iter;
  Alcotest.(check int) "traced recursive keeps the value" plain_rec rec_value;
  Alcotest.(check int) "traced iterative keeps the value" plain_iter iter_value;
  Alcotest.(check int) "recursive depth is n" n rec_depth;
  Alcotest.(check int) "iterative depth is 1" 1 iter_depth
;;

let ex_3_09_constant_space_at_scale () =
  let deep = 1_000_000 in
  let plain = Sec_3_9.factorial_iter deep in
  let value, depth = Sec_3_9.factorial_iter_traced deep in
  Alcotest.(check int) "the million-step loop answers" plain value;
  Alcotest.(check int) "its frame depth stayed at 1" 1 depth
;;

let is_balance n = function
  | Sec_3_10.Balance m -> n = m
  | Insufficient_funds -> false
;;

let ex_3_10_cell_witness () =
  let contents, distinct, (w1_sees, w2_sees), direct = Sec_3_10.ex_3_10 () in
  Alcotest.(check int) "first call leaves 50 in the cell" 50 contents;
  Alcotest.(check bool) "two accounts hold two cells" true distinct;
  Alcotest.(check (pair int int))
    "w2's withdrawal touched only its own cell"
    (50, 70)
    (w1_sees, w2_sees);
  Alcotest.(check bool)
    "the closure reads the returned cell after a direct write"
    true
    (is_balance 700 direct)
;;

let ex_3_10_witness_tracks_balance () =
  let w = Sec_3_10.make_withdraw_traced 42 in
  Alcotest.(check int) "witness starts at the initial balance" 42 !(w.balance_cell);
  Alcotest.(check bool)
    "a withdrawal answers Balance 40"
    true
    (is_balance 40 (w.withdraw 2));
  Alcotest.(check int) "witness tracks the balance" 40 !(w.balance_cell)
;;

let is_balance_11 n = function
  | Sec_3_11.Balance m -> n = m
  | Insufficient_funds -> false
;;

let ex_3_11_per_account_state () =
  let deposited, withdrawn, acc2_balance, distinct = Sec_3_11.ex_3_11 () in
  Alcotest.(check int) "deposit answers 90 after make_account 50" 90 deposited;
  Alcotest.(check bool) "withdrawal answers Balance 30" true (is_balance_11 30 withdrawn);
  Alcotest.(check int) "second account untouched" 100 acc2_balance;
  Alcotest.(check bool) "the two records are distinct objects" true distinct
;;

let ex_3_11_state_persists_across_calls () =
  let acc = Sec_3_11.make_account 10 in
  ignore (acc.deposit 5);
  ignore (acc.deposit 5);
  Alcotest.(check bool)
    "the same cell answers across calls"
    true
    (is_balance_11 17 (acc.withdraw 3))
;;

let ex_3_11a_identity_vs_equality () =
  let distinct, aliased, raises, same_behavior, still_distinct = Sec_3_11.ex_3_11a () in
  Alcotest.(check bool) "separate calls build different objects" false distinct;
  Alcotest.(check bool) "an alias is the same object" true aliased;
  Alcotest.(check bool) "structural comparison of closures raises" true raises;
  Alcotest.(check bool) "the twins answer the same withdrawal alike" true same_behavior;
  Alcotest.(check bool) "and remain different objects" true still_distinct
;;

let ex_3_11a_alias_shares_state () =
  let acc = Sec_3_11.make_account 100 in
  let alias = acc in
  ignore (alias.deposit 40);
  Alcotest.(check bool)
    "the alias's deposit reached acc's cell"
    true
    (is_balance_11 120 (acc.withdraw 20))
;;

let () =
  Alcotest.run
    "sicp_ch3 solutions, section 3.2"
    [ ( "3.9 factorial stack growth"
      , [ Alcotest.test_case "statement's instrumented runs" `Quick ex_3_09_statement_runs
        ; Alcotest.test_case
            "traced versions agree at depth 40"
            `Quick
            ex_3_09_traced_agrees_deeper
        ; Alcotest.test_case
            "million-step tail loop stays flat"
            `Quick
            ex_3_09_constant_space_at_scale
        ] )
    ; ( "3.10 captured cell lifetime"
      , [ Alcotest.test_case "physical-equality checks" `Quick ex_3_10_cell_witness
        ; Alcotest.test_case
            "witness tracks the balance"
            `Quick
            ex_3_10_witness_tracks_balance
        ] )
    ; ( "3.11 per-account closure state"
      , [ Alcotest.test_case "statement's interactions" `Quick ex_3_11_per_account_state
        ; Alcotest.test_case
            "state persists across calls"
            `Quick
            ex_3_11_state_persists_across_calls
        ] )
    ; ( "3.11a identity versus equality"
      , [ Alcotest.test_case
            "the statement's checklist"
            `Quick
            ex_3_11a_identity_vs_equality
        ; Alcotest.test_case "alias shares one cell" `Quick ex_3_11a_alias_shares_state
        ] )
    ]
;;
