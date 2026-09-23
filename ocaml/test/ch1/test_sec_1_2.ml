(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 1.2. [sicp_ch1_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module Sec_1_9 = Sicp_ch1_solutions.Sec_1_9
module Sec_1_10 = Sicp_ch1_solutions.Sec_1_10
module Sec_1_11 = Sicp_ch1_solutions.Sec_1_11
module Sec_1_12 = Sicp_ch1_solutions.Sec_1_12
module Sec_1_13 = Sicp_ch1_solutions.Sec_1_13
module Sec_1_14 = Sicp_ch1_solutions.Sec_1_14
module Sec_1_15 = Sicp_ch1_solutions.Sec_1_15
module Sec_1_16 = Sicp_ch1_solutions.Sec_1_16
module Sec_1_17 = Sicp_ch1_solutions.Sec_1_17
module Sec_1_18 = Sicp_ch1_solutions.Sec_1_18
module Sec_1_19 = Sicp_ch1_solutions.Sec_1_19
module Sec_1_20 = Sicp_ch1_solutions.Sec_1_20
module Sec_1_21 = Sicp_ch1_solutions.Sec_1_21
module Sec_1_22 = Sicp_ch1_solutions.Sec_1_22
module Sec_1_23 = Sicp_ch1_solutions.Sec_1_23
module Sec_1_24 = Sicp_ch1_solutions.Sec_1_24
module Sec_1_25 = Sicp_ch1_solutions.Sec_1_25
module Sec_1_26 = Sicp_ch1_solutions.Sec_1_26
module Sec_1_27 = Sicp_ch1_solutions.Sec_1_27
module Sec_1_28 = Sicp_ch1_solutions.Sec_1_28
module Fib_linear = Sicp_ch1.Sec_1_2.Fibonacci

let seeded_generator () =
  match Sicp_common.Random.create 42L with
  | Ok gen -> gen
  | Error Sicp_common.Error.Zero_seed -> failwith "the fixed seed 42 is nonzero"
;;

let ex_1_09_processes () =
  let deferred, tail = Sec_1_9.ex_1_09 () in
  Alcotest.(check int) "plus_deferred 4 5" 9 deferred;
  Alcotest.(check int) "plus_tail 4 5" 9 tail
;;

let ex_1_09_tail_is_constant_space () =
  Alcotest.(check int)
    "plus_tail runs to a million"
    1_000_000
    (Sec_1_9.plus_tail 1_000_000 0)
;;

let ex_1_10_ackermann_values () =
  let a, b, c = Sec_1_10.ex_1_10 () in
  Alcotest.(check int) "A(1, 10)" 1024 a;
  Alcotest.(check int) "A(2, 4)" 65536 b;
  Alcotest.(check int) "A(3, 3)" 65536 c
;;

let ex_1_10_concise_definitions () =
  Alcotest.(check (list int))
    "f n = 2n"
    [ 0; 2; 4; 6; 8; 10 ]
    (List.map Sec_1_10.f [ 0; 1; 2; 3; 4; 5 ]);
  Alcotest.(check (list int))
    "g n = 2^n for n >= 1"
    [ 2; 4; 8; 16; 32 ]
    (List.map Sec_1_10.g [ 1; 2; 3; 4; 5 ]);
  Alcotest.(check (list int))
    "h n, a tower of n twos"
    [ 2; 4; 16; 65536 ]
    (List.map Sec_1_10.h [ 1; 2; 3; 4 ])
;;

let ex_1_11_processes_agree () =
  let recursive, iterative = Sec_1_11.ex_1_11 10 in
  Alcotest.(check int) "f_recursive 10" 1892 recursive;
  Alcotest.(check int) "f_iterative 10" 1892 iterative;
  Alcotest.(check int) "f 5" 25 (Sec_1_11.f_recursive 5)
;;

let ex_1_12_pascal_row () =
  Alcotest.(check (list int)) "row 4" [ 1; 4; 6; 4; 1 ] (Sec_1_12.ex_1_12 4);
  Alcotest.(check (list int)) "row 0" [ 1 ] (Sec_1_12.ex_1_12 0);
  Alcotest.(check (list int)) "row 2" [ 1; 2; 1 ] (Sec_1_12.ex_1_12 2)
;;

let ex_1_13_closed_form_matches_direct () =
  Alcotest.(check int) "Fib(10)" 55 (Sec_1_13.ex_1_13 10);
  for n = 0 to 40 do
    Alcotest.(check int)
      (Printf.sprintf "closed form at %d" n)
      (Sec_1_13.fib_direct n)
      (Sec_1_13.closed_form_fib n)
  done
;;

let ex_1_14_node_count () =
  Alcotest.(check int) "11 cents has 55 nodes" 55 (Sec_1_14.ex_1_14 ())
;;

let ex_1_14a_measured_growth () =
  Alcotest.(check (list int))
    "amounts"
    [ 50; 100; 200; 400 ]
    (List.map fst (Sec_1_14.ex_1_14a ()))
;;

let ex_1_14a_call_counts () =
  Alcotest.(check (list int))
    "call counts"
    [ 1571; 15499; 229589; 4642025 ]
    (List.map snd (Sec_1_14.ex_1_14a ()))
;;

let ex_1_15_p_applications () =
  Alcotest.(check int) "p applied 5 times" 5 (Sec_1_15.ex_1_15 ())
;;

let ex_1_16_iterative_fast_expt () =
  Alcotest.(check int) "2 ** 10" 1024 (Sec_1_16.ex_1_16 2 10);
  Alcotest.(check int) "3 ** 13" 1594323 (Sec_1_16.ex_1_16 3 13);
  Alcotest.(check int) "n = 0" 1 (Sec_1_16.ex_1_16 5 0)
;;

let ex_1_17_logarithmic_multiply () =
  Alcotest.(check int) "3 times 7" 21 (Sec_1_17.ex_1_17 3 7);
  Alcotest.(check int) "zero factor" 0 (Sec_1_17.ex_1_17 0 5);
  Alcotest.(check int) "even factor" 48 (Sec_1_17.ex_1_17 6 8)
;;

let ex_1_18_iterative_multiply () =
  Alcotest.(check int) "3 times 7" 21 (Sec_1_18.ex_1_18 3 7);
  List.iter
    (fun (a, b) ->
       Alcotest.(check int)
         (Printf.sprintf "agrees with 1.17 at (%d, %d)" a b)
         (Sec_1_17.fast_mult a b)
         (Sec_1_18.ex_1_18 a b))
    [ 0, 5; 6, 8; 12, 13 ]
;;

let ex_1_19_logarithmic_fibonacci () =
  Alcotest.(check int) "Fib(10)" 55 (Sec_1_19.ex_1_19 10);
  Alcotest.(check int) "Fib(20)" 6765 (Sec_1_19.ex_1_19 20);
  for n = 0 to 90 do
    Alcotest.(check int)
      (Printf.sprintf "matches linear fib at %d" n)
      (Fib_linear.iterative n)
      (Sec_1_19.ex_1_19 n)
  done;
  Alcotest.(check int)
    "wraps at 91, visibly negative"
    (-4563325426479245499)
    (Sec_1_19.ex_1_19 91)
;;

let ex_1_20_eager_evaluation_count () =
  let value, count = Sec_1_20.ex_1_20 () in
  Alcotest.(check int) "gcd 206 40" 2 value;
  Alcotest.(check int) "remainder calls, eager" 4 count
;;

let ex_1_21_smallest_divisors () =
  let a, b, c = Sec_1_21.ex_1_21 () in
  Alcotest.(check int) "199 is prime" 199 a;
  Alcotest.(check int) "1999 is prime" 1999 b;
  Alcotest.(check int) "19999 = 7 * 2857" 7 c
;;

let ex_1_22_searches () =
  let expected = [ 1000; 10000; 100000; 1000000 ] in
  let found = Sec_1_22.ex_1_22 () in
  Alcotest.(check (list int)) "thresholds" expected (List.map fst found);
  Alcotest.(check (list (list int)))
    "three smallest primes above each"
    [ [ 1009; 1013; 1019 ]
    ; [ 10007; 10009; 10037 ]
    ; [ 100003; 100019; 100043 ]
    ; [ 1000003; 1000033; 1000037 ]
    ]
    (List.map snd found)
;;

let ex_1_22_timed_prime_test_shape () =
  Alcotest.(check bool)
    "composite reports no result"
    true
    (Sec_1_22.timed_prime_test 100 = None);
  match Sec_1_22.timed_prime_test 101 with
  | None -> Alcotest.fail "101 is prime and should report a result"
  | Some elapsed ->
    Alcotest.(check bool) "elapsed time is nonnegative" true (elapsed >= 0.0)
;;

let ex_1_23_matches_naive_search () =
  Alcotest.(check int) "19999's smallest divisor" 7 (Sec_1_23.ex_1_23 19999);
  List.iter
    (fun n ->
       Alcotest.(check int)
         (Printf.sprintf "agrees with the naive search at %d" n)
         (Sec_1_23.smallest_divisor_naive n)
         (Sec_1_23.ex_1_23 n))
    [ 199; 1999; 19999; 1000003; 100; 561 ]
;;

let ex_1_24_fermat_accepts_every_found_prime () =
  Alcotest.(check (list bool))
    "all twelve pass fast_prime"
    (List.map (fun _ -> true) Sec_1_24.twelve_primes)
    (Sec_1_24.ex_1_24 ())
;;

let ex_1_25_alyssa_disagrees () =
  let correct, wrapped = Sec_1_25.ex_1_25 () in
  Alcotest.(check int) "the correct remainder" 3 correct;
  Alcotest.(check int) "Alyssa's wrapped shortcut" 0 wrapped
;;

let ex_1_26_louis_calls_far_more_often () =
  let square_calls, double_calls = Sec_1_26.ex_1_26 () in
  Alcotest.(check int) "section's expmod, one call per level" 8 square_calls;
  Alcotest.(check int) "Louis's expmod, exponentially more" 191 double_calls
;;

let ex_1_27_carmichael_numbers_fool_fermat () =
  let results = Sec_1_27.ex_1_27 () in
  Alcotest.(check (list int))
    "the six Carmichael numbers"
    Sec_1_27.carmichael_numbers
    (List.map fst results);
  Alcotest.(check (list bool))
    "every one fools the Fermat test"
    (List.map (fun _ -> true) Sec_1_27.carmichael_numbers)
    (List.map snd results);
  Alcotest.(check bool)
    "an ordinary composite does not fool it"
    false
    (Sec_1_27.fools_fermat 100)
;;

let ex_1_28_miller_rabin_is_not_fooled () =
  let results = Sec_1_28.ex_1_28 () in
  Alcotest.(check (list bool))
    "Miller-Rabin rejects every Carmichael number"
    (List.map (fun _ -> false) Sec_1_28.carmichael_numbers)
    (List.map snd results);
  let gen = seeded_generator () in
  Alcotest.(check bool)
    "accepts an ordinary prime"
    true
    (Sec_1_28.miller_rabin_prime 97 30 gen);
  Alcotest.(check bool)
    "rejects an ordinary composite"
    false
    (Sec_1_28.miller_rabin_prime 100 30 gen)
;;

let () =
  Alcotest.run
    "sicp_ch1 solutions, section 1.2"
    [ ( "1.9 substitution model for two additions"
      , [ Alcotest.test_case "both processes add 4 and 5" `Quick ex_1_09_processes
        ; Alcotest.test_case
            "the tail-shaped one runs to a million"
            `Quick
            ex_1_09_tail_is_constant_space
        ] )
    ; ( "1.10 Ackermann's function"
      , [ Alcotest.test_case "named applications" `Quick ex_1_10_ackermann_values
        ; Alcotest.test_case "f, g, h in closed form" `Quick ex_1_10_concise_definitions
        ] )
    ; ( "1.11 recursive and iterative f"
      , [ Alcotest.test_case "both processes compute f(10)" `Quick ex_1_11_processes_agree
        ] )
    ; ( "1.12 Pascal's triangle"
      , [ Alcotest.test_case "row 4 and the edges" `Quick ex_1_12_pascal_row ] )
    ; ( "1.13 closed-form Fibonacci"
      , [ Alcotest.test_case
            "matches the direct definition"
            `Quick
            ex_1_13_closed_form_matches_direct
        ] )
    ; ( "1.14 count-change tree"
      , [ Alcotest.test_case "11 cents has 55 nodes" `Quick ex_1_14_node_count ] )
    ; ( "1.14a measured growth"
      , [ Alcotest.test_case "amounts measured" `Quick ex_1_14a_measured_growth
        ; Alcotest.test_case "call counts climb toward a^5" `Quick ex_1_14a_call_counts
        ] )
    ; ( "1.15 sine reduction steps"
      , [ Alcotest.test_case "p applied 5 times" `Quick ex_1_15_p_applications ] )
    ; ( "1.16 iterative fast exponentiation"
      , [ Alcotest.test_case "invariant-based results" `Quick ex_1_16_iterative_fast_expt
        ] )
    ; ( "1.17 logarithmic multiplication"
      , [ Alcotest.test_case "doubling and halving" `Quick ex_1_17_logarithmic_multiply ]
      )
    ; ( "1.18 iterative multiplication"
      , [ Alcotest.test_case "agrees with 1.17" `Quick ex_1_18_iterative_multiply ] )
    ; ( "1.19 logarithmic Fibonacci"
      , [ Alcotest.test_case
            "correct through 90, wraps at 91"
            `Quick
            ex_1_19_logarithmic_fibonacci
        ] )
    ; ( "1.20 eager evaluation count"
      , [ Alcotest.test_case
            "gcd 206 40 makes 4 remainder calls"
            `Quick
            ex_1_20_eager_evaluation_count
        ] )
    ; ( "1.21 smallest divisors"
      , [ Alcotest.test_case "199, 1999, 19999" `Quick ex_1_21_smallest_divisors ] )
    ; ( "1.22 timed prime search"
      , [ Alcotest.test_case
            "three smallest primes above each threshold"
            `Quick
            ex_1_22_searches
        ; Alcotest.test_case
            "timed_prime_test's shape"
            `Quick
            ex_1_22_timed_prime_test_shape
        ] )
    ; ( "1.23 skip even test divisors"
      , [ Alcotest.test_case
            "matches the naive search"
            `Quick
            ex_1_23_matches_naive_search
        ] )
    ; ( "1.24 timed Fermat test"
      , [ Alcotest.test_case
            "accepts every prime exercise 1.22 found"
            `Quick
            ex_1_24_fermat_accepts_every_found_prime
        ] )
    ; ( "1.25 naive expmod overflows"
      , [ Alcotest.test_case
            "disagrees with the correct remainder"
            `Quick
            ex_1_25_alyssa_disagrees
        ] )
    ; ( "1.26 doubled expmod calls"
      , [ Alcotest.test_case
            "Louis's version calls far more often"
            `Quick
            ex_1_26_louis_calls_far_more_often
        ] )
    ; ( "1.27 Carmichael numbers fool Fermat"
      , [ Alcotest.test_case
            "every witness passes"
            `Quick
            ex_1_27_carmichael_numbers_fool_fermat
        ] )
    ; ( "1.28 Miller-Rabin"
      , [ Alcotest.test_case
            "not fooled by Carmichael numbers"
            `Quick
            ex_1_28_miller_rabin_is_not_fooled
        ] )
    ]
;;
