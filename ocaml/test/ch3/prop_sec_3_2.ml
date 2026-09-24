(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Properties over the reference solutions of section 3.2. The unit
   spot checks in [test_sec_3_2.ml] cover the statements' own runs;
   these generalize them over arbitrary inputs. *)

module Sec_3_9 = Sicp_ch3_solutions.Sec_3_9
module Sec_3_10 = Sicp_ch3_solutions.Sec_3_10
module Sec_3_11 = Sicp_ch3_solutions.Sec_3_11
open QCheck2

let factorial_versions_agree =
  Test.make
    ~name:"both factorials compute the same value, with the drawn depths"
    ~count:200
    (Gen.int_range 1 300)
    (fun n ->
       let rec_value, rec_depth = Sec_3_9.factorial_traced n in
       let iter_value, iter_depth = Sec_3_9.factorial_iter_traced n in
       rec_value = iter_value && rec_depth = n && iter_depth = 1)
;;

let make_withdraw_answers_running_balance =
  Test.make
    ~name:"make_withdraw answers the running balance, refusing overdrafts"
    ~count:200
    (Gen.pair
       (Gen.int_range 0 200)
       (Gen.list_size (Gen.int_range 0 50) (Gen.int_range 0 300)))
    (fun (initial, amounts) ->
       let w = Sec_3_10.make_withdraw initial in
       let running = ref initial in
       List.for_all
         (fun amount ->
            let expected =
              if !running >= amount
              then (
                running := !running - amount;
                Sec_3_10.Balance !running)
              else Sec_3_10.Insufficient_funds
            in
            w amount = expected)
         amounts)
;;

let make_account_objects_are_distinct_but_stable =
  Test.make
    ~name:"every make_account call builds a new object that stays itself"
    ~count:100
    (Gen.int_range 0 1000)
    (fun initial ->
       let a = Sec_3_11.make_account initial in
       let b = Sec_3_11.make_account initial in
       (not (a == b))
       && a == a
       && b == b
       && a.deposit 0 = initial
       && b.deposit 0 = initial)
;;

let () =
  QCheck_base_runner.run_tests_main
    [ factorial_versions_agree
    ; make_withdraw_answers_running_balance
    ; make_account_objects_are_distinct_but_stable
    ]
;;
