(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 1.1. [sicp_ch1_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. The two exercises
   whose solved forms diverge by design (1.5 and 1.6) are tested only
   through terminating calls and through evaluation-order probes that
   observe eagerness with a counter, never through the diverging
   calls themselves. *)

module Sec_1_1 = Sicp_ch1_solutions.Sec_1_1
module Sec_1_2 = Sicp_ch1_solutions.Sec_1_2
module Sec_1_3 = Sicp_ch1_solutions.Sec_1_3
module Sec_1_4 = Sicp_ch1_solutions.Sec_1_4
module Sec_1_5 = Sicp_ch1_solutions.Sec_1_5
module Sec_1_6 = Sicp_ch1_solutions.Sec_1_6
module Sec_1_7 = Sicp_ch1_solutions.Sec_1_7
module Sec_1_8 = Sicp_ch1_solutions.Sec_1_8

let float6 = Alcotest.float 1e-6

let ex_1_01_sequence () =
  let values, equality = Sec_1_1.ex_1_01 () in
  Alcotest.(check (list int))
    "the ten value-producing results in order"
    [ 10; 12; 8; 3; 6; 19; 4; 16; 6; 16 ]
    values;
  Alcotest.(check bool) "a = b is false" false equality
;;

let ex_1_02_nested_calls () =
  Alcotest.(check float6) "the fraction's value" (-37.0 /. 150.0) (Sec_1_2.ex_1_02 ())
;;

let ex_1_03_two_larger () =
  Alcotest.(check int) "1 2 3" 13 (Sec_1_3.ex_1_03 1 2 3);
  Alcotest.(check int) "3 2 1" 13 (Sec_1_3.ex_1_03 3 2 1);
  Alcotest.(check int) "2 1 2 keeps both twos" 8 (Sec_1_3.ex_1_03 2 1 2);
  Alcotest.(check int) "ties for smallest" 5 (Sec_1_3.ex_1_03 2 1 1);
  Alcotest.(check int) "all equal" 18 (Sec_1_3.ex_1_03 3 3 3);
  Alcotest.(check int) "negatives" 13 (Sec_1_3.ex_1_03 (-2) (-3) (-4))
;;

let ex_1_04_operator_choice () =
  Alcotest.(check int) "b positive adds" 7 (Sec_1_4.ex_1_04 3 4);
  Alcotest.(check int) "b negative subtracts" 7 (Sec_1_4.ex_1_04 3 (-4));
  Alcotest.(check int) "b zero subtracts" 3 (Sec_1_4.ex_1_04 3 0);
  Alcotest.(check int) "negative a" (-1) (Sec_1_4.ex_1_04 (-3) 2)
;;

let ex_1_05_test_itself () =
  Alcotest.(check int) "test 0 9" 0 (Sec_1_5.ex_1_05_test 0 9);
  Alcotest.(check int) "test 1 9" 9 (Sec_1_5.ex_1_05_test 1 9)
;;

let ex_1_05_arguments_evaluated_first () =
  let counter = ref 0 in
  let result =
    Sec_1_5.ex_1_05_test
      0
      (incr counter;
       42)
  in
  Alcotest.(check int) "test still answers 0" 0 result;
  Alcotest.(check int)
    "the unneeded argument was evaluated anyway, so the call order is applicative"
    1
    !counter
;;

let ex_1_06_new_if_values () =
  Alcotest.(check int) "new_if (2 = 3) 0 5" 5 (Sec_1_6.ex_1_06_new_if (2 = 3) 0 5);
  Alcotest.(check int) "new_if (1 = 1) 0 5" 0 (Sec_1_6.ex_1_06_new_if (1 = 1) 0 5)
;;

let ex_1_06_new_if_is_not_a_special_form () =
  let counter = ref 0 in
  let result =
    Sec_1_6.ex_1_06_new_if
      true
      1
      (incr counter;
       2)
  in
  Alcotest.(check int) "the chosen branch is returned" 1 result;
  Alcotest.(check int)
    "the unchosen branch was evaluated anyway, so Alyssa's sqrt_iter can never settle"
    1
    !counter
;;

let ex_1_07_absolute_tolerance_fails_small () =
  let answer = Sec_1_7.ex_1_07_sqrt 0.0001 in
  Alcotest.(check bool)
    "the fixed 0.001 tolerance stops far from the true root 0.01"
    true
    (Float.abs (answer -. 0.01) > 0.005)
;;

let ex_1_07_relative_test_handles_both_ends () =
  Alcotest.(check float6) "small radicand" 0.01 (Sec_1_7.ex_1_07_sqrt_improved 0.0001);
  Alcotest.(check float6) "ordinary radicand" 3.0 (Sec_1_7.ex_1_07_sqrt_improved 9.0);
  let large = Sec_1_7.ex_1_07_sqrt_improved 1e12 in
  Alcotest.(check bool)
    "large radicand lands within a relative hair of 1e6"
    true
    (Float.abs (large -. 1e6) < 1e3)
;;

let ex_1_08_cube_roots () =
  Alcotest.(check float6) "cube root 27" 3.0 (Sec_1_8.ex_1_08_cube_root 27.0);
  Alcotest.(check float6) "cube root 8" 2.0 (Sec_1_8.ex_1_08_cube_root 8.0);
  Alcotest.(check float6) "cube root 0 is guarded" 0.0 (Sec_1_8.ex_1_08_cube_root 0.0);
  let root = Sec_1_8.ex_1_08_cube_root 1000.0 in
  Alcotest.(check bool)
    "the cube of the answer comes back to 1000"
    true
    (Float.abs ((root *. root *. root) -. 1000.0) < 0.01)
;;

let () =
  Alcotest.run
    "sicp_ch1 solutions, section 1.1"
    [ ( "1.1 expression sequence"
      , [ Alcotest.test_case "results in order" `Quick ex_1_01_sequence ] )
    ; ( "1.2 pure nested calls"
      , [ Alcotest.test_case "fraction value" `Quick ex_1_02_nested_calls ] )
    ; ( "1.3 sum of two larger squares"
      , [ Alcotest.test_case "orders and ties" `Quick ex_1_03_two_larger ] )
    ; ( "1.4 operator chosen by conditional"
      , [ Alcotest.test_case "sign of b picks the operator" `Quick ex_1_04_operator_choice
        ] )
    ; ( "1.5 evaluation order"
      , [ Alcotest.test_case "test on its own" `Quick ex_1_05_test_itself
        ; Alcotest.test_case
            "arguments evaluated before the call"
            `Quick
            ex_1_05_arguments_evaluated_first
        ] )
    ; ( "1.6 new_if as ordinary function"
      , [ Alcotest.test_case "demo values" `Quick ex_1_06_new_if_values
        ; Alcotest.test_case
            "both branches evaluated"
            `Quick
            ex_1_06_new_if_is_not_a_special_form
        ] )
    ; ( "1.7 tolerance at the extremes"
      , [ Alcotest.test_case
            "absolute tolerance fails for small radicands"
            `Quick
            ex_1_07_absolute_tolerance_fails_small
        ; Alcotest.test_case
            "relative change test handles both ends"
            `Quick
            ex_1_07_relative_test_handles_both_ends
        ] )
    ; ( "1.8 cube roots"
      , [ Alcotest.test_case "Newton iteration" `Quick ex_1_08_cube_roots ] )
    ]
;;
