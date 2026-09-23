(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_1_2 = Sicp_ch1.Sec_1_2
open Sec_1_2

let expect_int n = Replay.expect (string_of_int n)
let expect_bool b = Replay.expect (string_of_bool b)

let seeded_generator () =
  match Sicp_common.Random.create 42L with
  | Ok gen -> gen
  | Error Sicp_common.Error.Zero_seed -> failwith "the fixed seed 42 is nonzero"
;;

let () =
  expect_int (Factorial.recursive 6) "720";
  expect_int (Factorial.iterative 6) "720";
  expect_int (Factorial.hidden 6) "720";
  expect_int (Factorial.recursive 20) "2432902008176640000";
  expect_int (Factorial.recursive 21) "-4249290049419214848";
  expect_int (Fibonacci.recursive 10) "55";
  expect_int (Fibonacci.iterative 10) "55";
  expect_int (Fibonacci.iterative 90) "2880067194370816120";
  expect_int (Counting_change.count_change 100) "292";
  expect_int (Exponentiation.expt 2 10) "1024";
  expect_int (Exponentiation.expt_iterative 2 10) "1024";
  expect_int (Exponentiation.fast_expt 2 10) "1024";
  expect_int (Exponentiation.fast_expt 3 5) "243";
  expect_int (Gcd.gcd 206 40) "2";
  expect_int (Primality.smallest_divisor 199) "199";
  expect_int (Primality.smallest_divisor 1999) "1999";
  expect_bool (Primality.is_prime 199) "true";
  expect_bool (Primality.is_prime 100) "false";
  let gen = seeded_generator () in
  expect_bool (Primality.fermat_test 7 gen) "true";
  expect_bool (Primality.fast_prime 97 10 gen) "true";
  expect_bool (Primality.fast_prime 100 10 gen) "false"
;;
