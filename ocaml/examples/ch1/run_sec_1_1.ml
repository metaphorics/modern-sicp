(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_1_1 = Sicp_ch1.Sec_1_1
open Sec_1_1

let expect_int n = Replay.expect (string_of_int n)
let expect_bool b = Replay.expect (string_of_bool b)

let () =
  expect_int Expressions.forty_eight_six "486";
  expect_int Expressions.sum "486";
  expect_int Expressions.difference "666";
  expect_int Expressions.product "495";
  expect_int Expressions.quotient "2";
  Replay.expect_float Expressions.mixed "12.7";
  expect_int Expressions.chained_sum "75";
  expect_int Expressions.chained_product "1200";
  expect_int Expressions.nested "19";
  expect_int Expressions.essential_parens "9";
  expect_int Expressions.deep "57";
  expect_int Naming.size "2";
  expect_int Naming.five_times_size "10";
  Replay.expect_float Naming.area "314.159";
  Replay.expect_float Naming.circumference "62.8318";
  expect_int (Compound.square 21) "441";
  expect_int (Compound.square (2 + 5)) "49";
  expect_int (Compound.square (Compound.square 3)) "81";
  expect_int (Compound.sum_of_squares 3 4) "25";
  expect_int (Compound.f 5) "136";
  expect_int (Conditionals.abs_cases 5) "5";
  expect_int (Conditionals.abs_cases 0) "0";
  expect_int (Conditionals.abs_cases (-7)) "7";
  expect_int (Conditionals.abs_two_way (-7)) "7";
  expect_int (Conditionals.abs_match (-7)) "7";
  expect_bool (Conditionals.in_range 7) "true";
  expect_bool (Conditionals.in_range 11) "false";
  expect_bool (Conditionals.greater_or_equal 5 4) "true";
  expect_bool (Conditionals.greater_or_equal 4 5) "false";
  expect_bool (Conditionals.greater_or_equal 4 4) "true";
  expect_bool (Conditionals.greater_or_equal_not 5 4) "true";
  expect_bool (Conditionals.greater_or_equal_not 4 5) "false";
  expect_bool (Conditionals.greater_or_equal_not 4 4) "true";
  Replay.expect_float (Sqrt.sqrt 9.0) "3.00009155413138";
  Replay.expect_float (Sqrt.sqrt (100.0 +. 37.0)) "11.704699917758145";
  Replay.expect_float (Sqrt.sqrt (Sqrt.sqrt 2.0 +. Sqrt.sqrt 3.0)) "1.7739279023207892";
  Replay.expect_float (Sqrt.square (Sqrt.sqrt 1000.0)) "1000.000369924366";
  Replay.expect_float (Sqrt.sqrt_block 9.0) "3.00009155413138";
  Replay.expect_float (Black_box.square 21.0) "441";
  Replay.expect_float (Black_box.square_via_exp 21.0) "441";
  Replay.expect_float (Black_box.square_via_exp 7.0) "48.99999999999999";
  Replay.expect_float (Black_box.square_via_exp 5.0) "24.999999999999996"
;;
