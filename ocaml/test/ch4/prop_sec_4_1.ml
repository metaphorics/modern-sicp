(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Properties over the reference solutions of section 4.1. The unit
   spot checks in [test_sec_4_1.ml] pin the deterministic values;
   these assert the evaluator's invariants over random inputs, because
   one worked example can satisfy them by accident. *)

open QCheck2

let eval_text env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> Sicp_ch4.Sec_4_1.eval exp env
  | Error e ->
    Error (Sicp_common.Eval_error.Invalid_form (Sicp_common.Reader.to_string e))
;;

let shown = function
  | Ok v -> Sicp_common.Value.to_string v
  | Error e -> "Error: " ^ Sicp_common.Eval_error.to_string e
;;

let fresh () = Sicp_ch4.Sec_4_1.the_global_environment ()

(* A definition answers ok, a set! through the nearest binding, and a
   lookup reads what was written. *)
let define_set_lookup n =
  n >= 0
  &&
  let env = fresh () in
  let open_ok = eval_text env "(define x 0)" = Ok (Sicp_common.Value.symbol "ok") in
  let set_ok =
    match eval_text env Printf.(sprintf "(set! x %d)" n) with
    | Error _ -> false
    | Ok _ -> eval_text env "x" = Ok (Sicp_common.Value.int n)
  in
  open_ok && set_ok
;;

let define_set_lookup_prop =
  Test.make
    ~name:"define, set!, and lookup round-trip the newest binding"
    ~count:100
    (Gen.int_range 0 1000)
    define_set_lookup
;;

(* Closures capture their definition environment: two calls of the
   same maker keep independent state. *)
let closures_capture_independently d1 d2 =
  let env = fresh () in
  let _ = eval_text env "(define (make) (define v 0) (lambda (d) (set! v (+ v d)) v))" in
  let _ = eval_text env "(define a (make))" in
  let _ = eval_text env "(define b (make))" in
  let _ = eval_text env (Printf.sprintf "(a %d)" d1) in
  let _ = eval_text env (Printf.sprintf "(b %d)" d2) in
  let _ = eval_text env "(a 3)" in
  shown (eval_text env "(a 0)")
  = Sicp_common.Value.to_string (Sicp_common.Value.int (d1 + 3))
  && shown (eval_text env "(b 0)")
     = Sicp_common.Value.to_string (Sicp_common.Value.int d2)
;;

let closures_capture_prop =
  Test.make
    ~name:"two maker calls keep independent captured state"
    ~count:40
    (Gen.pair (Gen.int_range 0 50) (Gen.int_range 0 50))
    (fun (d1, d2) -> closures_capture_independently d1 d2)
;;

(* The analyzed evaluator agrees with the direct one on a defined
   recursive procedure over random argument sizes. *)
let analyzed_agrees n =
  let definition = "(define (factorial k) (if (= k 1) 1 (* k (factorial (- k 1)))))" in
  let program = Printf.sprintf "(begin %s (factorial %d))" definition n in
  let direct =
    let env = fresh () in
    eval_text env program
  in
  let analyzed =
    match Sicp_common.Reader.read program with
    | Ok exp -> Sicp_ch4.Sec_4_1.Analyze.eval exp (fresh ())
    | Error e ->
      Error (Sicp_common.Eval_error.Invalid_form (Sicp_common.Reader.to_string e))
  in
  shown direct = shown analyzed
;;

let analyzed_agrees_prop =
  Test.make
    ~name:"the analyzed evaluator agrees with the direct one"
    ~count:100
    (Gen.int_range 1 12)
    analyzed_agrees
;;

let () =
  QCheck_base_runner.run_tests_main
    [ define_set_lookup_prop; closures_capture_prop; analyzed_agrees_prop ]
;;
