(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 3.1: for any starting value and any sequence of amounts, an
   accumulator's successive answers are the running prefix sums of the
   sequence over the starting value. The unit spot check in
   [test_sec_3_1.ml] covers only the statement's own two-call example. *)

module Sec_3_1 = Sicp_ch3_solutions.Sec_3_1
open QCheck2

let small_int = Gen.int_range (-1_000) 1_000
let amounts = Gen.list_size (Gen.int_range 0 50) small_int

let accumulator_returns_running_prefix_sums =
  Test.make
    ~name:"make_accumulator answers the running prefix sum of every call"
    ~count:200
    ~print:(fun (initial, xs) ->
      Printf.sprintf
        "initial=%d, xs=[%s]"
        initial
        (String.concat "; " (List.map string_of_int xs)))
    (Gen.pair small_int amounts)
    (fun (initial, xs) ->
       let accumulator = Sec_3_1.make_accumulator initial in
       let answers = List.map accumulator xs in
       let running = ref initial in
       let expected =
         List.map
           (fun amount ->
              running := !running + amount;
              !running)
           xs
       in
       answers = expected)
;;

let () = QCheck_base_runner.run_tests_main [ accumulator_returns_running_prefix_sums ]
