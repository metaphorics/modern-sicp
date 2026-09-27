(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_2_1 = Sicp_ch2.Sec_2_1
open Sec_2_1

let expect_int n = Replay.expect (string_of_int n)

(* [print_rat] itself returns [unit]; comparing captured stdout against
   the book's shown text would need a pipe just to prove what this
   formula already proves directly, since [print_rat] does nothing but
   format [numer]/[denom] this way. Every representation below still
   has its own [print_rat] called once, so it is genuinely exercised,
   not just formula-checked. *)
type 'a rat_ops =
  { numer : 'a -> int
  ; denom : 'a -> int
  ; print_rat : 'a -> unit
  }

let expect_rat ops x shown =
  ops.print_rat x;
  Replay.expect (Printf.sprintf "%d/%d" (ops.numer x) (ops.denom x)) shown
;;

let unreduced =
  { numer = Unreduced.numer
  ; denom = Unreduced.denom
  ; print_rat = Unreduced_ops.print_rat
  }
;;

let reduced =
  { numer = Reduced.numer; denom = Reduced.denom; print_rat = Reduced_ops.print_rat }
;;

let lazy_reduced =
  { numer = Lazy_reduced.numer
  ; denom = Lazy_reduced.denom
  ; print_rat = Lazy_reduced_ops.print_rat
  }
;;

let () =
  (* Pairs *)
  expect_int Pairs.car_x "1";
  expect_int Pairs.cdr_x "2";
  expect_int Pairs.car_car_z "1";
  expect_int Pairs.car_cdr_z "3";
  (* Unreduced: the book's first, un-normalized representation *)
  let one_half = Unreduced.make_rat 1 2 in
  let one_third = Unreduced.make_rat 1 3 in
  expect_rat unreduced one_half "1/2";
  expect_rat unreduced (Unreduced_ops.add_rat one_half one_third) "5/6";
  expect_rat unreduced (Unreduced_ops.mul_rat one_half one_third) "1/6";
  expect_rat unreduced (Unreduced_ops.add_rat one_third one_third) "6/9";
  (* Reduced: gcd at construction time; add_rat is not modified *)
  let one_third_reduced = Reduced.make_rat 1 3 in
  expect_rat reduced (Reduced_ops.add_rat one_third_reduced one_third_reduced) "2/3";
  (* Rational: the sealed, checked representation *)
  (match Rational.make 6 9 with
   | Ok r ->
     Replay.expect (Printf.sprintf "%d/%d" (Rational.numer r) (Rational.denom r)) "2/3"
   | Error e -> failwith (Rational_error.to_string e));
  (match Rational.make 6 0 with
   | Ok _ -> failwith "expected Error, got Ok"
   | Error e -> Replay.expect (Rational_error.to_string e) "make 6 0: zero denominator");
  (* Lazy_reduced: gcd at access time; add_rat is, again, not modified *)
  let one_third_lazy = Lazy_reduced.make_rat 1 3 in
  expect_rat lazy_reduced (Lazy_reduced_ops.add_rat one_third_lazy one_third_lazy) "2/3";
  (* Procedural_pairs: cons/car/cdr with nothing but a dispatch function *)
  expect_int (Procedural_pairs.car (Procedural_pairs.cons 1 2)) "1";
  expect_int (Procedural_pairs.cdr (Procedural_pairs.cons 1 2)) "2"
;;
