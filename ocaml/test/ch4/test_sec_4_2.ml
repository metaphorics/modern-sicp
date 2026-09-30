(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 4.2.  Every exercise's demonstration is pinned to the exact
   observable outcomes the solutions produce; the lazy experiment's own
   contract is pinned too, so a solution that leans on a broken clause
   cannot pass. *)

let check_strings = Alcotest.(check (list string))

let counts allocations forces recomputations memo_hits =
  Printf.sprintf
    "thunk-allocations: %d\n\
     thunk-forces: %d\n\
     thunk-recomputations: %d\n\
     thunk-memo-hits: %d\n"
    allocations
    forces
    recomputations
    memo_hits
;;

let substrate_try () =
  Alcotest.(check string)
    "the lazy experiment answers the try expression"
    ("1\n" ^ counts 2 1 1 0)
    (Sicp_ch4.Sec_4_1.transcript
       ~experiment:Sicp_common.Check.Lazy
       Sicp_ch4.Sec_4_2.run
       "let try_ a b = if a = 0 then 1 else b\n\
        let () = print_int (try_ 0 (1 / 0)); print_newline ()")
;;

let ex_4_25_unless_under_orders () =
  check_strings
    "unless works lazily and dies eagerly"
    [ "120\n" ^ counts 20 23 15 8
    ; "42\n" ^ counts 3 2 2 0
    ; "error: the strict evaluation is still descending after 200 applications"
    ; "error: division by zero"
    ]
    (Sicp_ch4_solutions.Sec_4_25.ex_4_25 ())
;;

let ex_4_26_special_form_versus_procedure () =
  check_strings
    "unless as syntax and as a lazy procedure"
    [ "42\n"
    ; "42\n" ^ counts 4 3 3 0
    ; "42 7 \n" ^ counts 25 26 20 6
    ; "error: unbound variable unless"
    ]
    (Sicp_ch4_solutions.Sec_4_26.ex_4_26 ())
;;

let ex_4_27_lazy_id_with_counter () =
  check_strings
    "the interaction's missing values and the memoized re-forcing"
    [ "1"
    ; "10"
    ; "2"
    ; "10"
    ; "2"
    ; "thunk-allocations: 2"
    ; "thunk-forces: 3"
    ; "thunk-recomputations: 2"
    ; "thunk-memo-hits: 1"
    ]
    (Sicp_ch4_solutions.Sec_4_27.ex_4_27 ())
;;

let ex_4_28_forcing_the_operator () =
  check_strings
    "the operator thunk is forced before apply dispatches"
    [ "5\n" ^ counts 3 3 3 0; "error: not applicable: thunk delayed is not a procedure" ]
    (Sicp_ch4_solutions.Sec_4_28.ex_4_28 ())
;;

let ex_4_29_memoization_difference () =
  check_strings
    "count advances once with memoization and more without"
    [ "100\n1\n1000\n2\n" ^ counts 4 7 4 3; "100\n2\n1000\n5\n" ^ counts 7 10 10 0 ]
    (Sicp_ch4_solutions.Sec_4_29.ex_4_29 ())
;;

let ex_4_29a_memoization_toggle_counts () =
  check_strings
    "memoization turns a recomputation into a memo hit"
    [ "100"
    ; "allocations=2 forces=3 recomputations=2 memo_hits=1"
    ; "100"
    ; "allocations=3 forces=4 recomputations=4 memo_hits=0"
    ]
    (Sicp_ch4_solutions.Sec_4_29.ex_4_29a ())
;;

let ex_4_30_forcing_in_sequence () =
  let for_each = "57\n321\n88\ndone\n" ^ counts 11 12 10 2 in
  check_strings
    "for_each works either way; p2 differs exactly as Cy predicts"
    [ for_each
    ; for_each
    ; "1 2 \n1 \n" ^ counts 9 10 8 2
    ; "1 2 \n1 2 \n" ^ counts 10 14 10 4
    ]
    (Sicp_ch4_solutions.Sec_4_30.ex_4_30 ())
;;

let ex_4_31_lazy_parameter_declarations () =
  check_strings
    "strict, lazy, and lazy-memo bind per declaration"
    [ "taken\n"
    ; "error: division by zero"
    ; "1 5 5 4 30 30 \n5\n"
    ; "10 10 \n2\n10 10 \n3\n"
    ]
    (Sicp_ch4_solutions.Sec_4_31.ex_4_31 ())
;;

let ex_4_32_streams_versus_lazy_lists () =
  check_strings
    "skipped elements stay unforced under the lazy lists"
    [ "42\n" ^ counts 7 4 4 0; "error: division by zero"; "7\n" ^ counts 4 2 2 0 ]
    (Sicp_ch4_solutions.Sec_4_32.ex_4_32 ())
;;

let ex_4_33_literal_lists_are_lazy () =
  check_strings
    "the literal list becomes the program's own lazy structure"
    [ "error: division by zero"
    ; "42\n" ^ counts 7 4 4 0
    ; "42\n" ^ counts 7 4 4 0
    ; "7\n" ^ counts 16 15 12 3
    ]
    (Sicp_ch4_solutions.Sec_4_33.ex_4_33 ())
;;

let ex_4_34_printing_lazy_pairs () =
  check_strings
    "lazy lists print a forced prefix, infinite lists an ellipsis"
    [ "[1; 2]"
    ; "[[1]; [2]]"
    ; "[1; 1; 1; 1; 1; 1; 1; 1; 1; 1; ...]"
    ; "1"
    ; "error: division by zero"
    ]
    (Sicp_ch4_solutions.Sec_4_34.ex_4_34 ())
;;

let () =
  let test name run = Alcotest.test_case name `Quick run in
  Alcotest.run
    "sec_4_2"
    [ "substrate", [ test "the lazy experiment answers the try expression" substrate_try ]
    ; ( "exercises"
      , [ test "4.25 unless under both orders" ex_4_25_unless_under_orders
        ; test "4.26 special form versus procedure" ex_4_26_special_form_versus_procedure
        ; test "4.27 lazy id with a counter" ex_4_27_lazy_id_with_counter
        ; test "4.28 forcing the operator" ex_4_28_forcing_the_operator
        ; test "4.29 memoization difference" ex_4_29_memoization_difference
        ; test "4.29a memoization toggle counts" ex_4_29a_memoization_toggle_counts
        ; test "4.30 forcing in the sequence clause" ex_4_30_forcing_in_sequence
        ; test "4.31 lazy parameter declarations" ex_4_31_lazy_parameter_declarations
        ; test "4.32 streams versus lazy lists" ex_4_32_streams_versus_lazy_lists
        ; test "4.33 literal lists are lazy" ex_4_33_literal_lists_are_lazy
        ; test "4.34 printing lazy pairs" ex_4_34_printing_lazy_pairs
        ] )
    ]
;;
