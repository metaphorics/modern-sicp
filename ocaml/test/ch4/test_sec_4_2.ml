(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 4.2. Every exercise's demonstration is pinned to the exact
   observable outcomes the solutions produce; the substrate lazy
   evaluator's own contract is pinned too, so a solution that leans on a
   broken clause cannot pass. *)

let check_strings = Alcotest.(check (list string))

let show = function
  | Ok v -> Sicp_common.Value.to_string v
  | Error e -> "Error: " ^ Sicp_common.Eval_error.to_string e
;;

let substrate_try () =
  let env = Sicp_ch4.Sec_4_2.the_global_environment () in
  let (_ : (Sicp_common.Value.t, Sicp_common.Eval_error.t) result) =
    Sicp_ch4.Sec_4_2.run env "(define (try a b) (if (= a 0) 1 b))"
  in
  Alcotest.(check string)
    "the lazy evaluator answers the try expression"
    "1"
    (show (Sicp_ch4.Sec_4_2.run env "(try 0 (/ 1 0))"))
;;

let ex_4_25_unless_under_orders () =
  check_strings
    "unless works lazily and dies eagerly"
    [ "ok"
    ; "ok"
    ; "120"
    ; "42"
    ; "ok"
    ; "Error: division by zero"
    ; "ok"
    ; "ok"
    ; "Error: the strict evaluation is still descending after 200 applications"
    ]
    (Sicp_ch4_solutions.Sec_4_25.ex_4_25 ())
;;

let ex_4_26_special_form_versus_procedure () =
  check_strings
    "unless as syntax and as a lazy procedure"
    [ "42"; "ok"; "42"; "(42 7)"; "Error: unbound variable unless" ]
    (Sicp_ch4_solutions.Sec_4_26.ex_4_26 ())
;;

let ex_4_27_lazy_id_with_set () =
  check_strings
    "the interaction's missing values and the memoized re-forcing"
    [ "ok"; "ok"; "ok"; "1"; "10"; "2"; "10"; "2" ]
    (Sicp_ch4_solutions.Sec_4_27.ex_4_27 ())
;;

let ex_4_28_forcing_the_operator () =
  check_strings
    "the operator thunk is forced before apply dispatches"
    [ "ok"; "ok"; "5"; "Error: invalid form: the primitive thunk is not installed" ]
    (Sicp_ch4_solutions.Sec_4_28.ex_4_28 ())
;;

let ex_4_29_memoization_difference () =
  check_strings
    "count advances once with memoization and more without"
    [ "100"; "1"; "1000"; "2"; "100"; "2"; "1000"; "5" ]
    (Sicp_ch4_solutions.Sec_4_29.ex_4_29 ())
;;

let ex_4_29a_memoization_toggle_counts () =
  check_strings
    "two forcings either way; memoization halves the computations"
    [ "100"
    ; "creations=2 forcings=2 computations=2"
    ; "100"
    ; "creations=3 forcings=2 computations=4"
    ]
    (Sicp_ch4_solutions.Sec_4_29.ex_4_29a ())
;;

let ex_4_30_forcing_in_eval_sequence () =
  check_strings
    "for-each works either way; p2 differs exactly as Cy predicts"
    [ "ok"
    ; "done"
    ; "3"
    ; "ok"
    ; "done"
    ; "3"
    ; "ok"
    ; "(1 2)"
    ; "ok"
    ; "1"
    ; "ok"
    ; "(1 2)"
    ; "ok"
    ; "(1 2)"
    ]
    (Sicp_ch4_solutions.Sec_4_30.ex_4_30 ())
;;

let ex_4_31_lazy_parameter_declarations () =
  check_strings
    "strict, lazy, and lazy-memo bind per declaration"
    [ "ok"
    ; "taken"
    ; "ok"
    ; "(1 5 5 4 30 30)"
    ; "5"
    ; "ok"
    ; "(10 10)"
    ; "2"
    ; "ok"
    ; "(10 10)"
    ; "3"
    ]
    (Sicp_ch4_solutions.Sec_4_31.ex_4_31 ())
;;

let ex_4_32_streams_versus_lazy_lists () =
  check_strings
    "skipped elements stay unforced under the lazy lists"
    [ "42"; "Error: division by zero"; "7" ]
    (Sicp_ch4_solutions.Sec_4_32.ex_4_32 ())
;;

let ex_4_33_quote_produces_lazy_lists () =
  check_strings
    "the quoted list becomes the object language's own lazy structure"
    [ "Error: not applicable: (a b c) is not a procedure"; "a"; "b"; "d" ]
    (Sicp_ch4_solutions.Sec_4_33.ex_4_33 ())
;;

let ex_4_34_printing_lazy_pairs () =
  check_strings
    "lazy pairs print a forced prefix, infinite lists an ellipsis"
    [ "(1 2)"; "((1) 2)"; "(1 1 1 1 1 1 1 1 1 1 ...)"; "1" ]
    (Sicp_ch4_solutions.Sec_4_34.ex_4_34 ())
;;

let () =
  let test name run = Alcotest.test_case name `Quick run in
  Alcotest.run
    "sec_4_2"
    [ "substrate", [ test "the lazy evaluator answers the try expression" substrate_try ]
    ; ( "exercises"
      , [ test "4.25 unless under both orders" ex_4_25_unless_under_orders
        ; test "4.26 special form versus procedure" ex_4_26_special_form_versus_procedure
        ; test "4.27 lazy id with set!" ex_4_27_lazy_id_with_set
        ; test "4.28 forcing the operator" ex_4_28_forcing_the_operator
        ; test "4.29 memoization difference" ex_4_29_memoization_difference
        ; test "4.29a memoization toggle counts" ex_4_29a_memoization_toggle_counts
        ; test "4.30 forcing in eval-sequence" ex_4_30_forcing_in_eval_sequence
        ; test "4.31 lazy parameter declarations" ex_4_31_lazy_parameter_declarations
        ; test "4.32 streams versus lazy lists" ex_4_32_streams_versus_lazy_lists
        ; test "4.33 quote produces lazy lists" ex_4_33_quote_produces_lazy_lists
        ; test "4.34 printing lazy pairs" ex_4_34_printing_lazy_pairs
        ] )
    ]
;;
