(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 1.3. [sicp_ch1_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module Sec_1_29 = Sicp_ch1_solutions.Sec_1_29
module Sec_1_30 = Sicp_ch1_solutions.Sec_1_30
module Sec_1_31 = Sicp_ch1_solutions.Sec_1_31
module Sec_1_32 = Sicp_ch1_solutions.Sec_1_32
module Sec_1_33 = Sicp_ch1_solutions.Sec_1_33
module Sec_1_34 = Sicp_ch1_solutions.Sec_1_34
module Sec_1_35 = Sicp_ch1_solutions.Sec_1_35
module Sec_1_36 = Sicp_ch1_solutions.Sec_1_36
module Sec_1_37 = Sicp_ch1_solutions.Sec_1_37
module Sec_1_38 = Sicp_ch1_solutions.Sec_1_38
module Sec_1_39 = Sicp_ch1_solutions.Sec_1_39
module Sec_1_40 = Sicp_ch1_solutions.Sec_1_40
module Sec_1_41 = Sicp_ch1_solutions.Sec_1_41
module Sec_1_42 = Sicp_ch1_solutions.Sec_1_42
module Sec_1_43 = Sicp_ch1_solutions.Sec_1_43
module Sec_1_44 = Sicp_ch1_solutions.Sec_1_44
module Sec_1_45 = Sicp_ch1_solutions.Sec_1_45
module Sec_1_46 = Sicp_ch1_solutions.Sec_1_46
module Average_damping = Sicp_ch1.Sec_1_3.Average_damping

let float_close =
  Alcotest.testable
    (fun ppf v -> Format.fprintf ppf "%.17g" v)
    (fun a b -> Float.abs (a -. b) < 1e-9)
;;

let ex_1_29_simpson_beats_integral () =
  let n100, n1000 = Sec_1_29.ex_1_29 () in
  Alcotest.(check float_close) "simpson n=100" 0.25000000000000006 n100;
  Alcotest.(check float_close) "simpson n=1000" 0.25000000000000006 n1000
;;

let ex_1_30_iterative_sum () =
  Alcotest.(check float_close) "sum 1..10" 55.0 (Sec_1_30.ex_1_30 ())
;;

let ex_1_31_product_factorial_pi () =
  let factorial_6, pi_approx = Sec_1_31.ex_1_31 () in
  Alcotest.(check float_close) "factorial 6" 720.0 factorial_6;
  Alcotest.(check float_close) "pi approx n=1000" 3.143160705532257 pi_approx
;;

let ex_1_32_accumulate () =
  let sum10, product6 = Sec_1_32.ex_1_32 () in
  Alcotest.(check float_close) "sum via accumulate" 55.0 sum10;
  Alcotest.(check float_close) "product via accumulate" 720.0 product6
;;

let ex_1_33_filtered_accumulate () =
  let sq_primes, rel_prime_product = Sec_1_33.ex_1_33 () in
  Alcotest.(check int) "sum of squares of primes 2..20" 1027 sq_primes;
  Alcotest.(check int) "product relatively prime to 10" 189 rel_prime_product
;;

let ex_1_34_f_applied_to_procedures () =
  let f_square, f_lambda = Sec_1_34.ex_1_34 () in
  Alcotest.(check int) "f square" 4 f_square;
  Alcotest.(check int) "f (fun z -> z * (z + 1))" 6 f_lambda
;;

let ex_1_35_golden_ratio () =
  Alcotest.(check float_close) "golden ratio" 1.6180327868852458 (Sec_1_35.ex_1_35 ())
;;

let ex_1_36_traced_fixed_point () =
  let (undamped_v, undamped_steps), (damped_v, damped_steps) = Sec_1_36.ex_1_36 () in
  Alcotest.(check float_close) "undamped value" 4.555532270803653 undamped_v;
  Alcotest.(check int) "undamped steps" 34 undamped_steps;
  Alcotest.(check float_close) "damped value" 4.555537551999825 damped_v;
  Alcotest.(check int) "damped steps" 9 damped_steps;
  Alcotest.(check bool) "damping needs fewer steps" true (damped_steps < undamped_steps)
;;

let ex_1_37_cont_frac_golden_ratio () =
  let value, k = Sec_1_37.ex_1_37 () in
  Alcotest.(check int) "smallest robust k" 11 k;
  Alcotest.(check float_close) "cont_frac 11" 0.6180555555555556 value
;;

let ex_1_38_euler_e () =
  Alcotest.(check float_close) "e approximation" (Float.exp 1.0) (Sec_1_38.ex_1_38 ())
;;

let ex_1_39_tan_cf () =
  let small_x, near_one = Sec_1_39.ex_1_39 () in
  Alcotest.(check float_close) "tan_cf 0.1 10" (Float.tan 0.1) small_x;
  Alcotest.(check float_close) "tan_cf 1.0 20" (Float.tan 1.0) near_one
;;

let ex_1_40_cubic_newtons_method () =
  Alcotest.(check float_close) "root of (x-1)(x-2)(x-3) near 1" 1.0 (Sec_1_40.ex_1_40 ())
;;

let ex_1_41_double_combinator () =
  Alcotest.(check int) "double puzzle" 21 (Sec_1_41.ex_1_41 ())
;;

let ex_1_42_compose () =
  Alcotest.(check int) "compose square inc 6" 49 (Sec_1_42.ex_1_42 ())
;;

let ex_1_43_repeated () =
  Alcotest.(check int) "repeated square 2 applied to 5" 625 (Sec_1_43.ex_1_43 ())
;;

let ex_1_44_smoothing () =
  let noisy_2, smoothed_2 = Sec_1_44.ex_1_44 () in
  Alcotest.(check float_close) "noisy(2)" 4.0093003950441615 noisy_2;
  Alcotest.(check float_close) "5-fold smoothed noisy(2)" 4.009298845427903 smoothed_2;
  Alcotest.(check bool)
    "smoothing pulls closer to the true value 4."
    true
    (Float.abs (smoothed_2 -. 4.0) < Float.abs (noisy_2 -. 4.0))
;;

let ex_1_45_nth_roots () =
  Alcotest.(check float_close)
    "square root of 2 via damped search"
    1.4142135623746899
    (Sec_1_45.ex_1_45 2);
  Alcotest.(check int) "damps needed for n=4" 2 (Sec_1_45.damps_needed 4);
  Alcotest.(check int) "damps needed for n=16" 4 (Sec_1_45.damps_needed 16)
;;

let ex_1_46_iterative_improve () =
  let sqrt9, fixed_cos = Sec_1_46.ex_1_46 () in
  Alcotest.(check float_close) "sqrt via iterative_improve" 3.00009155413138 sqrt9;
  Alcotest.(check float_close)
    "fixed_point via iterative_improve"
    0.7390893414033927
    fixed_cos
;;

let ex_1_46a_lazy_guesses () =
  let guesses = Sec_1_46.ex_1_46a () in
  Alcotest.(check (list float_close))
    "first five guesses toward sqrt 2"
    [ 1.0; 1.5; 1.4166666666666665; 1.4142156862745097; 1.4142135623746899 ]
    guesses;
  match Average_damping.sqrt 2.0 with
  | Ok converged ->
    Alcotest.(check float_close)
      "fifth guess already agrees with the section's converged sqrt"
      converged
      (List.nth guesses 4)
  | Error _ -> Alcotest.fail "Average_damping.sqrt 2. was expected to converge"
;;

let () =
  Alcotest.run
    "Section 1.3"
    [ ( "1.29 Simpson's Rule"
      , [ Alcotest.test_case
            "beats the midpoint sum"
            `Quick
            ex_1_29_simpson_beats_integral
        ] )
    ; ( "1.30 iterative sum"
      , [ Alcotest.test_case "sums 1..10" `Quick ex_1_30_iterative_sum ] )
    ; ( "1.31 product"
      , [ Alcotest.test_case "factorial and Wallis pi" `Quick ex_1_31_product_factorial_pi
        ] )
    ; ( "1.32 accumulate"
      , [ Alcotest.test_case "sum and product" `Quick ex_1_32_accumulate ] )
    ; ( "1.33 filtered-accumulate"
      , [ Alcotest.test_case
            "primes and relative primality"
            `Quick
            ex_1_33_filtered_accumulate
        ] )
    ; ( "1.34 apply to itself"
      , [ Alcotest.test_case
            "typechecks against procedures"
            `Quick
            ex_1_34_f_applied_to_procedures
        ] )
    ; ( "1.35 golden ratio"
      , [ Alcotest.test_case "fixed point of 1 + 1/x" `Quick ex_1_35_golden_ratio ] )
    ; ( "1.36 traced fixed point"
      , [ Alcotest.test_case "damping halves the steps" `Quick ex_1_36_traced_fixed_point
        ] )
    ; ( "1.37 continued fraction"
      , [ Alcotest.test_case "converges to 1/phi" `Quick ex_1_37_cont_frac_golden_ratio ]
      )
    ; "1.38 Euler e", [ Alcotest.test_case "matches Float.exp 1." `Quick ex_1_38_euler_e ]
    ; ( "1.39 tangent continued fraction"
      , [ Alcotest.test_case "matches Float.tan" `Quick ex_1_39_tan_cf ] )
    ; ( "1.40 cubic and Newton's method"
      , [ Alcotest.test_case
            "finds a root near the guess"
            `Quick
            ex_1_40_cubic_newtons_method
        ] )
    ; ( "1.41 double combinator"
      , [ Alcotest.test_case "applies inc 16 times" `Quick ex_1_41_double_combinator ] )
    ; "1.42 compose", [ Alcotest.test_case "f after g" `Quick ex_1_42_compose ]
    ; "1.43 repeated", [ Alcotest.test_case "n-fold application" `Quick ex_1_43_repeated ]
    ; "1.44 smoothing", [ Alcotest.test_case "reduces noise" `Quick ex_1_44_smoothing ]
    ; ( "1.45 nth roots"
      , [ Alcotest.test_case "damped fixed-point search" `Quick ex_1_45_nth_roots ] )
    ; ( "1.46 iterative-improve"
      , [ Alcotest.test_case
            "rewrites sqrt and fixed_point"
            `Quick
            ex_1_46_iterative_improve
        ] )
    ; ( "1.46a lazy guesses"
      , [ Alcotest.test_case "matches the section's own sqrt" `Quick ex_1_46a_lazy_guesses
        ] )
    ]
;;
