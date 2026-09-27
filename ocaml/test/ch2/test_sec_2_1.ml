(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 2.1. [sicp_ch2_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module Sec_2_1 = Sicp_ch2_solutions.Sec_2_1
module Sec_2_2 = Sicp_ch2_solutions.Sec_2_2
module Sec_2_3 = Sicp_ch2_solutions.Sec_2_3
module Sec_2_4 = Sicp_ch2_solutions.Sec_2_4
module Sec_2_5 = Sicp_ch2_solutions.Sec_2_5
module Sec_2_6 = Sicp_ch2_solutions.Sec_2_6
module Sec_2_7 = Sicp_ch2_solutions.Sec_2_7
module Sec_2_8 = Sicp_ch2_solutions.Sec_2_8
module Sec_2_9 = Sicp_ch2_solutions.Sec_2_9
module Sec_2_10 = Sicp_ch2_solutions.Sec_2_10
module Sec_2_11 = Sicp_ch2_solutions.Sec_2_11
module Sec_2_12 = Sicp_ch2_solutions.Sec_2_12
module Sec_2_13 = Sicp_ch2_solutions.Sec_2_13
module Sec_2_14 = Sicp_ch2_solutions.Sec_2_14
module Sec_2_15 = Sicp_ch2_solutions.Sec_2_15
module Sec_2_16 = Sicp_ch2_solutions.Sec_2_16

let float6 = Alcotest.float 1e-6

let ex_2_01_sign_normalizes () =
  Alcotest.(check (pair int int))
    "(-4)/(-6)"
    (2, 3)
    (Result.get_ok (Sec_2_1.ex_2_01 (-4) (-6)));
  Alcotest.(check (pair int int))
    "4/(-6)"
    (-2, 3)
    (Result.get_ok (Sec_2_1.ex_2_01 4 (-6)));
  match Sec_2_1.ex_2_01 3 0 with
  | Error (Sec_2_1.Zero_denominator 3) -> ()
  | Error (Sec_2_1.Zero_denominator n) ->
    Alcotest.failf "expected rejected numerator 3, got %d" n
  | Ok _ -> Alcotest.fail "expected Error, got Ok"
;;

let ex_2_02_midpoint () =
  Alcotest.(check (pair float6 float6)) "midpoint" (2.0, 3.0) (Sec_2_2.ex_2_02 ())
;;

let ex_2_03_two_representations () =
  let (p1, a1), (p2, a2) = Sec_2_3.ex_2_03 () in
  Alcotest.(check float6) "perimeter, two-corners" 14.0 p1;
  Alcotest.(check float6) "area, two-corners" 12.0 a1;
  Alcotest.(check float6) "perimeter, corner-and-dims" 14.0 p2;
  Alcotest.(check float6) "area, corner-and-dims" 12.0 a2
;;

let ex_2_04_lambda_pairs () =
  Alcotest.(check bool) "car/cdr recover x/y" true (Sec_2_4.ex_2_04 3 4)
;;

let ex_2_05_two_three_powers () =
  Alcotest.(check bool) "car/cdr recover a/b" true (Sec_2_5.ex_2_05 3 4)
;;

let ex_2_06_church_add () = Alcotest.(check int) "one + two" 3 (Sec_2_6.ex_2_06 ())

let ex_2_07_bounds () =
  Alcotest.(check (pair float6 float6))
    "6.8-ohm 10% resistor bounds"
    (6.12, 7.48)
    (Sec_2_7.ex_2_07 ())
;;

let ex_2_08_subtraction () =
  Alcotest.(check (pair float6 float6))
    "resistor difference"
    (1.185, 3.015)
    (Sec_2_8.ex_2_08 ())
;;

let ex_2_09_width_algebra () =
  Alcotest.(check (pair float6 float6))
    "same-width factors, different product widths"
    (13.0, 17.0)
    (Sec_2_9.ex_2_09 ())
;;

let ex_2_10_zero_spanning_divisor () =
  match Sec_2_10.ex_2_10 () with
  | Error (Sec_2_10.Spans_zero (-1.0, 1.0)) -> ()
  | Error (Sec_2_10.Spans_zero (lo, hi)) ->
    Alcotest.failf "expected [-1, 1], got [%g, %g]" lo hi
  | Ok _ -> Alcotest.fail "expected Error, got Ok"
;;

let ex_2_11_nine_case_matches_naive () =
  Alcotest.(check bool) "nine-case agrees with naive" true (Sec_2_11.ex_2_11 ())
;;

let ex_2_12_center_percent () =
  Alcotest.(check (pair float6 float6))
    "center and percent"
    (6.8, 10.0)
    (Sec_2_12.ex_2_12 ())
;;

let ex_2_12a_interfaces_agree () =
  Alcotest.(check bool) "endpoint and center-width agree" true (Sec_2_12.ex_2_12a ())
;;

let ex_2_13_product_tolerance () =
  let actual, approx = Sec_2_13.ex_2_13 () in
  Alcotest.(check (Alcotest.float 1e-3))
    "small-tolerance product approximation"
    approx
    actual
;;

let ex_2_14_par1_par2_disagree () =
  let percent_a_over_a, percent_a_over_b, percent_par1, percent_par2 =
    Sec_2_14.ex_2_14 ()
  in
  Alcotest.(check bool)
    "A / A is not the exact interval [1, 1]"
    true
    (percent_a_over_a > 0.0);
  Alcotest.(check bool) "A / B has nonzero tolerance too" true (percent_a_over_b > 0.0);
  Alcotest.(check bool) "par1 is wider than par2" true (percent_par1 > percent_par2)
;;

let ex_2_15_par2_tighter () =
  let percent_par1, percent_par2 = Sec_2_15.ex_2_15 () in
  Alcotest.(check bool)
    "par2's tolerance is tighter than par1's"
    true
    (percent_par2 < percent_par1)
;;

let ex_2_16_repeated_variable () =
  let lo, hi = Sec_2_16.ex_2_16 () in
  Alcotest.(check bool) "A - A is not [0, 0] for nonzero width" true (lo < 0.0 && hi > 0.0)
;;

let () =
  Alcotest.run
    "sicp_ch2 solutions, section 2.1"
    [ ( "2.1 sign-normalizing make-rat"
      , [ Alcotest.test_case
            "normalizes and refuses zero denominator"
            `Quick
            ex_2_01_sign_normalizes
        ] )
    ; ( "2.2 line segments from points"
      , [ Alcotest.test_case "midpoint" `Quick ex_2_02_midpoint ] )
    ; ( "2.3 two rectangle representations"
      , [ Alcotest.test_case "perimeter and area agree" `Quick ex_2_03_two_representations
        ] )
    ; ( "2.4 procedural cons, car, cdr"
      , [ Alcotest.test_case "lambda pairs" `Quick ex_2_04_lambda_pairs ] )
    ; ( "2.5 pairs as 2^a 3^b"
      , [ Alcotest.test_case "prime-power encoding" `Quick ex_2_05_two_three_powers ] )
    ; ( "2.6 Church numerals"
      , [ Alcotest.test_case "one + two = three" `Quick ex_2_06_church_add ] )
    ; "2.7 interval selectors", [ Alcotest.test_case "bounds" `Quick ex_2_07_bounds ]
    ; ( "2.8 sub-interval"
      , [ Alcotest.test_case "resistor difference" `Quick ex_2_08_subtraction ] )
    ; ( "2.9 interval width algebra"
      , [ Alcotest.test_case
            "multiplication is not width-only"
            `Quick
            ex_2_09_width_algebra
        ] )
    ; ( "2.10 divide by a zero-spanning interval"
      , [ Alcotest.test_case "signals an error" `Quick ex_2_10_zero_spanning_divisor ] )
    ; ( "2.11 nine-case multiplication"
      , [ Alcotest.test_case
            "matches naive version"
            `Quick
            ex_2_11_nine_case_matches_naive
        ] )
    ; ( "2.12 center-percent constructor"
      , [ Alcotest.test_case "center and percent" `Quick ex_2_12_center_percent ] )
    ; ( "2.12a endpoint and center-width interfaces agree"
      , [ Alcotest.test_case "add_interval matches" `Quick ex_2_12a_interfaces_agree ] )
    ; ( "2.13 percentage tolerance formula"
      , [ Alcotest.test_case "approximate additivity" `Quick ex_2_13_product_tolerance ] )
    ; ( "2.14 par1 versus par2"
      , [ Alcotest.test_case
            "repeated variables widen the answer"
            `Quick
            ex_2_14_par1_par2_disagree
        ] )
    ; ( "2.15 par2 is tighter"
      , [ Alcotest.test_case "non-repeating formula wins" `Quick ex_2_15_par2_tighter ] )
    ; ( "2.16 equivalent expressions, different answers"
      , [ Alcotest.test_case "A - A is not [0, 0]" `Quick ex_2_16_repeated_variable ] )
    ]
;;
