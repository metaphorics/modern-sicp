(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner installs every package section 2.5's narrative builds,
   then replays a worked example from each subsection: a
   scheme-number/rational/complex generic-arithmetic tower (2.5.1), a
   scheme-number added to a complex through the cross-type procedure
   and again through coercion (2.5.2), and a polynomial addition and
   multiplication (2.5.3). *)

module Replay = Sicp_ch1.Replay
module S = Sicp_ch2.Sec_2_5

let expect_float = Replay.expect_float
let expect_bool computed shown = Replay.expect (string_of_bool computed) shown

let () =
  S.install_scheme_number_package ();
  S.install_rational_package ();
  S.install_rectangular_package ();
  S.install_polar_package ();
  S.install_complex_package ();
  S.install_polynomial_package ()
;;

(* 2.5.1: one generic [add] spans scheme-number, rational, and
   complex, dispatching through the operation-and-type table instead
   of a per-type [cond]. *)
let () =
  let three = S.make_scheme_number 3.0 in
  let four = S.make_scheme_number 4.0 in
  (match S.add three four with
   | S.Tagged { tag = "scheme-number"; contents = S.Num n } -> expect_float n "7."
   | _ -> failwith "add: expected a tagged scheme-number");
  let half = S.make_rational 1 2 in
  let quarter = S.make_rational 1 4 in
  (match S.add half quarter with
   | S.Tagged { tag = "rational"; contents = S.Ratpair (n, d) } ->
     expect_bool (n = 3 && d = 4) "true"
   | _ -> failwith "add: expected a tagged rational");
  let z1 = S.make_complex_from_real_imag 3.0 4.0 in
  let z2 = S.make_complex_from_real_imag 1.0 2.0 in
  match S.add z1 z2 with
  | S.Tagged { tag = "complex"; contents = S.Tagged { contents = S.Cpx (x, y); _ } } ->
    expect_float x "4.";
    expect_float y "6."
  | _ -> failwith "add: expected a tagged complex"
;;

(* 2.5.2: the cumbersome cross-type procedure and, once installed,
   the coercion table both answer [3 + (1+2i)]; both routes must
   agree. *)
let () =
  S.install_cross_type_example ();
  let three = S.make_scheme_number 3.0 in
  let z = S.make_complex_from_real_imag 1.0 2.0 in
  let via_cross_type = S.apply_generic_coerce "add" [ z; three ] in
  (match via_cross_type with
   | S.Tagged { tag = "complex"; contents = S.Tagged { contents = S.Cpx (x, y); _ } } ->
     expect_float x "4.";
     expect_float y "2."
   | _ -> failwith "add: expected a tagged complex");
  S.install_coercions ();
  match S.add three z with
  | S.Tagged { tag = "complex"; contents = S.Tagged { contents = S.Cpx (x, y); _ } } ->
    expect_float x "4.";
    expect_float y "2."
  | _ -> failwith "add: expected a tagged complex"
;;

(* 2.5.3: [x^2 + 1] and [x^3 + 1], added and multiplied through the
   same generic [add]/[mul] the numeric packages use. *)
let () =
  let one = S.make_scheme_number 1.0 in
  let p1 = S.make_polynomial "x" [ S.make_term 2 one; S.make_term 0 one ] in
  let p2 = S.make_polynomial "x" [ S.make_term 3 one; S.make_term 0 one ] in
  let term_floats = function
    | S.Poly p ->
      List.map
        (fun t ->
           match S.coeff t with
           | S.Tagged { contents = S.Num n; _ } -> S.order t, n
           | _ -> failwith "term_floats: expected a tagged scheme-number coefficient")
        (S.term_list p)
    | _ -> failwith "term_floats: expected a polynomial"
  in
  (match S.add p1 p2 with
   | S.Tagged { tag = "polynomial"; contents } ->
     expect_bool (term_floats contents = [ 3, 1.0; 2, 1.0; 0, 2.0 ]) "true"
   | _ -> failwith "add: expected a tagged polynomial");
  match S.mul p1 p2 with
  | S.Tagged { tag = "polynomial"; contents } ->
    expect_bool (term_floats contents = [ 5, 1.0; 3, 1.0; 2, 1.0; 0, 1.0 ]) "true"
  | _ -> failwith "mul: expected a tagged polynomial"
;;
