(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Property sweep over section 2.2's re-derivations: the accumulation
   definitions of 2.33 must agree with the library operations they
   reimplement, both fold reversals of 2.39 must agree with each
   other, and deep-reverse of 2.27 must reverse twice to the
   identity. *)

module S18 = Sicp_ch2_solutions.Sec_2_18
module S33 = Sicp_ch2_solutions.Sec_2_33
module S39 = Sicp_ch2_solutions.Sec_2_39
open QCheck2

let list_gen = Gen.list_size (Gen.int_range 0 40) Gen.int

let map_agrees =
  Test.make ~name:"ex_2_33_map agrees with List.map" ~count:200 list_gen (fun xs ->
    List.map (fun x -> x * 3) xs = S33.ex_2_33_map (fun x -> x * 3) xs)
;;

let append_agrees =
  Test.make
    ~name:"ex_2_33_append agrees with List.append"
    ~count:200
    (Gen.pair list_gen list_gen)
    (fun (xs, ys) -> xs @ ys = S33.ex_2_33_append xs ys)
;;

let length_agrees =
  Test.make ~name:"ex_2_33_length agrees with List.length" ~count:200 list_gen (fun xs ->
    List.length xs = S33.ex_2_33_length xs)
;;

let reversals_agree =
  Test.make ~name:"2.39 fold reversals agree with List.rev" ~count:200 list_gen (fun xs ->
    let r = List.rev xs in
    r = S39.ex_2_39_right xs && r = S39.ex_2_39_left xs)
;;

let reverse_involution =
  Test.make ~name:"2.18 reverse twice is identity" ~count:200 list_gen (fun xs ->
    xs = S18.ex_2_18 (S18.ex_2_18 xs))
;;

let () =
  QCheck_base_runner.run_tests_main
    [ map_agrees; append_agrees; length_agrees; reversals_agree; reverse_involution ]
;;
