(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* An Alcotest suite over the reference solutions' public contracts:
   [sicp_ch0_solutions] implements the same signatures as
   [sicp_ch0_exercises], so these tests double as the proof that each
   exercise's stated answer is correct, independent of the exercises
   library's [Pending_solution] stubs. *)

module Ex_0_01 = Sicp_ch0_solutions.Ex_0_01
module Ex_0_02 = Sicp_ch0_solutions.Ex_0_02
module Ex_0_03 = Sicp_ch0_solutions.Ex_0_03
module Ex_0_04 = Sicp_ch0_solutions.Ex_0_04
module Ex_0_05 = Sicp_ch0_solutions.Ex_0_05

let float3 = Alcotest.float 0.0001

let ex_0_01_five_interactions () =
  Alcotest.(check int) "sum_with_product" 26 Ex_0_01.sum_with_product;
  Alcotest.(check int) "square 7" 49 (Ex_0_01.square 7);
  Alcotest.(check int) "seven_squared" 49 Ex_0_01.seven_squared;
  Alcotest.(check int) "larger 4 11" 11 (Ex_0_01.larger 4 11);
  Alcotest.(check int) "larger 11 4" 11 (Ex_0_01.larger 11 4);
  Alcotest.(check int) "larger_of_4_and_11" 11 Ex_0_01.larger_of_4_and_11;
  Alcotest.(check int) "anonymous_square" 25 Ex_0_01.anonymous_square
;;

let ex_0_02_sum_cubes_agree () =
  Alcotest.(check int) "sum_cubes_rec 1 10" 3025 (Ex_0_02.sum_cubes_rec 1 10);
  Alcotest.(check int) "sum_cubes_fold 1 10" 3025 (Ex_0_02.sum_cubes_fold 1 10);
  Alcotest.(check int)
    "sum_cubes_rec and sum_cubes_fold agree on a single-element range"
    (Ex_0_02.sum_cubes_rec 4 4)
    (Ex_0_02.sum_cubes_fold 4 4);
  Alcotest.(check int)
    "sum_cubes_rec and sum_cubes_fold agree on the empty range"
    (Ex_0_02.sum_cubes_rec 5 1)
    (Ex_0_02.sum_cubes_fold 5 1);
  Alcotest.(check int)
    "sum_cubes_rec on the empty range is 0"
    0
    (Ex_0_02.sum_cubes_rec 5 1);
  Alcotest.(check int)
    "sum_cubes_rec and sum_cubes_fold agree on a range straddling zero"
    (Ex_0_02.sum_cubes_rec (-2) 2)
    (Ex_0_02.sum_cubes_fold (-2) 2)
;;

let ex_0_03_every_constructor () =
  Alcotest.(check float3)
    "area (Circle 2.0)"
    12.5663706143591725
    (Ex_0_03.area (Ex_0_03.Circle 2.0));
  Alcotest.(check float3)
    "area (Rectangle (3.0, 4.0))"
    12.0
    (Ex_0_03.area (Ex_0_03.Rectangle (3.0, 4.0)));
  Alcotest.(check float3)
    "area (Triangle (6.0, 4.0))"
    12.0
    (Ex_0_03.area (Ex_0_03.Triangle (6.0, 4.0)))
;;

let ex_0_04_independent_accounts () =
  let account = Ex_0_04.make_account 100.0 in
  Alcotest.(check float3) "deposit 50" 150.0 (account.deposit 50.0);
  Alcotest.(check float3) "withdraw 30 after deposit" 120.0 (account.withdraw 30.0);
  Alcotest.(check float3) "balance reads the same cell" 120.0 (account.balance ())
;;

let ex_0_04_shared_accounts_alias () =
  let first, second = Ex_0_04.make_shared_accounts 100.0 in
  Alcotest.(check float3) "deposit 50 through first" 150.0 (first.deposit 50.0);
  Alcotest.(check float3) "second observes first's deposit" 150.0 (second.balance ());
  Alcotest.(check float3) "withdraw 20 through second" 130.0 (second.withdraw 20.0);
  Alcotest.(check float3) "first observes second's withdrawal" 130.0 (first.balance ())
;;

let ex_0_05_age_summary () =
  Alcotest.(check string)
    "age_summary \"30\""
    "30 is a fine age"
    (Ex_0_05.age_summary "30");
  Alcotest.(check string)
    "age_summary \"abc\""
    "age check failed: not a number: abc"
    (Ex_0_05.age_summary "abc");
  Alcotest.(check string)
    "age_summary \"200\""
    "age check failed: 200 is out of range"
    (Ex_0_05.age_summary "200")
;;

let () =
  Alcotest.run
    "sicp_ch0 solutions"
    [ ( "0.1 five Scheme interactions"
      , [ Alcotest.test_case "translation" `Quick ex_0_01_five_interactions ] )
    ; ( "0.2 sum_cubes twice"
      , [ Alcotest.test_case "recursion and fold agree" `Quick ex_0_02_sum_cubes_agree ] )
    ; ( "0.3 shape variant"
      , [ Alcotest.test_case "every constructor" `Quick ex_0_03_every_constructor ] )
    ; ( "0.4 make_account"
      , [ Alcotest.test_case "independent accounts" `Quick ex_0_04_independent_accounts
        ; Alcotest.test_case "shared-ref aliasing" `Quick ex_0_04_shared_accounts_alias
        ] )
    ; ( "0.5 Result.bind chain"
      , [ Alcotest.test_case "age_summary" `Quick ex_0_05_age_summary ] )
    ]
;;
