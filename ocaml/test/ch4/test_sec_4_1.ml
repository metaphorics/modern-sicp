(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the solutions of section 4.1: every exercise's
   demonstration is pinned to the exact observable outcomes the
   solutions produce.  The substrate evaluator's contract is pinned too,
   so a solution that leans on a broken clause cannot pass.  The 4.24
   timing is inherently unstable, so only its structure and positivity
   are pinned. *)

let check_strings = Alcotest.(check (list string))

let unwrap = function
  | Ok lines -> lines
  | Error e -> Alcotest.failf "setup: %s" (Sicp_common.Eval_error.to_string e)
;;

let substrate_runs_definition () =
  Alcotest.(check string)
    "a definition and a call through the substrate evaluator"
    "3"
    (Sicp_ch4_solutions.Sec_4_1.run_source
       Sicp_ch4.Sec_4_1.eval_expr
       "let x = 1 + 2 in x")
;;

let substrate_rejects_while () =
  Alcotest.(check string)
    "the subset rejects a native loop"
    "rejected: host-valid, subset-unsupported at line 1, column 27: an expression \
     outside the grammar's expr production"
    (Sicp_ch4_solutions.Sec_4_1.run_source
       Sicp_ch4.Sec_4_1.eval_expr
       "let i = ref 0 in while !i < 3 do i := !i + 1 done; !i")
;;

let ex name f expected () = check_strings name expected (f ())
let exr name f expected () = check_strings name expected (unwrap (f ()))

let tests =
  [ "substrate runs a definition and a call", `Quick, substrate_runs_definition
  ; "substrate rejects a native loop", `Quick, substrate_rejects_while
  ; ( "4.1 prints the operand order of each evaluator, then of each evaluator on a \
       failing pair"
    , `Quick
    , ex
        "4.1"
        Sicp_ch4_solutions.Sec_4_1.ex_4_01
        [ "left right 7"
        ; "right left 7"
        ; "left error: division by zero"
        ; "right left error: division by zero"
        ] )
  ; ( "4.2 answers Louis's misread bindings under his dispatch, then the call language \
       and its rejection"
    , `Quick
    , ex
        "4.2"
        Sicp_ch4_solutions.Sec_4_2.ex_4_02
        [ "error: unbound variable x"
        ; "error: not applicable: 1 is not a procedure"
        ; "\"42\""
        ; "\"42\""
        ; "42"
        ; "120"
        ; "error: invalid form: an application must start with call"
        ] )
  ; ( "4.3 answers factorail through a counting if handler"
    , `Quick
    , ex "4.3" Sicp_ch4_solutions.Sec_4_3.ex_4_03 [ "120"; "if handled 6 times"; "true" ]
    )
  ; ( "4.4 agrees special and derived on four connectives, then the variadic edges"
    , `Quick
    , ex
        "4.4"
        Sicp_ch4_solutions.Sec_4_4.ex_4_04
        [ "false / false"
        ; "true / true"
        ; "true / true"
        ; "false / false"
        ; "true"
        ; "false"
        ; "false"
        ; "true"
        ] )
  ; ( "4.5 answers the found value, the fall-through, the arrow, the default, and the \
       malformed one"
    , `Quick
    , exr
        "4.5"
        Sicp_ch4_solutions.Sec_4_5.ex_4_05
        [ "2"; "7"; "4"; "0"; "error: invalid form: Else must be the last clause" ] )
  ; ( "4.6 lowers one let, matches the base evaluator, and pins the scoping through \
       shadowing"
    , `Quick
    , ex "4.6" Sicp_ch4_solutions.Sec_4_6.ex_4_06 [ "7"; "7"; "5"; "effect 1"; "120" ] )
  ; ( "4.7 answers 39 through both evaluators and the rebinding"
    , `Quick
    , exr "4.7" Sicp_ch4_solutions.Sec_4_7.ex_4_07 [ "39"; "39"; "20" ] )
  ; ( "4.8 answers fib 10, the plain let, the shadowed init, and the empty named let"
    , `Quick
    , exr "4.8" Sicp_ch4_solutions.Sec_4_8.ex_4_08 [ "55"; "12"; "7"; "10" ] )
  ; ( "4.9 answers the sum, the squares, the empty loop, and the two rejections"
    , `Quick
    , ex
        "4.9"
        Sicp_ch4_solutions.Sec_4_9.ex_4_09
        [ "55"
        ; "1 4 9 16 25 ()"
        ; "()"
        ; "rejected: host-valid, subset-unsupported at line 1, column 27: an expression \
           outside the grammar's expr production"
        ; "rejected: host-valid, subset-unsupported at line 1, column 9: an expression \
           outside the grammar's expr production"
        ] )
  ; ( "4.10 evaluates the new syntax factorial, sequence, call, and unknown operator"
    , `Quick
    , ex
        "4.10"
        Sicp_ch4_solutions.Sec_4_10.ex_4_10
        [ "720"; "new \"syntax!\""; "7"; "error: invalid form: unknown operator %" ] )
  ; ( "4.11 pins the frames, the shadowing, and the two failure shapes"
    , `Quick
    , exr
        "4.11"
        Sicp_ch4_solutions.Sec_4_11.ex_4_11
        [ "[x = 100] -> [z = 3; x = 10; y = 20]"
        ; "100"
        ; "10"
        ; "20"
        ; "error: unbound variable w"
        ; "error: arity mismatch: expected 1, given 0"
        ] )
  ; ( "4.12 pins the traversal's reads from each frame"
    , `Quick
    , exr
        "4.12"
        Sicp_ch4_solutions.Sec_4_12.ex_4_12
        [ "11"; "1"; "20"; "3"; "error: unbound variable c"; "error: unbound variable d" ]
    )
  ; ( "4.13 unbinds the shadowing and the defined name but never the outer names"
    , `Quick
    , exr
        "4.13"
        Sicp_ch4_solutions.Sec_4_13.ex_4_13
        [ "unbound x"
        ; "1"
        ; "error: unbound variable y"
        ; "2"
        ; "unbound w"
        ; "error: unbound variable w"
        ] )
  ; ( "4.14 maps a primitive with the host map and a guest function with Eva's and the \
       prelude's"
    , `Quick
    , ex
        "4.14"
        Sicp_ch4_solutions.Sec_4_14.ex_4_14
        [ "[\"1\"; \"2\"; \"3\"]"
        ; "error: type error: host code cannot call the guest closure; only the \
           evaluator can"
        ; "[1; 4; 9]"
        ; "[1; 4; 9]"
        ] )
  ; ( "4.14a sorts with Eva's and the prelude's, never with the host sort"
    , `Quick
    , ex
        "4.14a"
        Sicp_ch4_solutions.Sec_4_14.ex_4_14a
        [ "error: type error: host code cannot call the guest closure; only the \
           evaluator can"
        ; "[3; 2; 1]"
        ; "[3; 2; 1]"
        ] )
  ; ( "4.15 contradicts each constant halting oracle with its own answer"
    , `Quick
    , ex
        "4.15"
        Sicp_ch4_solutions.Sec_4_15.ex_4_15
        [ "halts try_ try_ = true; try_ try_ -> error: no answer within the step budget"
        ; "halts try_ try_ = false; try_ try_ -> \"halted\""
        ] )
  ; ( "4.16 keeps mutual recursion and rejects eager reads of constructive groups"
    , `Quick
    , exr
        "4.16"
        Sicp_ch4_solutions.Sec_4_16.ex_4_16
        [ "true"
        ; "true"
        ; "error: unassigned variable ys"
        ; "4"
        ; "rejected: type-invalid at line 1, column 32: the pinned type checker rejects \
           the source"
        ; "cells: even, odd"
        ] )
  ; ( "4.17 counts one frame fewer when the definitions share the call frame"
    , `Quick
    , ex
        "4.17"
        Sicp_ch4_solutions.Sec_4_17.ex_4_17
        [ "scanned: 36 in 5 frames"; "shared frame: 36 in 4 frames" ] )
  ; ( "4.18 runs the solve-shaped group where delayed uses work and eager ones fail"
    , `Quick
    , ex
        "4.18"
        Sicp_ch4_solutions.Sec_4_18.ex_4_18
        [ "4"; "error: unassigned variable y"; "4"; "8" ] )
  ; ( "4.19 answers Ben's 16, Alyssa's error, Eva's 20, and OCaml's 20"
    , `Quick
    , ex
        "4.19"
        Sicp_ch4_solutions.Sec_4_19.ex_4_19
        [ "Ben: 16"; "Alyssa: error: unassigned variable a"; "Eva: 20"; "OCaml: 20" ] )
  ; ( "4.20 derives let rec and refuses a plain let group for mutual recursion"
    , `Quick
    , ex
        "4.20"
        Sicp_ch4_solutions.Sec_4_20.ex_4_20
        [ "false"
        ; "3628800"
        ; "rejected: type-invalid at line 1, column 58: the pinned type checker rejects \
           the source"
        ] )
  ; ( "4.21 rejects bare self-application and runs the three wrapped programs"
    , `Quick
    , ex
        "4.21"
        Sicp_ch4_solutions.Sec_4_21.ex_4_21
        [ "rejected: type-invalid"; "3628800"; "55"; "even odd" ] )
  ; ( "4.22 lowers every let before analysis"
    , `Quick
    , ex
        "4.22"
        Sicp_ch4_solutions.Sec_4_22.ex_4_22
        [ "7 (1 lets before, 0 after)"
        ; "5 (2 lets before, 0 after)"
        ; "610 (1 lets before, 0 after)"
        ] )
  ; ( "4.23 measures analysis work against runtime walking"
    , `Quick
    , ex
        "4.23"
        Sicp_ch4_solutions.Sec_4_23.ex_4_23
        [ "book, one expression: output aaa, 0 analysis steps, 3 runtime steps"
        ; "Alyssa, one expression: output aaa, 0 analysis steps, 6 runtime steps"
        ; "book, two expressions: output ababab, 1 analysis steps, 6 runtime steps"
        ; "Alyssa, two expressions: output ababab, 0 analysis steps, 12 runtime steps"
        ] )
  ]
;;

let parse_timing words =
  match words with
  | [ direct; analyzed; analysis; ratio ] ->
    let seconds line = Scanf.sscanf line "%s@: %f s" (fun _ n -> n) in
    [ seconds direct
    ; seconds analyzed
    ; seconds analysis
    ; Scanf.sscanf ratio "direct / analyzed: %f" Fun.id
    ]
  | _ -> Alcotest.fail "ex_4_24 answered the wrong number of lines"
;;

let ex_4_24_timing () =
  let numbers = parse_timing (unwrap (Sicp_ch4_solutions.Sec_4_24.ex_4_24 ())) in
  List.iter
    (fun n -> Alcotest.(check bool) "timings stay positive" true (n > 0.0))
    numbers
;;

let () =
  Alcotest.run
    "sec-4-1"
    [ "solutions", tests
    ; "benchmark", [ "4.24 reports four positive timings", `Quick, ex_4_24_timing ]
    ]
;;
