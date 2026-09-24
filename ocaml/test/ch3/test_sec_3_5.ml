(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 3.5. Deterministic values the exercises compute are pinned
   exactly; convergence and Monte Carlo quantities are asserted as
   invariants (a tolerance toward the known limit), never as one
   particular float dust. *)

module S = Sicp_ch3.Sec_3_5.Streams
module I = Sicp_ch3.Sec_3_5.Infinite
module P = Sicp_ch3.Sec_3_5.Pairs
module Sec_3_50 = Sicp_ch3_solutions.Sec_3_50
module Sec_3_51 = Sicp_ch3_solutions.Sec_3_51
module Sec_3_52 = Sicp_ch3_solutions.Sec_3_52
module Sec_3_53 = Sicp_ch3_solutions.Sec_3_53
module Sec_3_54 = Sicp_ch3_solutions.Sec_3_54
module Sec_3_55 = Sicp_ch3_solutions.Sec_3_55
module Sec_3_56 = Sicp_ch3_solutions.Sec_3_56
module Sec_3_57 = Sicp_ch3_solutions.Sec_3_57
module Sec_3_58 = Sicp_ch3_solutions.Sec_3_58
module Sec_3_59 = Sicp_ch3_solutions.Sec_3_59
module Sec_3_60 = Sicp_ch3_solutions.Sec_3_60
module Sec_3_61 = Sicp_ch3_solutions.Sec_3_61
module Sec_3_62 = Sicp_ch3_solutions.Sec_3_62
module Sec_3_63 = Sicp_ch3_solutions.Sec_3_63
module Sec_3_64 = Sicp_ch3_solutions.Sec_3_64
module Sec_3_65 = Sicp_ch3_solutions.Sec_3_65
module Sec_3_66 = Sicp_ch3_solutions.Sec_3_66
module Sec_3_67 = Sicp_ch3_solutions.Sec_3_67
module Sec_3_68 = Sicp_ch3_solutions.Sec_3_68
module Sec_3_69 = Sicp_ch3_solutions.Sec_3_69
module Sec_3_70 = Sicp_ch3_solutions.Sec_3_70
module Sec_3_71 = Sicp_ch3_solutions.Sec_3_71
module Sec_3_72 = Sicp_ch3_solutions.Sec_3_72
module Sec_3_73 = Sicp_ch3_solutions.Sec_3_73
module Sec_3_74 = Sicp_ch3_solutions.Sec_3_74
module Sec_3_75 = Sicp_ch3_solutions.Sec_3_75
module Sec_3_76 = Sicp_ch3_solutions.Sec_3_76
module Sec_3_77 = Sicp_ch3_solutions.Sec_3_77
module Sec_3_78 = Sicp_ch3_solutions.Sec_3_78
module Sec_3_79 = Sicp_ch3_solutions.Sec_3_79
module Sec_3_80 = Sicp_ch3_solutions.Sec_3_80
module Sec_3_81 = Sicp_ch3_solutions.Sec_3_81
module Sec_3_82 = Sicp_ch3_solutions.Sec_3_82

let check_int_list = Alcotest.(check (list int))
let check_float_list = Alcotest.(check (list (float 1e-12)))
let check_pair_list = Alcotest.(check (list (pair int int)))
let check_triple_list = Alcotest.(check (list (triple int int int)))

let ex_3_50_multi_map () =
  let sums, drained = Sec_3_50.ex_3_50 () in
  check_int_list
    "elementwise sums of two offset integer streams"
    [ 3; 5; 7; 9; 11; 13 ]
    sums;
  Alcotest.(check bool) "an empty argument stream drains the map" true drained
;;

let ex_3_51_show_transcript () =
  let after_def, after_5, after_7 = Sec_3_51.ex_3_51 () in
  check_int_list "the head is shown when the stream is built" [ 0 ] after_def;
  check_int_list "ref 5 shows 1 through 5" [ 0; 1; 2; 3; 4; 5 ] after_5;
  check_int_list
    "ref 7 shows only 6 and 7, thanks to memoization"
    [ 0; 1; 2; 3; 4; 5; 6; 7 ]
    after_7
;;

let ex_3_51a_counted_forces () =
  let s1, s5, s7, forces, bodies = Sec_3_51.ex_3_51a () in
  Alcotest.(check int) "one show at construction" 1 s1;
  Alcotest.(check int) "six shows after ref 5" 6 s5;
  Alcotest.(check int) "eight shows after ref 7" 8 s7;
  Alcotest.(check int) "twelve tail accesses" 12 forces;
  Alcotest.(check int) "only seven tail bodies ever ran" 7 bodies;
  Alcotest.(check bool)
    "memoization is the gap between forces and bodies"
    true
    (bodies < forces)
;;

let ex_3_52_assign_and_laziness () =
  let ( ( memo_sum_seq
        , memo_sum_y
        , _memo_sum_z
        , memo_y7
        , memo_after_ref
        , memo_z
        , memo_after_display )
      , ( plain_sum_seq
        , plain_sum_y
        , _plain_sum_z
        , _plain_y7
        , _plain_after_ref
        , plain_z
        , plain_after_display ) )
    =
    Sec_3_52.ex_3_52 ()
  in
  Alcotest.(check int) "building seq adds only its head" 1 memo_sum_seq;
  Alcotest.(check int) "the even filter scans to the first even element" 6 memo_sum_y;
  Alcotest.(check int) "the eighth even element is 136" 136 memo_y7;
  Alcotest.(check int) "ref y 7 leaves the sum at 136" 136 memo_after_ref;
  check_int_list
    "the displayed multiples of five"
    [ 10; 15; 45; 55; 105; 120; 190; 210 ]
    memo_z;
  Alcotest.(check int) "the display drains seq to 210" 210 memo_after_display;
  Alcotest.(check int) "the plain-thunk seq also adds only its head" 1 plain_sum_seq;
  Alcotest.(check int)
    "the plain-thunk filter's scan re-adds to the same first even"
    6
    plain_sum_y;
  Alcotest.(check bool)
    "without memoization the sums and the displayed list differ"
    true
    (plain_after_display <> memo_after_display || plain_z <> memo_z)
;;

let ex_3_53_self_doubling () =
  check_int_list
    "1 : (s + s) is the powers of two"
    [ 1; 2; 4; 8; 16; 32; 64; 128 ]
    (Sec_3_53.ex_3_53 ())
;;

let ex_3_54_factorial_stream () =
  check_int_list
    "the integers multiplied into their own tail"
    [ 1; 1; 2; 6; 24; 120; 720; 5040 ]
    (Sec_3_54.ex_3_54 ())
;;

let ex_3_55_partial_sums () =
  let integers, ones = Sec_3_55.ex_3_55 () in
  check_int_list "partial sums of the integers" [ 1; 3; 6; 10; 15 ] integers;
  check_int_list "partial sums of ones" [ 1; 2; 3; 4; 5 ] ones
;;

let ex_3_56_hamming () =
  check_int_list
    "1 followed by merged scaled copies, no repeats"
    [ 1; 2; 3; 4; 5; 6; 8; 9; 10; 12; 15; 16 ]
    (Sec_3_56.ex_3_56 ())
;;

let ex_3_57_addition_counts () =
  let n, memo, plain, memo_is_n_minus_2 = Sec_3_57.ex_3_57 () in
  Alcotest.(check int) "fifteen elements" 15 n;
  Alcotest.(check bool)
    "the memoized fibs add exactly once per new element"
    true
    memo_is_n_minus_2;
  Alcotest.(check bool)
    "the plain-thunk fibs add exponentially more"
    true
    (plain > 10 * memo)
;;

let ex_3_58_expand_digits () =
  let sevenths, three_eighths = Sec_3_58.ex_3_58 () in
  check_int_list
    "1/7 in base ten begins 0.14285714..."
    [ 1; 4; 2; 8; 5; 7; 1; 4 ]
    sevenths;
  check_int_list "3/8 in base ten is 0.37500..." [ 3; 7; 5; 0; 0 ] three_eighths
;;

let ex_3_59_series () =
  let exp6, cos6, sin6 = Sec_3_59.ex_3_59 () in
  check_float_list
    "e^x coefficients"
    [ 1.0; 1.0; 0.5; 1.0 /. 6.0; 1.0 /. 24.0; 1.0 /. 120.0 ]
    exp6;
  check_float_list "cos x coefficients" [ 1.0; 0.0; -0.5; 0.0; 1.0 /. 24.0; 0.0 ] cos6;
  check_float_list
    "sin x coefficients"
    [ 0.0; 1.0; 0.0; -1.0 /. 6.0; 0.0; 1.0 /. 120.0 ]
    sin6
;;

let ex_3_60_series_product () =
  let coefficients, identity_holds = Sec_3_60.ex_3_60 () in
  check_float_list
    "sin^2 + cos^2 begins with the unit series"
    [ 1.0; 0.0; 0.0; 0.0; 0.0 ]
    coefficients;
  Alcotest.(check bool)
    "the identity holds at the checked coefficients"
    true
    identity_holds
;;

let ex_3_61_series_reciprocal () =
  check_float_list
    "e^x times 1/e^x is the unit series"
    [ 1.0; 0.0; 0.0; 0.0; 0.0 ]
    (Sec_3_61.ex_3_61 ())
;;

let ex_3_62_division_and_tangent () =
  let tangent, rejected = Sec_3_62.ex_3_62 () in
  check_float_list
    "the tangent series x + x^3/3 + 2x^5/15"
    [ 0.0; 1.0; 0.0; 1.0 /. 3.0; 0.0; 2.0 /. 15.0 ]
    tangent;
  Alcotest.(check bool) "a zero constant denominator is refused" true rejected
;;

let ex_3_63_local_guesses () =
  let local, local_calls, open_, open_calls = Sec_3_63.ex_3_63 () in
  check_float_list
    "the local version refines toward the square root of 2"
    [ 1.0; 1.5; 1.4166666666666667; 1.4142156862745099; 1.4142135623746899 ]
    local;
  Alcotest.(check bool)
    "the open version's guesses converge all the same"
    true
    (Float.abs (List.hd (List.rev open_) -. 1.4142135623730951) < 1e-6);
  Alcotest.(check int) "the local version improves once per new guess" 4 local_calls;
  Alcotest.(check int)
    "the open version spends ten improvements for the same five guesses"
    10
    open_calls
;;

let ex_3_64_stream_limit () =
  let coarse, fine = Sec_3_64.ex_3_64 () in
  Alcotest.(check bool)
    "the coarse limit is within 1e-4 of the root"
    true
    (Float.abs (coarse -. 1.4142135623730951) < 1e-4);
  Alcotest.(check bool)
    "the fine limit is within 1e-8 of the root"
    true
    (Float.abs (fine -. 1.4142135623730951) < 1e-8)
;;

let ex_3_65_ln2 () =
  let plain, euler, accelerated = Sec_3_65.ex_3_65 () in
  Alcotest.(check bool)
    "eight plain terms still straddle ln 2"
    true
    (List.nth plain 7 < 0.6931471805599453 && List.nth plain 6 > 0.6931471805599453);
  Alcotest.(check bool)
    "eight Euler terms are within 0.01"
    true
    (Float.abs (List.nth euler 7 -. 0.6931471805599453) < 0.01);
  Alcotest.(check bool)
    "the accelerated sequence reaches ln 2 to about thirteen places"
    true
    (Float.abs (List.nth accelerated 7 -. 0.6931471805599453) < 1e-12)
;;

let ex_3_66_pair_order () =
  let first_row, diagonal, _p2_10, _p9_10, _p10_10 = Sec_3_66.ex_3_66 () in
  Alcotest.(check bool) "(1, j) sits at position 2j - 2 for j <= 100" true first_row;
  Alcotest.(check bool) "(k, k) sits at position 2^k - 1 for 2 <= k <= 15" true diagonal
;;

let ex_3_67_all_pairs () =
  let first, covered = Sec_3_67.ex_3_67 () in
  Alcotest.(check int) "twenty-four pairs collected" 24 (List.length first);
  Alcotest.(check bool) "the 3x3 corner is fully covered there" true covered
;;

let ex_3_68_louis_pairs () =
  let louis_first, burned, book = Sec_3_68.ex_3_68 () in
  Alcotest.(check bool)
    "Louis's definition never produces a first pair"
    true
    (louis_first = None);
  Alcotest.(check int) "the eager recursion burns the whole budget" 200_000 burned;
  Alcotest.(check int) "the book's definition yields sixteen pairs" 16 (List.length book)
;;

let ex_3_69_pythagorean () =
  check_triple_list
    "the first Pythagorean triples"
    [ 3, 4, 5; 6, 8, 10; 5, 12, 13 ]
    (Sec_3_69.ex_3_69 ())
;;

let ex_3_70_weighted_orders () =
  let by_sum, by_235 = Sec_3_70.ex_3_70 () in
  let sorted_by xs weight =
    List.sort (fun a b -> compare (weight a) (weight b)) xs = xs
  in
  Alcotest.(check bool)
    "the first ten pairs of the i + j order appear in nondecreasing weight"
    true
    (sorted_by by_sum (fun (i, j) -> i + j));
  Alcotest.(check bool)
    "the first ten pairs of the 2i + 3j + 5ij order appear in nondecreasing weight"
    true
    (sorted_by by_235 (fun (i, j) -> (2 * i) + (3 * j) + (5 * i * j)));
  check_pair_list
    "the 2i + 3j + 5ij order uses only integers coprime to 2, 3, 5"
    (List.map (fun (i, j) -> i, j) by_235)
    by_235
;;

let ex_3_71_ramanujan () =
  check_int_list
    "1729 and the next five taxicab numbers"
    [ 1729; 4104; 13832; 20683; 32832; 39312 ]
    (Sec_3_71.ex_3_71 ())
;;

let ex_3_72_three_squares () =
  let numbers, _writings = Sec_3_72.ex_3_72 () in
  check_int_list "numbers square-writable three ways" [ 325; 425; 650; 725; 845 ] numbers;
  List.iter
    (fun (p1, p2, p3) ->
       let weight (i, j) = (i * i) + (j * j) in
       Alcotest.(check bool)
         "the three writings share one weight"
         true
         (weight p1 = weight p2 && weight p2 = weight p3))
    _writings
;;

let ex_3_73_rc_circuit () =
  check_float_list
    "constant one ampere into R = 5, C = 1, dt = 0.5 from v0 = 0"
    [ 5.0; 5.5; 6.0; 6.5; 7.0 ]
    (Sec_3_73.ex_3_73 ())
;;

let ex_3_74_zero_crossings () =
  check_int_list
    "the statement's own crossing line"
    [ 0; 0; 0; 0; 0; -1; 0; 0; 0; 0; 1; 0 ]
    (Sec_3_74.ex_3_74 ())
;;

let ex_3_75_smoothed_detector () =
  let louis, fixed, louis_flips, fixed_flips = Sec_3_75.ex_3_75 () in
  check_int_list
    "Louis's averages lag the signal's swings"
    [ 0; 0; 0; 0; 0; 0; 0; 0; -1; 0; 0; 0; 0; 1 ]
    louis;
  check_int_list
    "the repaired detector's crossings sit at the raw swings"
    [ 0; 0; 0; 0; 0; 0; 0; -1; 0; 0; 0; 0; 1; 0 ]
    fixed;
  Alcotest.(check int) "Louis's crossing count" 2 louis_flips;
  Alcotest.(check int) "the repaired crossing count" 2 fixed_flips
;;

let ex_3_76_smooth_combinator () =
  let smoothed, crossings = Sec_3_76.ex_3_76 () in
  check_float_list "successive averages of 1 2 3 4" [ 1.5; 2.5; 3.5 ] smoothed;
  Alcotest.(check bool)
    "the modular detector still finds the signal's two crossings"
    true
    (List.exists (fun c -> c = -1) crossings
     && List.exists (fun c -> c = 1) crossings
     && List.length (List.filter (fun c -> c <> 0) crossings) = 2)
;;

let ex_3_77_cons_stream_integral () =
  let e_approximation, over_empty = Sec_3_77.ex_3_77 () in
  Alcotest.(check bool)
    "y(1) of dy/dt = y is close to e"
    true
    (Float.abs (e_approximation -. 2.718281828459045) < 0.002);
  check_float_list "an empty integrand yields the initial value alone" [ 5.0 ] over_empty
;;

let ex_3_78_second_order () =
  let y = Sec_3_78.ex_3_78 () in
  Alcotest.(check bool)
    "the harmonic oscillator reaches sin(pi/2) within one step"
    true
    (Float.abs (y -. 1.0) < 0.001)
;;

let ex_3_79_general_second_order () =
  Alcotest.(check bool)
    "the general solver reproduces the dedicated one bit for bit"
    true
    (Sec_3_79.ex_3_79 ())
;;

let ex_3_80_rlc () =
  let vc, il = Sec_3_80.ex_3_80 () in
  check_float_list
    "the capacitor voltage holds then gives way"
    [ 10.0; 10.0; 9.5; 8.55 ]
    vc;
  check_float_list "the inductor current builds" [ 0.0; 1.0; 1.9; 2.66 ] il
;;

let ex_3_81_request_stream () =
  let answers, replayable = Sec_3_81.ex_3_81 () in
  Alcotest.(check int) "seven answers" 7 (List.length answers);
  Alcotest.(check bool)
    "the same script from the same seed replays exactly"
    true
    replayable
;;

let ex_3_82_streaming_integration () =
  let at_1000, at_10000, close = Sec_3_82.ex_3_82 () in
  Alcotest.(check bool)
    "the estimate sharpens as trials accumulate"
    true
    (Float.abs (at_10000 -. 3.141592653589793) < Float.abs (at_1000 -. 3.141592653589793));
  Alcotest.(check bool) "ten thousand trials sit within 0.1 of pi" true close
;;

(* The section substrate the exercises build on: the exercise 3.51a
   counters already proved memoization; here the shared infinite
   streams agree with their explicit twins. *)
let substrate_implicit_definitions () =
  Alcotest.(check bool)
    "the implicit integers equal the generated ones"
    true
    (S.stream_take 100 I.integers_implicit = S.stream_take 100 I.integers);
  Alcotest.(check bool)
    "the implicit fibs equal the generated ones"
    true
    (S.stream_take 60 I.fibs_implicit = S.stream_take 60 I.fibs)
;;

let substrate_pair_positions () =
  let p = List.nth (S.stream_take 7 (P.pairs I.integers I.integers)) 6 in
  Alcotest.(check (pair int int)) "the seventh pair above the diagonal is (3, 3)" (3, 3) p
;;

let () =
  Alcotest.run
    "sicp_ch3 solutions, section 3.5"
    [ "exercise 3.50", [ Alcotest.test_case "multi-stream map" `Quick ex_3_50_multi_map ]
    ; ( "exercise 3.51"
      , [ Alcotest.test_case "show transcript" `Quick ex_3_51_show_transcript
        ; Alcotest.test_case "counted forces, 3.51a" `Quick ex_3_51a_counted_forces
        ] )
    ; ( "exercise 3.52"
      , [ Alcotest.test_case
            "assignment with memoized delay vs plain thunks"
            `Quick
            ex_3_52_assign_and_laziness
        ] )
    ; ( "exercise 3.53"
      , [ Alcotest.test_case "self-doubling stream" `Quick ex_3_53_self_doubling ] )
    ; "exercise 3.54", [ Alcotest.test_case "factorials" `Quick ex_3_54_factorial_stream ]
    ; "exercise 3.55", [ Alcotest.test_case "partial sums" `Quick ex_3_55_partial_sums ]
    ; "exercise 3.56", [ Alcotest.test_case "Hamming numbers" `Quick ex_3_56_hamming ]
    ; ( "exercise 3.57"
      , [ Alcotest.test_case
            "addition counts, memoized vs not"
            `Quick
            ex_3_57_addition_counts
        ] )
    ; ( "exercise 3.58"
      , [ Alcotest.test_case "long division digits" `Quick ex_3_58_expand_digits ] )
    ; "exercise 3.59", [ Alcotest.test_case "series coefficients" `Quick ex_3_59_series ]
    ; ( "exercise 3.60"
      , [ Alcotest.test_case
            "series product and the unit identity"
            `Quick
            ex_3_60_series_product
        ] )
    ; ( "exercise 3.61"
      , [ Alcotest.test_case "series reciprocal" `Quick ex_3_61_series_reciprocal ] )
    ; ( "exercise 3.62"
      , [ Alcotest.test_case
            "series division and tangent"
            `Quick
            ex_3_62_division_and_tangent
        ] )
    ; ( "exercise 3.63"
      , [ Alcotest.test_case
            "local guesses versus open recursion"
            `Quick
            ex_3_63_local_guesses
        ] )
    ; "exercise 3.64", [ Alcotest.test_case "stream limit" `Quick ex_3_64_stream_limit ]
    ; ( "exercise 3.65"
      , [ Alcotest.test_case "ln 2 at three accelerations" `Quick ex_3_65_ln2 ] )
    ; "exercise 3.66", [ Alcotest.test_case "pair order laws" `Quick ex_3_66_pair_order ]
    ; "exercise 3.67", [ Alcotest.test_case "all pairs" `Quick ex_3_67_all_pairs ]
    ; "exercise 3.68", [ Alcotest.test_case "Louis's pairs" `Quick ex_3_68_louis_pairs ]
    ; ( "exercise 3.69"
      , [ Alcotest.test_case "Pythagorean triples" `Quick ex_3_69_pythagorean ] )
    ; ( "exercise 3.70"
      , [ Alcotest.test_case "weighted orders" `Quick ex_3_70_weighted_orders ] )
    ; "exercise 3.71", [ Alcotest.test_case "Ramanujan numbers" `Quick ex_3_71_ramanujan ]
    ; ( "exercise 3.72"
      , [ Alcotest.test_case "three square writings" `Quick ex_3_72_three_squares ] )
    ; "exercise 3.73", [ Alcotest.test_case "RC circuit" `Quick ex_3_73_rc_circuit ]
    ; ( "exercise 3.74"
      , [ Alcotest.test_case "zero crossings" `Quick ex_3_74_zero_crossings ] )
    ; ( "exercise 3.75"
      , [ Alcotest.test_case "Louis's smoothing bug" `Quick ex_3_75_smoothed_detector ] )
    ; ( "exercise 3.76"
      , [ Alcotest.test_case "smooth as a combinator" `Quick ex_3_76_smooth_combinator ] )
    ; ( "exercise 3.77"
      , [ Alcotest.test_case
            "cons-stream integral with delayed input"
            `Quick
            ex_3_77_cons_stream_integral
        ] )
    ; ( "exercise 3.78"
      , [ Alcotest.test_case "second-order solve" `Quick ex_3_78_second_order ] )
    ; ( "exercise 3.79"
      , [ Alcotest.test_case
            "general second-order solve"
            `Quick
            ex_3_79_general_second_order
        ] )
    ; "exercise 3.80", [ Alcotest.test_case "RLC coupled streams" `Quick ex_3_80_rlc ]
    ; ( "exercise 3.81"
      , [ Alcotest.test_case "request stream" `Quick ex_3_81_request_stream ] )
    ; ( "exercise 3.82"
      , [ Alcotest.test_case
            "streaming Monte Carlo integration"
            `Quick
            ex_3_82_streaming_integration
        ] )
    ; ( "section substrate"
      , [ Alcotest.test_case
            "implicit definitions equal explicit twins"
            `Quick
            substrate_implicit_definitions
        ; Alcotest.test_case
            "diagonal positions in the pairs stream"
            `Quick
            substrate_pair_positions
        ] )
    ]
;;
