(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Properties over the reference solutions of section 3.5. The unit
   spot checks in [test_sec_3_5.ml] pin the deterministic values; these
   assert the section's invariants -- memoization arithmetic, order
   laws of the combinators, and the contracts the exercise answers
   claim -- over random prefixes and inputs, because a single prefix
   can satisfy them by accident. *)

module Sec_3_50 = Sicp_ch3_solutions.Sec_3_50
module Sec_3_55 = Sicp_ch3_solutions.Sec_3_55
module Sec_3_56 = Sicp_ch3_solutions.Sec_3_56
module Sec_3_57 = Sicp_ch3_solutions.Sec_3_57
module Sec_3_58 = Sicp_ch3_solutions.Sec_3_58
module Sec_3_64 = Sicp_ch3_solutions.Sec_3_64
module Sec_3_67 = Sicp_ch3_solutions.Sec_3_67
module Sec_3_81 = Sicp_ch3_solutions.Sec_3_81
module Sec_3_82 = Sicp_ch3_solutions.Sec_3_82
module S = Sicp_ch3.Sec_3_5.Streams
module I = Sicp_ch3.Sec_3_5.Infinite
module P = Sicp_ch3.Sec_3_5.Pairs
open QCheck2

(* [hamming_smooth] holds when the prefix is strictly increasing and
   every element has no prime factor outside 2, 3, 5. *)
let hamming_smooth xs =
  let smooth n =
    let rec strip n p = if n mod p = 0 then strip (n / p) p else n in
    strip (strip (strip n 2) 3) 5 = 1
  in
  let rec sorted_desc = function
    | [] | [ _ ] -> true
    | a :: (b :: _ as rest) -> a < b && sorted_desc rest
  in
  sorted_desc xs && List.for_all smooth xs
;;

let multi_map_agrees_with_the_pairwise_one =
  Test.make
    ~name:"the n-ary stream map over two integer streams is the elementwise sum"
    ~count:100
    (Gen.int_range 1 40)
    (fun n ->
       let via_multi =
         S.stream_take
           n
           (Sec_3_50.stream_map_multi
              (List.fold_left ( + ) 0)
              [ I.integers; I.integers_starting_from 2 ])
       in
       let via_pair =
         S.stream_take n (I.add_streams I.integers (I.integers_starting_from 2))
       in
       via_multi = via_pair)
;;

let memo_proc_runs_the_body_once =
  Test.make
    ~name:"n forces of a memoized thunk run the body exactly once"
    ~count:60
    (Gen.int_range 1 50)
    (fun n ->
       let runs = ref 0 in
       let memo =
         Sicp_ch3.Sec_3_5.Memo.memo_proc (fun () ->
           incr runs;
           7)
       in
       for _ = 1 to n do
         ignore (memo ())
       done;
       !runs = 1)
;;

let partial_sums_is_the_running_total =
  Test.make
    ~name:"element k of partial-sums integers is the sum of the first k+1 integers"
    ~count:100
    (Gen.int_range 0 29)
    (fun k ->
       S.stream_ref (Sec_3_55.partial_sums I.integers) k
       = List.fold_left ( + ) 0 (S.stream_take (k + 1) I.integers))
;;

let hamming_prefixes_are_ordered_and_smooth =
  Test.make
    ~name:"hamming prefixes are strictly increasing 2-3-5-smooth numbers"
    ~count:40
    (Gen.int_range 20 80)
    (fun n -> hamming_smooth (S.stream_take n Sec_3_56.hamming))
;;

let expand_shifts_by_whole_units =
  Test.make
    ~name:
      "adding one whole unit to the numerator shifts the digit stream's head by the radix"
    ~count:100
    (Gen.pair (Gen.int_range 1 50) (Gen.int_range 2 30))
    (fun (num, den) ->
       let base = S.stream_take 5 (Sec_3_58.expand num den 10) in
       let shifted = S.stream_take 5 (Sec_3_58.expand (num + den) den 10) in
       match base, shifted with
       | b :: base_rest, s :: shifted_rest -> s = b + 10 && base_rest = shifted_rest
       | _ -> false)
;;

let memoized_fib_additions_are_linear_thunk_additions_exponential =
  Test.make
    ~name:"memoized fibs performs n-2 additions while plain thunks exceed twice that"
    ~count:60
    (Gen.int_range 8 25)
    (fun n ->
       Sec_3_57.memoized_additions n = n - 2 && Sec_3_57.thunk_additions n >= 2 * (n - 2))
;;

let stream_limit_lands_within_the_tolerance =
  Test.make
    ~name:"sqrt via stream-limit is within the tolerance of the host root"
    ~count:60
    (Gen.int_range 1 100)
    (fun x ->
       Float.abs (Sec_3_64.sqrt (float_of_int x) 1e-4 -. Float.sqrt (float_of_int x))
       < 1e-4)
;;

let all_pairs_carries_both_orders =
  Test.make
    ~name:"the all-pairs stream eventually contains (i, j) and (j, i)"
    ~count:60
    (Gen.pair (Gen.int_range 1 5) (Gen.int_range 1 5))
    (fun (i, j) ->
       let prefix = S.stream_take 500 (Sec_3_67.pairs_all I.integers I.integers) in
       List.mem (i, j) prefix && List.mem (j, i) prefix)
;;

let interleave_reaches_the_second_stream =
  Test.make
    ~name:
      "interleaving the integers with ones reaches every early element of the second \
       stream"
    ~count:40
    (Gen.int_range 1 30)
    (fun j -> List.mem j (S.stream_take 120 (P.interleave I.integers I.ones)))
;;

let same_script_replays_the_same_answers =
  Test.make
    ~name:"a random request script from the same seed yields the identical answer stream"
    ~count:60
    (Gen.list_size
       (Gen.int_range 1 25)
       (Gen.map
          (fun k -> if k = 0 then Sec_3_81.Generate else Sec_3_81.Reset (Int64.of_int k))
          (Gen.int_range 0 999_999)))
    (fun requests ->
       let stream_of rs =
         List.fold_right
           (fun r tail -> S.cons_stream r (fun () -> tail))
           rs
           S.the_empty_stream
       in
       let n = List.length requests in
       let answers s = List.init n (fun k -> S.stream_ref s k) in
       let first = answers (Sec_3_81.rand_stream (stream_of requests) 42L) in
       let second = answers (Sec_3_81.rand_stream (stream_of requests) 42L) in
       List.equal Int64.equal first second)
;;

let seeded_estimates_hold_their_error_bound =
  Test.make
    ~name:"a thousand seeded Monte Carlo trials land within 0.3 of pi"
    ~count:20
    (Gen.int_range 1000 3000)
    (fun trials ->
       Float.abs (S.stream_ref Sec_3_82.pi_estimates trials -. 3.141592653589793) < 0.3)
;;

let () =
  QCheck_base_runner.run_tests_main
    [ multi_map_agrees_with_the_pairwise_one
    ; memo_proc_runs_the_body_once
    ; partial_sums_is_the_running_total
    ; hamming_prefixes_are_ordered_and_smooth
    ; expand_shifts_by_whole_units
    ; memoized_fib_additions_are_linear_thunk_additions_exponential
    ; stream_limit_lands_within_the_tolerance
    ; all_pairs_carries_both_orders
    ; interleave_reaches_the_second_stream
    ; same_script_replays_the_same_answers
    ; seeded_estimates_hold_their_error_bound
    ]
;;
