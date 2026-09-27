(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_1_3 = Sicp_ch1.Sec_1_3
open Sec_1_3

let expect_int n = Replay.expect (string_of_int n)
let expect_bool b = Replay.expect (string_of_bool b)

let expect_ok_float result shown =
  match result with
  | Ok v -> Replay.expect_float v shown
  | Error e -> failwith (Format.asprintf "expected Ok, got Error %a" Numeric_error.pp e)
;;

let expect_error_not_converged result =
  match result with
  | Error Numeric_error.Not_converged -> ()
  | Error Numeric_error.Values_not_of_opposite_sign ->
    failwith "expected Not_converged, got Values_not_of_opposite_sign"
  | Ok v -> failwith (Printf.sprintf "expected Error Not_converged, got Ok %.17g" v)
;;

let () =
  (* Sum_templates *)
  expect_int (Sum_templates.sum_integers 1 10) "55";
  expect_int (Sum_templates.sum_cubes 1 10) "3025";
  Replay.expect_float (Sum_templates.pi_sum 1 1000) "0.39244908194872286";
  (* Sum_abstraction: the reformulations built on one [sum] *)
  Replay.expect_float (Sum_abstraction.sum_cubes 1 10) "3025.";
  Replay.expect_float (Sum_abstraction.sum_integers 1 10) "55.";
  Replay.expect_float (8.0 *. Sum_abstraction.pi_sum 1 1000) "3.139592655589783";
  Replay.expect_float
    (Sum_abstraction.integral Sum_abstraction.cube 0.0 1.0 0.01)
    "0.24998750000000042";
  Replay.expect_float
    (Sum_abstraction.integral Sum_abstraction.cube 0.0 1.0 0.001)
    "0.249999875000001";
  (* Lambda_and_let *)
  Replay.expect_float (8.0 *. Lambda_and_let.pi_sum 1 1000) "3.139592655589783";
  Replay.expect_float
    (Lambda_and_let.integral Sum_abstraction.cube 0.0 1.0 0.01)
    "0.24998750000000042";
  expect_int (Lambda_and_let.plus4 6) "10";
  expect_int (Lambda_and_let.plus4_via_lambda 6) "10";
  expect_int (Lambda_and_let.lambda_as_operator ()) "12";
  Replay.expect_float (Lambda_and_let.f_via_helper 3.0 4.0) "456.";
  Replay.expect_float (Lambda_and_let.f_via_lambda 3.0 4.0) "456.";
  Replay.expect_float (Lambda_and_let.f_via_let 3.0 4.0) "456.";
  Replay.expect_float (Lambda_and_let.let_inner_value ()) "33.";
  Replay.expect_float (Lambda_and_let.let_shadows_outer 5.0) "38.";
  Replay.expect_float (Lambda_and_let.let_binds_from_outer_scope 2.0) "12.";
  (* Half_interval *)
  expect_ok_float (Half_interval.half_interval_method sin 2.0 4.0) "3.14111328125";
  expect_ok_float
    (Half_interval.half_interval_method
       (fun x -> (x *. x *. x) -. (2.0 *. x) -. 3.0)
       1.0
       2.0)
    "1.89306640625";
  (match Half_interval.half_interval_method sin 2.0 3.0 with
   | Error Numeric_error.Values_not_of_opposite_sign -> ()
   | Error Numeric_error.Not_converged ->
     failwith "expected Values_not_of_opposite_sign, got Not_converged"
   | Ok v -> failwith (Printf.sprintf "expected an error, got Ok %.17g" v));
  (* Fixed_point *)
  expect_ok_float (Fixed_point.fixed_point cos 1.0) "0.7390822985224024";
  expect_ok_float
    (Fixed_point.fixed_point (fun y -> sin y +. cos y) 1.0)
    "1.2587315962971173";
  expect_error_not_converged (Fixed_point.fixed_point (fun y -> 2.0 /. y) 1.0);
  (* Average_damping *)
  expect_bool
    (Float.equal (Average_damping.average_damp (fun x -> x *. x) 10.0) 55.0)
    "true";
  expect_ok_float (Average_damping.sqrt 2.0) "1.4142135623746899";
  expect_ok_float (Average_damping.cube_root 27.0) "2.9999972321057697";
  (* Newtons_method *)
  Replay.expect_float (Newtons_method.deriv Newtons_method.cube 5.0) "75.00014999664018";
  expect_ok_float (Newtons_method.sqrt 2.0) "1.4142135623822438";
  (* First_class_procedures *)
  expect_ok_float (First_class_procedures.sqrt_via_average_damp 2.0) "1.4142135623746899";
  expect_ok_float
    (First_class_procedures.sqrt_via_newton_transform 2.0)
    "1.4142135623822438"
;;
