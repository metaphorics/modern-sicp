(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The runner touches every listing of section 2.4 in the order the book
   presents it, asserting each value. The book shows no interactive
   results for this section (every listing is a definition, not a
   call), so the values checked here are this edition's own worked
   examples: a 3-4-5 triangle for the rectangular/polar arithmetic, and
   the pair (3, 4) plus (1, 2) tagged two different ways for the
   generic-dispatch payoff. *)

module Replay = Sicp_ch1.Replay
module S = Sicp_ch2.Sec_2_4

let expect_float = Replay.expect_float
let expect_bool computed shown = Replay.expect (string_of_bool computed) shown

let () =
  (* 2.4.1: Ben's rectangular representation, on (3, 4). *)
  let open S.Untagged in
  let rect34 = 3.0, 4.0 in
  expect_float (Rectangular.real_part rect34) "3.";
  expect_float (Rectangular.imag_part rect34) "4.";
  expect_float (Rectangular.magnitude rect34) "5.";
  expect_float (Rectangular.angle rect34) "0.9272952180016122";
  let rrx, rry = Rectangular.make_from_real_imag 3.0 4.0 in
  expect_float rrx "3.";
  expect_float rry "4.";
  let rmx, rmy = Rectangular.make_from_mag_ang 5.0 0.9272952180016122 in
  expect_float rmx "3.0000000000000004";
  expect_float rmy "3.9999999999999996";
  (* Alyssa's polar representation, on (5, 0). *)
  let polar50 = 5.0, 0.0 in
  expect_float (Polar.real_part polar50) "5.";
  expect_float (Polar.imag_part polar50) "0.";
  expect_float (Polar.magnitude polar50) "5.";
  expect_float (Polar.angle polar50) "0.";
  let pmx, pmy = Polar.make_from_mag_ang 5.0 0.0 in
  expect_float pmx "5.";
  expect_float pmy "0.";
  let pr, pa = Polar.make_from_real_imag 5.0 0.0 in
  expect_float pr "5.";
  expect_float pa "0.";
  (* The same add_complex/sub_complex/mul_complex works with either
     representation: rectangular (3, 4) + (1, 2), and the same product
     computed the roundabout way, through magnitude and angle, still
     lands on the exact answer (3+4i)(1+2i) = -5+10i. *)
  let sum_rx, sum_ry = add_complex rectangular_ops (3.0, 4.0) (1.0, 2.0) in
  expect_float sum_rx "4.";
  expect_float sum_ry "6.";
  let prod_rx, prod_ry = mul_complex rectangular_ops (3.0, 4.0) (1.0, 2.0) in
  expect_float prod_rx "-5.";
  expect_float prod_ry "10.";
  (* Polar (5, angle34) is (3, 4) in Ben's terms; polar (r2, a2) is
     (1, 2). Adding through polar_ops takes the roundabout route
     (real/imag parts of each, summed, then converted back to
     magnitude and angle) and comes back close to (4, 6). *)
  let angle34 = 0.9272952180016122 in
  let r2, a2 = 2.23606797749979, 1.1071487177940904 in
  let sum_polar_r, sum_polar_a = add_complex polar_ops (5.0, angle34) (r2, a2) in
  expect_float sum_polar_r "7.211102550927979";
  expect_float sum_polar_a "0.9827937232473289"
;;

(* 2.4.2: tagged data. Rectangular (3, 4) and polar (r2, a2) [= (1, 2)]
   coexist as [S.tagged] values distinguished only by [S.type_tag]. *)
let () =
  let z_rect = S.attach_tag "rectangular" (S.Pair (3.0, 4.0)) in
  let z_polar = S.attach_tag "polar" (S.Pair (2.23606797749979, 1.1071487177940904)) in
  expect_bool (S.is_rectangular z_rect) "true";
  expect_bool (S.is_polar z_rect) "false";
  expect_bool (S.is_polar z_polar) "true";
  expect_float (S.real_part z_rect) "3.";
  expect_float (S.magnitude z_rect) "5.";
  expect_float (S.real_part z_polar) "1.0000000000000002";
  expect_float (S.imag_part z_polar) "2.";
  (* The payoff: real_part, imag_part, magnitude, and angle no longer
     care which representation their argument uses, so add_complex
     can add a rectangular number to a polar one directly. *)
  let sum = S.add_complex z_rect z_polar in
  expect_bool (S.is_rectangular sum) "true";
  (match S.contents_of sum with
   | S.Pair (x, y) ->
     expect_float x "4.";
     expect_float y "6."
   | S.Num _ | S.Tagged _ -> failwith "add_complex: expected a Pair contents");
  let product = S.mul_complex z_rect z_polar in
  expect_bool (S.is_polar product) "true";
  match S.contents_of product with
  | S.Pair (r, a) ->
    expect_float r "11.180339887498949";
    expect_float a "2.0344439357957027"
  | S.Num _ | S.Tagged _ -> failwith "mul_complex: expected a Pair contents"
;;

(* 2.4.3: data-directed programming. Installing both packages once
   makes the same [add_complex] work, this time by looking the right
   procedure up in a table instead of testing the tag with [cond]. *)
let () =
  let open S.Data_directed in
  install_rectangular_package ();
  install_polar_package ();
  let z_rect = S.attach_tag "rectangular" (S.Pair (3.0, 4.0)) in
  let z_polar = S.attach_tag "polar" (S.Pair (2.23606797749979, 1.1071487177940904)) in
  expect_float (real_part z_rect) "3.";
  expect_float (magnitude z_polar) "2.23606797749979";
  let sum = add_complex z_rect z_polar in
  expect_bool (S.is_rectangular sum) "true";
  (match S.contents_of sum with
   | S.Pair (x, y) ->
     expect_float x "4.";
     expect_float y "6."
   | S.Num _ | S.Tagged _ -> failwith "add_complex: expected a Pair contents");
  let built = make_from_real_imag 3.0 4.0 in
  expect_bool (S.is_rectangular built) "true";
  let built_polar = make_from_mag_ang 5.0 0.0 in
  expect_bool (S.is_polar built_polar) "true"
;;

(* Message passing: [make_from_real_imag] returns a closure that
   answers whichever operation it is sent. *)
let () =
  let open S.Message_passing in
  let z = make_from_real_imag 3.0 4.0 in
  expect_float (apply_generic Real_part z) "3.";
  expect_float (apply_generic Imag_part z) "4.";
  expect_float (apply_generic Magnitude z) "5.";
  expect_float (apply_generic Angle z) "0.9272952180016122"
;;
