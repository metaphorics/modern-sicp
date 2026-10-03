(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the explicit-control evaluator and the section's
   reference solutions.  The base machine is pinned to the 5.4.4
   monitored-stack lesson on the recursive factorial, and every
   exercise's demonstration is pinned to the exact observable outcomes
   its solution produces: guest output, stack statistics, and fitted
   formulas. *)

module Eval = Sicp_ch5.Sec_5_4
module Eval_error = Sicp_common.Eval_error
module Sec_5_23 = Sicp_ch5_solutions.Sec_5_23
module Sec_5_24 = Sicp_ch5_solutions.Sec_5_24
module Sec_5_25 = Sicp_ch5_solutions.Sec_5_25
module Sec_5_26 = Sicp_ch5_solutions.Sec_5_26
module Sec_5_27 = Sicp_ch5_solutions.Sec_5_27
module Sec_5_28 = Sicp_ch5_solutions.Sec_5_28
module Sec_5_29 = Sicp_ch5_solutions.Sec_5_29
module Sec_5_30 = Sicp_ch5_solutions.Sec_5_30

let strings = Alcotest.(check (list string))

let lines name expected = function
  | Ok got -> strings name expected got
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

let recursive_factorial_5 =
  "let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n\n\
   let v = factorial 5\n"
;;

(* The base evaluator computes the book's session and its monitored
   stack answers the n = 5 row of the 5.27 table. *)
let base_session () =
  lines
    "the base evaluator prints factorial 5"
    [ "120" ]
    (Sec_5_23.session
       ~controller:Eval.base_controller
       "let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n\n\
        let () = print_endline (string_of_int (factorial 5))\n");
  match Sec_5_23.admit recursive_factorial_5 with
  | Error e -> Alcotest.fail (Eval_error.to_string e)
  | Ok program ->
    (match Eval.stack_statistics_after program with
     | Ok stats ->
       Alcotest.(check (pair int int)) "factorial 5 pushes and depth" (105, 18) stats
     | Error e -> Alcotest.fail (Eval_error.to_string e))
;;

let classify_answers = [ "zero"; "one"; "many"; "no"; "14"; "6" ]

(* 5.23: the derived forms answer what the basic forms answer, and the
   derivation costs more than the base ev-match. *)
let derived_expressions () =
  lines
    "5.23 derived expressions"
    (classify_answers
     @ [ "classify 7 through cond->if: pushes = 26, instructions = 368"
       ; "classify 7 through ev-match: pushes = 8, instructions = 127"
       ])
    (Sec_5_23.ex_5_23 ())
;;

(* 5.23: a match with a constructor pattern is not the subset's cond; it
   still reaches the base ev-match and binds its payload. *)
let constructor_match_is_not_derived () =
  lines
    "5.23 constructor match"
    [ "3"; "0" ]
    (Sec_5_23.run
       "let get o = match o with None -> 0 | Some x -> x\n\
        let () = print_endline (string_of_int (get (Some 3)))\n\
        let () = print_endline (string_of_int (get None))\n")
;;

(* 5.24: the clause loop answers what the derived form answers, at the
   base cost instead of the derivation's. *)
let cond_basic_form () =
  lines
    "5.24 cond as a basic form"
    (classify_answers
     @ [ "classify 7 through ev-cond: pushes = 8, instructions = 144"
       ; "classify 7 through cond->if: pushes = 26, instructions = 368"
       ])
    (Sec_5_24.ex_5_24 ())
;;

(* 5.24a: the chain loops short-circuit past a division by zero in
   both directions and save [continue] once per chain. *)
let and_or_basic_forms () =
  lines
    "5.24a && and || chains"
    [ "true"
    ; "false"
    ; "true"
    ; "false"
    ; "true"
    ; "true"
    ; "false"
    ; "four-operand && through the chain loop: pushes = 27, instructions = 322"
    ; "four-operand && through the base ev-and: pushes = 29, instructions = 293"
    ]
    (Sec_5_24.ex_5_24a ())
;;

(* 5.25: normal order keeps the strict answer, never evaluates an unused
   operand, and forces a thunk once -- unless memoization is disabled,
   when it is forced once per reference. *)
let lazy_evaluator () =
  lines
    "5.25 lazy evaluator"
    [ "120"
    ; "42"
    ; "strict evaluator: error: division by zero"
    ; "1 1"
    ; "1"
    ; "without memoization:"
    ; "1 2"
    ; "2"
    ]
    (Sec_5_25.ex_5_25 ())
;;

(* 5.26: the iterative factorial's maximum depth is 8 whatever n, and
   the pushes fit 26n + 25 on every measured point. *)
let iterative_factorial_stack () =
  lines
    "5.26 iterative factorial measurements"
    [ "iterative factorial n=1: total-pushes = 51 maximum-depth = 8"
    ; "iterative factorial n=2: total-pushes = 77 maximum-depth = 8"
    ; "iterative factorial n=3: total-pushes = 103 maximum-depth = 8"
    ; "iterative factorial n=4: total-pushes = 129 maximum-depth = 8"
    ; "iterative factorial n=5: total-pushes = 155 maximum-depth = 8"
    ; "iterative factorial n=6: total-pushes = 181 maximum-depth = 8"
    ; "maximum depth: 8, independent of n = true"
    ; "total pushes = 26n + 25, holds on every measured n: true"
    ]
    (Sec_5_26.ex_5_26 ())
;;

(* 5.27: the recursive factorial's depth is 3n + 3 and its pushes are
   23n - 10. *)
let recursive_factorial_stack () =
  lines
    "5.27 recursive factorial measurements"
    [ "recursive factorial n=1: total-pushes = 13 maximum-depth = 6"
    ; "recursive factorial n=2: total-pushes = 36 maximum-depth = 9"
    ; "recursive factorial n=3: total-pushes = 59 maximum-depth = 12"
    ; "recursive factorial n=4: total-pushes = 82 maximum-depth = 15"
    ; "recursive factorial n=5: total-pushes = 105 maximum-depth = 18"
    ; "recursive factorial n=6: total-pushes = 128 maximum-depth = 21"
    ; "maximum depth = 3n + 3, holds on every measured n: true"
    ; "total pushes = 23n - 10, holds on every measured n: true"
    ]
    (Sec_5_27.ex_5_27 ())
;;

(* 5.28: without tail calls both factorials need space linear in n. *)
let non_tail_recursive_stack () =
  lines
    "5.28 non-tail-recursive measurements"
    [ "non-tail iterative factorial n=1: total-pushes = 54 maximum-depth = 10"
    ; "non-tail iterative factorial n=2: total-pushes = 81 maximum-depth = 11"
    ; "non-tail iterative factorial n=3: total-pushes = 108 maximum-depth = 12"
    ; "non-tail iterative factorial n=4: total-pushes = 135 maximum-depth = 13"
    ; "non-tail iterative factorial n=5: total-pushes = 162 maximum-depth = 14"
    ; "non-tail recursive factorial n=1: total-pushes = 14 maximum-depth = 7"
    ; "non-tail recursive factorial n=2: total-pushes = 38 maximum-depth = 11"
    ; "non-tail recursive factorial n=3: total-pushes = 62 maximum-depth = 15"
    ; "non-tail recursive factorial n=4: total-pushes = 86 maximum-depth = 19"
    ; "non-tail recursive factorial n=5: total-pushes = 110 maximum-depth = 23"
    ; "iterative maximum depth = n + 9, holds on every measured n: true"
    ; "recursive maximum depth = 4n + 3, holds on every measured n: true"
    ; "iterative maximum depth now grows with n: true"
    ]
    (Sec_5_28.ex_5_28 ())
;;

(* 5.29: the depth grows by 3 per n, the pushes obey
   S(n) = S(n-1) + S(n-2) + 28, and S(n) = 41 * Fib(n+1) - 28. *)
let fib_stack () =
  lines
    "5.29 fib measurements"
    [ "fib n=2: total-pushes = 54 maximum-depth = 9"
    ; "fib n=3: total-pushes = 95 maximum-depth = 12"
    ; "fib n=4: total-pushes = 177 maximum-depth = 15"
    ; "fib n=5: total-pushes = 300 maximum-depth = 18"
    ; "fib n=6: total-pushes = 505 maximum-depth = 21"
    ; "fib n=7: total-pushes = 833 maximum-depth = 24"
    ; "fib n=8: total-pushes = 1366 maximum-depth = 27"
    ; "fib n=9: total-pushes = 2227 maximum-depth = 30"
    ; "maximum depth = 3n + 3 (every step 3), linear: true"
    ; "S(n) = S(n-1) + S(n-2) + 28, k constant: true"
    ; "S(n) = 41 * Fib(n+1) - 28, holds on every measured n: true"
    ]
    (Sec_5_29.ex_5_29 ())
;;

(* 5.30: every runtime failure, a callback's included, is reported and
   ends only its own program; the failed call leaves its saved words on
   the stack; the base evaluator stops instead; and admission rejects
   what part (a) would catch at run time. *)
let error_signaling () =
  match Sec_5_30.ex_5_30 () with
  | Error e -> Alcotest.fail (Eval_error.to_string e)
  | Ok [] -> Alcotest.fail "5.30 answered no lines"
  | Ok (bounds :: rest) ->
    Alcotest.(check bool)
      "5.30 an out-of-bounds Array.get is a caught failure"
      true
      (String.starts_with ~prefix:"error: bounds error" bounds);
    strings
      "5.30 the remaining interactions"
      [ "error: division by zero"
      ; "error: division by zero"
      ; "error: division by zero"
      ; "error: division by zero"
      ; "120"
      ; "saved frames left by the failed interaction: 6"
      ; "base evaluator stops: division by zero"
      ; "rejected before evaluation: type-invalid"
      ; "rejected before evaluation: type-invalid"
      ]
      rest
;;

let () =
  Alcotest.run
    "sicp section 5.4"
    [ "evaluator", [ Alcotest.test_case "base session" `Quick base_session ]
    ; ( "exercises"
      , [ Alcotest.test_case "5.23 derived expressions" `Quick derived_expressions
        ; Alcotest.test_case
            "5.23 constructor match reaches ev-match"
            `Quick
            constructor_match_is_not_derived
        ; Alcotest.test_case "5.24 cond as a basic form" `Quick cond_basic_form
        ; Alcotest.test_case "5.24a && and || chains" `Quick and_or_basic_forms
        ; Alcotest.test_case "5.25 lazy evaluator" `Quick lazy_evaluator
        ; Alcotest.test_case
            "5.26 iterative factorial stack"
            `Quick
            iterative_factorial_stack
        ; Alcotest.test_case
            "5.27 recursive factorial stack"
            `Quick
            recursive_factorial_stack
        ; Alcotest.test_case
            "5.28 non-tail-recursive stack"
            `Quick
            non_tail_recursive_stack
        ; Alcotest.test_case "5.29 fib stack formulas" `Quick fib_stack
        ; Alcotest.test_case "5.30 error signaling" `Quick error_signaling
        ] )
    ]
;;
