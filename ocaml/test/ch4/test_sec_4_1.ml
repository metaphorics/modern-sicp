(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 4.1. The substrate evaluator's own contract is pinned here
   too, so a solution that leans on a broken clause cannot pass. Every
   exercise's demonstration is pinned to the exact observable outcomes
   the solutions produce; 4.24 is a timing, so only its structure and
   positivity are pinned. *)

let check_string = Alcotest.(check string)
let check_strings = Alcotest.(check (list string))
let check_pos = Alcotest.(check bool)

let run_ok env text =
  match Sicp_ch4.Sec_4_1.run env text with
  | Ok v -> Sicp_common.Value.to_string v
  | Error e -> "Error: " ^ Sicp_common.Eval_error.to_string e
;;

let base_program () =
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  let _ : Sicp_common.Value.t =
    match Sicp_ch4.Sec_4_1.run env "(define (f x) (* x x))" with
    | Ok v -> v
    | Error e -> failwith (Sicp_common.Eval_error.to_string e)
  in
  env
;;

let substrate_define_and_call () =
  check_string
    "a definition answers ok"
    "ok"
    (run_ok (Sicp_ch4.Sec_4_1.the_global_environment ()) "(define x 3)");
  check_string
    "a call computes through the environment"
    "9"
    (run_ok (base_program ()) "(f 3)")
;;

let substrate_errors () =
  check_string
    "an unbound variable is a typed error"
    "Error: unbound variable nope"
    (run_ok (Sicp_ch4.Sec_4_1.the_global_environment ()) "nope");
  check_string
    "a non-procedure operator is not applicable"
    "Error: not applicable: 3 is not a procedure"
    (run_ok (Sicp_ch4.Sec_4_1.the_global_environment ()) "(3 4)")
;;

let reader_cond_arrow () =
  match
    Sicp_common.Reader.read "(cond ((assoc 'b '((a 1) (b 2))) => cadr) (else #f))"
  with
  | Error e -> failwith (Sicp_common.Reader.to_string e)
  | Ok exp ->
    (match Sicp_common.Ast.view exp with
     | Sicp_common.Ast.Cond ((_, [ op; recipient ]) :: _, Some _) ->
       check_string
         "the arrow of an arrow clause arrives as a variable"
         "=>"
         (match Sicp_common.Ast.view op with
          | Sicp_common.Ast.Variable name -> name
          | _ -> "other");
       check_string
         "the recipient follows it"
         "cadr"
         (match Sicp_common.Ast.view recipient with
          | Sicp_common.Ast.Variable name -> name
          | _ -> "other")
     | _ -> failwith "the cond did not parse as clauses plus else")
;;

let base_special_forms () =
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  check_string "quote returns the datum" "(a b)" (run_ok env "'(a b)");
  check_string
    "cond lowers through its clauses"
    "10"
    (run_ok env "(cond ((> 1 2) 1) ((= 1 1) 10) (else 0))");
  check_string
    "cond without a hit answers the false object"
    "#f"
    (run_ok env "(cond ((> 1 2) 1))");
  check_string
    "begin answers the last value"
    "3"
    (run_ok env "(begin (define z 1) (set! z (+ z 2)) z)");
  check_string
    "a lambda value prints as a compound procedure"
    "#[compound-procedure]"
    (run_ok env "(lambda (x) x)");
  check_string
    "a missing if alternative answers the false object"
    "#f"
    (run_ok env "(if #f 1)")
;;

let analyze_equivalence () =
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  let program = "(begin (define (twice f x) (f (f x))) (twice (lambda (n) (* n 3)) 2))" in
  let direct =
    match Sicp_common.Reader.read program with
    | Ok exp -> Sicp_ch4.Sec_4_1.eval exp env
    | Error e ->
      Error (Sicp_common.Eval_error.Invalid_form (Sicp_common.Reader.to_string e))
  in
  let analyzed =
    match Sicp_common.Reader.read program with
    | Ok exp ->
      Sicp_ch4.Sec_4_1.Analyze.eval exp (Sicp_ch4.Sec_4_1.the_global_environment ())
    | Error e ->
      Error (Sicp_common.Eval_error.Invalid_form (Sicp_common.Reader.to_string e))
  in
  let shown = function
    | Ok v -> Sicp_common.Value.to_string v
    | Error e -> "Error: " ^ Sicp_common.Eval_error.to_string e
  in
  check_string "the direct evaluator computes 18" "18" (shown direct);
  check_string "the analyzed evaluator agrees" (shown direct) (shown analyzed)
;;

let ex_4_01_operand_orders () =
  let left, right = Sicp_ch4_solutions.Sec_4_1.ex_4_01 () in
  check_strings
    "left to right evaluates the first operand first"
    [ "1"; "Error: unbound variable not-there" ]
    left;
  check_strings
    "right to left fails before the first operand"
    [ "Error: unbound variable not-there" ]
    right
;;

let ex_4_02_call_prefix () =
  check_strings
    "the call prefix language"
    [ "3"; "ok"; "42"; "Error: invalid form: the application does not start with call" ]
    (Sicp_ch4_solutions.Sec_4_2.ex_4_02 ())
;;

let ex_4_03_data_directed () =
  check_strings
    "dispatch through the table, replacement, and applications last"
    [ "(a b c)"; "yes"; "custom"; "marked"; "3" ]
    (Sicp_ch4_solutions.Sec_4_3.ex_4_03 ())
;;

let ex_4_04_and_or () =
  check_strings
    "and and or, direct and derived, short-circuiting"
    [ "3"; "3"; "7"; "7"; "#t"; "#f" ]
    (Sicp_ch4_solutions.Sec_4_4.ex_4_04 ())
;;

let ex_4_05_cond_arrow () =
  check_strings
    "arrow clauses apply the recipient to the test value"
    [ "2"; "(x c)"; "missing" ]
    (Sicp_ch4_solutions.Sec_4_5.ex_4_05 ())
;;

let ex_4_06_let () =
  check_strings
    "let lowers to an application of a lambda; inits see outer bindings"
    [ "7"; "7"; "5" ]
    (Sicp_ch4_solutions.Sec_4_6.ex_4_06 ())
;;

let ex_4_07_let_star () =
  check_strings
    "let* lowers to nested lets, and equals the hand-written nest"
    [ "39"; "39"; "7" ]
    (Sicp_ch4_solutions.Sec_4_7.ex_4_07 ())
;;

let ex_4_08_named_let () =
  check_strings
    "named let binds the loop procedure and iterates"
    [ "ok"; "55"; "10" ]
    (Sicp_ch4_solutions.Sec_4_8.ex_4_08 ())
;;

let ex_4_09_while () =
  check_strings
    "while iterates a set! accumulator in the environment"
    [ "ok"; "ok"; "#f"; "15" ]
    (Sicp_ch4_solutions.Sec_4_9.ex_4_09 ())
;;

let ex_4_10_new_syntax () =
  check_strings
    "the new syntax translates; eval and apply are untouched"
    [ "16"; "#[compound-procedure]" ]
    (Sicp_ch4_solutions.Sec_4_10.ex_4_10 ())
;;

let ex_4_11_assoc_frames () =
  check_strings
    "the association-list frames obey the environment contract"
    [ "10"; "42"; "7"; "Error: unbound variable z" ]
    (Sicp_ch4_solutions.Sec_4_11.ex_4_11 ())
;;

let ex_4_12_abstract_traversals () =
  check_strings
    "the three operations share the one traversal"
    [ "10"; "42"; "7"; "99"; "Error: unbound variable z" ]
    (Sicp_ch4_solutions.Sec_4_12.ex_4_12 ())
;;

let ex_4_13_make_unbound () =
  check_strings
    "make-unbound! removes the newest binding only"
    [ "2"
    ; "#t"
    ; "Error: unbound variable x"
    ; "1"
    ; "#t"
    ; "Error: unbound variable x"
    ; "#f"
    ]
    (Sicp_ch4_solutions.Sec_4_13.ex_4_13 ())
;;

let ex_4_14_host_map () =
  check_strings
    "Eva's object map works; Louis's host map does not"
    [ "(1 4 9)"
    ; "(1 3)"
    ; "Error: type error: map: the mapping argument is not a primitive procedure"
    ]
    (Sicp_ch4_solutions.Sec_4_14.ex_4_14 ())
;;

let ex_4_14a_host_sort () =
  check_strings
    "the host sort sorts data with a primitive compare and fails on a compound one"
    [ "(1 2 3)"
    ; "Error: type error: sort: the comparator argument is not a primitive procedure"
    ; "(3 2 1)"
    ]
    (Sicp_ch4_solutions.Sec_4_14.ex_4_14a ())
;;

let ex_4_15_halting () =
  check_strings
    "whichever way the oracle answers, the diagonal program breaks it"
    [ "halts? answered #t"
    ; "Error: invalid form: the fuel is exhausted"
    ; "halts? answered #f"
    ; "halted"
    ]
    (Sicp_ch4_solutions.Sec_4_15.ex_4_15 ())
;;

let ex_4_16_scan_out () =
  check_strings
    "mutual recursion works; a read before assignment is an error"
    [ "#t"; "Error: invalid form: b is read before it is assigned" ]
    (Sicp_ch4_solutions.Sec_4_16.ex_4_16 ())
;;

let ex_4_17_extra_frame () =
  check_strings
    "the scan-out adds one frame at the marked point"
    [ "1"; "2" ]
    (Sicp_ch4_solutions.Sec_4_17.ex_4_17 ())
;;

let ex_4_18_alternative_strategy () =
  check_strings
    "the text's strategy works; the eager inner let hits the marker"
    [ "3"; "Error: invalid form: y is read before it is assigned" ]
    (Sicp_ch4_solutions.Sec_4_18.ex_4_18 ())
;;

let ex_4_19_three_disciplines () =
  check_strings
    "sequential, Alyssa, and Eva on the same program"
    [ "16"; "Error: invalid form: a is read before it is assigned"; "20" ]
    (Sicp_ch4_solutions.Sec_4_19.ex_4_19 ())
;;

let ex_4_20_letrec () =
  check_strings
    "letrec reserves and assigns like the scan-out"
    [ "#t"; "2" ]
    (Sicp_ch4_solutions.Sec_4_20.ex_4_20 ())
;;

let ex_4_21_no_define () =
  check_strings
    "self-application recurses without define"
    [ "55"; "#t" ]
    (Sicp_ch4_solutions.Sec_4_21.ex_4_21 ())
;;

let ex_4_22_let_in_analyze () =
  check_strings
    "let analyzed once, then executed, agrees with the direct evaluator"
    [ "7"; "7" ]
    (Sicp_ch4_solutions.Sec_4_22.ex_4_22 ())
;;

let ex_4_23_sequence_counts () =
  check_strings
    "the book's sequence analyzes once; Alyssa's once per call"
    [ "1"; "2" ]
    (Sicp_ch4_solutions.Sec_4_23.ex_4_23 ())
;;

let ex_4_24_benchmark () =
  let direct, analyzed = Sicp_ch4_solutions.Sec_4_24.timings () in
  check_pos "both sides ran in positive time" true (direct > 0.0 && analyzed > 0.0);
  match Sicp_ch4_solutions.Sec_4_24.ex_4_24 () with
  | [ _direct_line; _analyzed_line; ratio ] ->
    check_pos
      "the ratio line names the analyzed side"
      true
      (String.length ratio >= 17 && String.sub ratio 0 17 = "analyzed wins by ")
  | other -> failwith ("ex_4_24 answered " ^ string_of_int (List.length other) ^ " lines")
;;

let () =
  Alcotest.run
    "section 4.1"
    [ ( "section substrate"
      , [ Alcotest.test_case "define and call" `Quick substrate_define_and_call
        ; Alcotest.test_case "typed errors" `Quick substrate_errors
        ; Alcotest.test_case "reader cond arrow" `Quick reader_cond_arrow
        ; Alcotest.test_case "base special forms" `Quick base_special_forms
        ; Alcotest.test_case "analyzed equals direct" `Quick analyze_equivalence
        ] )
    ; ( "exercises"
      , [ Alcotest.test_case "exercise 4.1" `Quick ex_4_01_operand_orders
        ; Alcotest.test_case "exercise 4.2" `Quick ex_4_02_call_prefix
        ; Alcotest.test_case "exercise 4.3" `Quick ex_4_03_data_directed
        ; Alcotest.test_case "exercise 4.4" `Quick ex_4_04_and_or
        ; Alcotest.test_case "exercise 4.5" `Quick ex_4_05_cond_arrow
        ; Alcotest.test_case "exercise 4.6" `Quick ex_4_06_let
        ; Alcotest.test_case "exercise 4.7" `Quick ex_4_07_let_star
        ; Alcotest.test_case "exercise 4.8" `Quick ex_4_08_named_let
        ; Alcotest.test_case "exercise 4.9" `Quick ex_4_09_while
        ; Alcotest.test_case "exercise 4.10" `Quick ex_4_10_new_syntax
        ; Alcotest.test_case "exercise 4.11" `Quick ex_4_11_assoc_frames
        ; Alcotest.test_case "exercise 4.12" `Quick ex_4_12_abstract_traversals
        ; Alcotest.test_case "exercise 4.13" `Quick ex_4_13_make_unbound
        ; Alcotest.test_case "exercise 4.14" `Quick ex_4_14_host_map
        ; Alcotest.test_case "exercise 4.14a" `Quick ex_4_14a_host_sort
        ; Alcotest.test_case "exercise 4.15" `Quick ex_4_15_halting
        ; Alcotest.test_case "exercise 4.16" `Quick ex_4_16_scan_out
        ; Alcotest.test_case "exercise 4.17" `Quick ex_4_17_extra_frame
        ; Alcotest.test_case "exercise 4.18" `Quick ex_4_18_alternative_strategy
        ; Alcotest.test_case "exercise 4.19" `Quick ex_4_19_three_disciplines
        ; Alcotest.test_case "exercise 4.20" `Quick ex_4_20_letrec
        ; Alcotest.test_case "exercise 4.21" `Quick ex_4_21_no_define
        ; Alcotest.test_case "exercise 4.22" `Quick ex_4_22_let_in_analyze
        ; Alcotest.test_case "exercise 4.23" `Quick ex_4_23_sequence_counts
        ; Alcotest.test_case "exercise 4.24" `Quick ex_4_24_benchmark
        ] )
    ]
;;
