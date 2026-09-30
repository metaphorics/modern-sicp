(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 2.5. [sicp_ch2_solutions] implements the same contracts the
   exercises library states with pending stubs, so these tests double
   as the proof of each exercise's stated answer. *)

module S77 = Sicp_ch2_solutions.Sec_2_77
module S78 = Sicp_ch2_solutions.Sec_2_78
module S79 = Sicp_ch2_solutions.Sec_2_79
module S80 = Sicp_ch2_solutions.Sec_2_80
module S81 = Sicp_ch2_solutions.Sec_2_81
module S82 = Sicp_ch2_solutions.Sec_2_82
module S83 = Sicp_ch2_solutions.Sec_2_83
module S84 = Sicp_ch2_solutions.Sec_2_84
module S85 = Sicp_ch2_solutions.Sec_2_85
module S86 = Sicp_ch2_solutions.Sec_2_86
module S87 = Sicp_ch2_solutions.Sec_2_87
module S88 = Sicp_ch2_solutions.Sec_2_88
module S89 = Sicp_ch2_solutions.Sec_2_89
module S90 = Sicp_ch2_solutions.Sec_2_90
module S91 = Sicp_ch2_solutions.Sec_2_91
module S92 = Sicp_ch2_solutions.Sec_2_92
module S93 = Sicp_ch2_solutions.Sec_2_93
module S94 = Sicp_ch2_solutions.Sec_2_94
module S95 = Sicp_ch2_solutions.Sec_2_95
module S96 = Sicp_ch2_solutions.Sec_2_96
module S97 = Sicp_ch2_solutions.Sec_2_97

let eq_bool name a b = Alcotest.check Alcotest.bool name true (a = b)
let feq = Alcotest.float 1e-9
let int_pairs = Alcotest.(list (pair int int))
let int_pair = Alcotest.(pair int int)

let ex_2_77_two_apply_generic_calls () =
  let raised_before_fix, magnitude_after_fix, calls_after_fix = S77.ex_2_77 () in
  eq_bool "2.77: no method before the fix" raised_before_fix true;
  Alcotest.check feq "2.77: magnitude of (3, 4)" 5.0 magnitude_after_fix;
  Alcotest.check Alcotest.int "2.77: exactly two apply_generic calls" 2 calls_after_fix
;;

let ex_2_78_bare_num_is_its_own_tag () =
  let tag, contents_is_num_5, rational_tag = S78.ex_2_78 () in
  Alcotest.check Alcotest.string "2.78: bare Num tags as real" "real" tag;
  eq_bool "2.78: contents of a bare Num is itself" contents_is_num_5 true;
  Alcotest.check Alcotest.string "2.78: a real tag still wraps" "rational" rational_tag
;;

let ex_2_79_equ () =
  let sn_eq, sn_neq, rat_eq, cpx_eq, cpx_neq = S79.ex_2_79 () in
  eq_bool "2.79: 3 = 3" sn_eq true;
  eq_bool "2.79: 3 <> 4" sn_neq false;
  eq_bool "2.79: 1/2 = 2/4" rat_eq true;
  eq_bool "2.79: (3,4) = (3,4)" cpx_eq true;
  eq_bool "2.79: (3,4) <> (3,5)" cpx_neq false
;;

let ex_2_80_is_zero () =
  let sn_zero, sn_nonzero, rat_zero, rat_nonzero, cpx_zero, cpx_nonzero =
    S80.ex_2_80 ()
  in
  eq_bool "2.80: 0 is zero" sn_zero true;
  eq_bool "2.80: 3 is not zero" sn_nonzero false;
  eq_bool "2.80: 0/5 is zero" rat_zero true;
  eq_bool "2.80: 1/5 is not zero" rat_nonzero false;
  eq_bool "2.80: 0+0i is zero" cpx_zero true;
  eq_bool "2.80: 0+1i is not zero" cpx_nonzero false
;;

let ex_2_81_self_coercion_loops_then_is_fixed () =
  let depth = S81.ex_2_81_a () in
  eq_bool "2.81a: the loop reaches the depth guard" (depth > 1000) true;
  eq_bool "2.81c: the fixed apply_generic raises immediately" (S81.ex_2_81_c ()) true
;;

let ex_2_82_multi_arg_coercion () =
  (match S82.ex_2_82_a () with
   | S82.Tagged { tag = "real"; contents = S82.Num n } ->
     Alcotest.check feq "2.82a: 1 + 2 + 3" 6.0 n
   | _ -> Alcotest.fail "2.82a: expected a tagged real");
  eq_bool "2.82b: the strategy misses a valid mixed-type entry" (S82.ex_2_82_b ()) true
;;

let ex_2_83_raise_through_the_tower () =
  let tag_after_rational, tag_after_real, tag_after_complex, (x, y) = S83.ex_2_83 () in
  Alcotest.check
    Alcotest.string
    "2.83: integer raises to rational"
    "rational"
    tag_after_rational;
  Alcotest.check Alcotest.string "2.83: rational raises to real" "real" tag_after_real;
  Alcotest.check
    Alcotest.string
    "2.83: real raises to complex"
    "complex"
    tag_after_complex;
  Alcotest.check feq "2.83: 3 raised to complex, real part" 3.0 x;
  Alcotest.check feq "2.83: 3 raised to complex, imaginary part" 0.0 y
;;

let ex_2_84_successive_raising () =
  let x, y = S84.ex_2_84 () in
  Alcotest.check feq "2.84: 3 + (2+3i), real part" 5.0 x;
  Alcotest.check feq "2.84: 3 + (2+3i), imaginary part" 3.0 y
;;

let ex_2_85_drop () =
  let tag_a, tag_b, tag_c, tag_sum = S85.ex_2_85 () in
  Alcotest.check Alcotest.string "2.85: 1.5+0i drops to real" "real" tag_a;
  Alcotest.check Alcotest.string "2.85: 1+0i drops to integer" "integer" tag_b;
  Alcotest.check Alcotest.string "2.85: 2+3i cannot be lowered" "complex" tag_c;
  Alcotest.check Alcotest.string "2.85: (2,3)+(-2,-3) drops to integer" "integer" tag_sum
;;

let ex_2_85a_coercion_graph_cycles () =
  let acyclic_tower, self_coercion_cycle, three_step_cycle = S85.ex_2_85a () in
  eq_bool "2.85a: the tower's coercions have no cycle" acyclic_tower false;
  eq_bool "2.85a: a self-coercion is a one-edge cycle" self_coercion_cycle true;
  eq_bool "2.85a: a -> b -> c -> a is a cycle" three_step_cycle true
;;

let ex_2_86_generic_complex_parts () =
  let real_part, imag_part, magnitude, angle = S86.ex_2_86 () in
  Alcotest.check feq "2.86: real part of (3, 4)" 3.0 real_part;
  Alcotest.check feq "2.86: imaginary part of (3, 4)" 4.0 imag_part;
  Alcotest.check feq "2.86: magnitude of (3, 4)" 5.0 magnitude;
  Alcotest.check feq "2.86: angle of (3, 4)" (Float.atan2 4.0 3.0) angle
;;

let ex_2_87_zero_polynomial_coefficient_dropped () =
  let surviving_count, empty_poly_is_zero = S87.ex_2_87 () in
  Alcotest.check
    Alcotest.int
    "2.87: only the nonzero-coefficient term survives"
    1
    surviving_count;
  eq_bool "2.87: the empty y-polynomial is zero" empty_poly_is_zero true
;;

let ex_2_88_polynomial_subtraction () =
  let result = S88.ex_2_88 () in
  Alcotest.check
    Alcotest.(list (pair int (float 1e-9)))
    "2.88: (x^2+3x+1) - (x^2+1) = 3x"
    [ 1, 3.0 ]
    result
;;

let ex_2_89_dense_term_lists () =
  let dense, first, sparse = S89.ex_2_89 () in
  Alcotest.check
    Alcotest.(list int)
    "2.89: dense list of x^5+2x^4+3x^2-2x-5"
    [ 1; 2; 0; 3; -2; -5 ]
    dense;
  Alcotest.check int_pair "2.89: leading term is (5, 1)" (5, 1) first;
  Alcotest.check
    int_pairs
    "2.89: to_terms undoes of_terms"
    [ 5, 1; 4, 2; 2, 3; 1, -2; 0, -5 ]
    sparse
;;

let ex_2_90_sparse_and_dense_agree () =
  let sparse_result, dense_result = S90.ex_2_90 () in
  Alcotest.check int_pairs "2.90: sparse (x^2+1)+(x+1)" [ 2, 1; 1, 1; 0, 2 ] sparse_result;
  Alcotest.check int_pairs "2.90: dense agrees with sparse" sparse_result dense_result
;;

let ex_2_91_polynomial_division () =
  let quotient, remainder = S91.ex_2_91 () in
  Alcotest.check
    Alcotest.(list (pair int (float 1e-9)))
    "2.91: (x^5-1)/(x^2-1) quotient is x^3+x"
    [ 3, 1.0; 1, 1.0 ]
    quotient;
  Alcotest.check
    Alcotest.(list (pair int (float 1e-9)))
    "2.91: (x^5-1)/(x^2-1) remainder is x-1"
    [ 1, 1.0; 0, -1.0 ]
    remainder
;;

let ex_2_92_multivariable_addition () =
  let var, terms = S92.ex_2_92 () in
  Alcotest.check Alcotest.string "2.92: x outranks y, result stays in x" "x" var;
  match terms with
  | [ (1, S92.Num c1); (0, S92.Sub_poly p) ] ->
    Alcotest.check feq "2.92: x^1 coefficient is 1" 1.0 c1;
    Alcotest.check Alcotest.string "2.92: promoted coefficient is in y" "y" p.var;
    Alcotest.check
      int_pairs
      "2.92: promoted coefficient is 3y + 2"
      [ 1, 3; 0, 2 ]
      (List.map (fun (t : S92.term) -> t.order, int_of_float t.coeff) p.term_list)
  | _ -> Alcotest.fail "2.92: expected [(1, Num _); (0, Sub_poly _)]"
;;

let ex_2_93_unreduced_rational_function () =
  let numer, denom = S93.ex_2_93 () in
  (* rf = p2/p1 with p1 = x^2+1, p2 = x^3+1; rf+rf has numer = 2*p2*p1 and
     denom = p1*p1, neither of which shares a factor once expanded, so the
     result is not the trivially-reduced [2*p2/p1]. *)
  Alcotest.check
    Alcotest.(list (pair int (float 1e-9)))
    "2.93: numerator is 2*p2*p1"
    [ 5, 2.0; 3, 2.0; 2, 2.0; 0, 2.0 ]
    numer;
  Alcotest.check
    Alcotest.(list (pair int (float 1e-9)))
    "2.93: denominator is p1^2"
    [ 4, 1.0; 2, 2.0; 0, 1.0 ]
    denom
;;

let ex_2_94_polynomial_gcd () =
  let _terms, divides_p1, divides_p2 = S94.ex_2_94 () in
  eq_bool "2.94: the gcd divides P1 evenly" divides_p1 true;
  eq_bool "2.94: the gcd divides P2 evenly" divides_p2 true
;;

let ex_2_95_naive_gcd_can_fail () =
  let _terms, ratio = S95.ex_2_95 () in
  eq_bool "2.95: naive gcd is not proportional to P1" (Option.is_none ratio) true
;;

let ex_2_96_pseudodivision_fixes_it () =
  let no_inexact_division, reduced_content = S96.ex_2_96 () in
  eq_bool "2.96a: pseudodivision never divides inexactly" no_inexact_division true;
  Alcotest.check
    Alcotest.int
    "2.96b: the reduced gcd's coefficients are coprime"
    1
    reduced_content
;;

let ex_2_97_reduce_to_lowest_terms () =
  let _numer, _denom, cross_multiply_agrees = S97.ex_2_97 () in
  eq_bool
    "2.97: reduced fraction cross-multiplies to the unreduced ratio"
    cross_multiply_agrees
    true
;;

let () =
  Alcotest.run
    "sec_2_5"
    [ ( "exercises"
      , Alcotest.
          [ test_case "2.77 nested apply_generic" `Quick ex_2_77_two_apply_generic_calls
          ; test_case
              "2.78 bare Num is its own tag"
              `Quick
              ex_2_78_bare_num_is_its_own_tag
          ; test_case "2.79 equ?" `Quick ex_2_79_equ
          ; test_case "2.80 =zero?" `Quick ex_2_80_is_zero
          ; test_case
              "2.81 self-coercion loops, then is fixed"
              `Quick
              ex_2_81_self_coercion_loops_then_is_fixed
          ; test_case "2.82 multi-argument coercion" `Quick ex_2_82_multi_arg_coercion
          ; test_case
              "2.83 raise through the tower"
              `Quick
              ex_2_83_raise_through_the_tower
          ; test_case "2.84 successive raising" `Quick ex_2_84_successive_raising
          ; test_case "2.85 drop" `Quick ex_2_85_drop
          ; test_case "2.85a coercion-graph cycles" `Quick ex_2_85a_coercion_graph_cycles
          ; test_case "2.86 generic complex parts" `Quick ex_2_86_generic_complex_parts
          ; test_case
              "2.87 zero polynomial coefficient dropped"
              `Quick
              ex_2_87_zero_polynomial_coefficient_dropped
          ; test_case "2.88 polynomial subtraction" `Quick ex_2_88_polynomial_subtraction
          ; test_case "2.89 dense term lists" `Quick ex_2_89_dense_term_lists
          ; test_case "2.90 sparse and dense agree" `Quick ex_2_90_sparse_and_dense_agree
          ; test_case "2.91 polynomial division" `Quick ex_2_91_polynomial_division
          ; test_case "2.92 multivariable addition" `Quick ex_2_92_multivariable_addition
          ; test_case
              "2.93 unreduced rational function"
              `Quick
              ex_2_93_unreduced_rational_function
          ; test_case "2.94 polynomial gcd" `Quick ex_2_94_polynomial_gcd
          ; test_case "2.95 naive gcd can fail" `Quick ex_2_95_naive_gcd_can_fail
          ; test_case
              "2.96 pseudodivision fixes it"
              `Quick
              ex_2_96_pseudodivision_fixes_it
          ; test_case "2.97 reduce to lowest terms" `Quick ex_2_97_reduce_to_lowest_terms
          ] )
    ]
;;
