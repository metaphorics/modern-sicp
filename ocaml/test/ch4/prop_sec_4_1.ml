(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Properties over the solutions of section 4.1.  The unit spot checks
   in [test_sec_4_1.ml] pin the deterministic values; these assert the
   evaluator's invariants over random inputs, because one worked example
   can satisfy them by accident. *)

open QCheck2

let bool_names i = "b" ^ string_of_int (i mod 4)

let rec gen_bool depth =
  if depth = 0
  then
    Gen.oneof
      [ Gen.return "true"; Gen.return "false"; Gen.map bool_names (Gen.int_bound 3) ]
  else
    Gen.oneof
      [ Gen.return "true"
      ; Gen.return "false"
      ; Gen.map bool_names (Gen.int_bound 3)
      ; Gen.map2
          (fun a b -> "(" ^ a ^ " && " ^ b ^ ")")
          (gen_bool (depth - 1))
          (gen_bool (depth - 1))
      ; Gen.map2
          (fun a b -> "(" ^ a ^ " || " ^ b ^ ")")
          (gen_bool (depth - 1))
          (gen_bool (depth - 1))
      ; Gen.map
          (fun c ->
             "(if "
             ^ c
             ^ " then "
             ^ bool_names depth
             ^ " else not "
             ^ bool_names depth
             ^ ")")
          (gen_bool (depth - 1))
      ]
;;

(* The generated bodies read b0..b3.  Admission type-checks before
   evaluation, so the bindings live in the source, not in the
   environment: b0 and b2 are true, b1 and b3 false. *)
let bound source =
  "let b0 = true and b1 = false and b2 = true and b3 = false in (" ^ source ^ ")"
;;

let run eval source =
  match Sicp_ch4.Sec_4_1.expression (bound source) with
  | Error d -> `Rejected d
  | Ok e ->
    (match eval e (Sicp_ch4.Sec_4_1.the_global_environment ()) with
     | Ok v -> `Value (Sicp_common.Value.to_string v)
     | Error e -> `Failed (Sicp_common.Eval_error.to_string e))
;;

(* The same conjunction evaluated three ways always agrees: directly,
   through the special-forms evaluator, and through the derived form. *)
let connectives_agree =
  Test.make
    ~name:"a connective expression answers the same value special, derived, and direct"
    (gen_bool 3)
    (fun source ->
       let direct = run Sicp_ch4.Sec_4_1.eval_expr source in
       let special = run Sicp_ch4_solutions.Sec_4_4.eval_special source in
       let derived = run Sicp_ch4_solutions.Sec_4_4.eval_derived source in
       match direct with
       | `Value _ -> direct = special && direct = derived
       | _ -> false)
;;

(* Lowering a plain let never changes its value: the standard evaluator
   sees the same program as the 4.6 evaluator, and shadowing pins the
   scope. *)
let let_lowering_sound =
  Test.make
    ~name:"let_to_combination preserves the value of a shadowing let"
    Gen.(pair (int_bound 100) (int_bound 100))
    (fun (outer, inner) ->
       let source =
         Printf.sprintf "let x = %d in let x = %d and y = x in x + y" outer inner
       in
       let expected = `Value (string_of_int (inner + outer)) in
       run Sicp_ch4.Sec_4_1.eval_expr source = expected
       && run Sicp_ch4_solutions.Sec_4_6.eval source = expected)
;;

(* Analyzing a connective-heavy body never changes its answer: the
   4.22 lowering plus the section analyzer is the direct run. *)
let analyze_let_sound =
  Test.make
    ~name:"4.22 analyzes a connective let to the direct value"
    (gen_bool 3)
    (fun body ->
       let source =
         Printf.sprintf
           "let r = if %s then 1 else 2 in if r = 1 then %s else not %s"
           body
           body
           body
       in
       let direct = run Sicp_ch4.Sec_4_1.eval_expr source in
       match direct with
       | `Value _ -> direct = run Sicp_ch4_solutions.Sec_4_22.eval source
       | _ -> false)
;;

let () =
  QCheck_base_runner.run_tests_main
    [ connectives_agree; let_lowering_sound; analyze_let_sound ]
;;
