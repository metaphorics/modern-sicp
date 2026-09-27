(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Eval = Sicp_ch4.Sec_4_1
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let expect_value actual expected = Replay.expect (show actual) expected

(* The analyzed evaluator takes the typed expression, so its driver
   reads the same surface text through the shared reader first. *)
let analyze_run env text =
  match Reader.read text with
  | Ok exp -> Eval.Analyze.eval exp env
  | Error e -> Error (Eval_error.Invalid_form (Reader.to_string e))
;;

let () =
  let env = Eval.the_global_environment () in
  (* 4.1.4: the driver sample, from the section's interaction. *)
  expect_value
    (Eval.run
       env
       "(define (append x y) (if (null? x) y (cons (car x) (append (cdr x) y))))")
    "ok";
  expect_value (Eval.run env "(append '(a b c) '(d e f))") "(a b c d e f)";
  (* 4.1.5: the factorial program is data for the evaluator. *)
  expect_value
    (Eval.run env "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))")
    "ok";
  expect_value (Eval.run env "(factorial 5)") "120";
  (* 4.1.7: the analyzed evaluator runs the same program to the same
     value. *)
  let analyzed = Eval.the_global_environment () in
  expect_value
    (analyze_run
       analyzed
       "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))")
    "ok";
  expect_value (analyze_run analyzed "(factorial 10)") "3628800";
  (* The error channel of a primitive reaches the driver untouched. *)
  expect_value (Eval.run env "(car 5)") "Error: type error: car: not a pair: 5"
;;
