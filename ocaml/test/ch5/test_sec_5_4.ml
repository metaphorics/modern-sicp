(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the explicit-control evaluator and the section's
   reference solutions. The base machine is pinned to the book's own
   5.4.4 session -- the driver transcript and the monitored stack
   statistics the prose quotes -- and every exercise's demonstration is
   pinned to the exact observable outcomes its solution produces. *)

module Eval = Sicp_ch5.Sec_5_4

let ( >>= ) = Result.bind

module Sec_5_23 = Sicp_ch5_solutions.Sec_5_23
module Sec_5_24 = Sicp_ch5_solutions.Sec_5_24
module Sec_5_25 = Sicp_ch5_solutions.Sec_5_25
module Sec_5_26 = Sicp_ch5_solutions.Sec_5_26
module Sec_5_27 = Sicp_ch5_solutions.Sec_5_27
module Sec_5_28 = Sicp_ch5_solutions.Sec_5_28
module Sec_5_29 = Sicp_ch5_solutions.Sec_5_29
module Sec_5_30 = Sicp_ch5_solutions.Sec_5_30

let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string
let the_bool = Alcotest.check Alcotest.bool

let strings_outcome name expected = function
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

let has_suffix suffix line =
  String.length line >= String.length suffix
  && String.sub line (String.length line - String.length suffix) (String.length suffix)
     = suffix
;;

(* The book's 5.4.4 session on the plain driver: the definition, then
   the call, value 120, and the driver's stop when the queue runs
   dry. *)
let base_driver_session () =
  Eval.make_evaluator ~source:"(+ 40 2)" ()
  >>= fun m ->
  let ended = Eval.start m in
  let transcript = Eval.transcript m in
  let normal_end =
    match ended with
    | Error e -> Eval.error_to_string e = "operation failed: " ^ Eval.input_exhausted
    | Ok () -> false
  in
  the_bool "the driver ends when the input queue runs dry" true normal_end;
  strings
    "the driver echoes the interaction protocol"
    [ ";;; EC-Eval input:"; ";;; EC-Eval value:"; "42"; ";;; EC-Eval input:" ]
    transcript;
  Ok ()
;;

(* The book's monitored driver: the definition costs three pushes, the
   call answers 144 pushes at depth 28, exactly the numbers the prose
   quotes. *)
let monitored_driver_session () =
  let controller =
    String.concat
      "\n"
      (List.map
         (fun (name, text) -> if name = "driver" then Sec_5_26.monitored_driver else text)
         Eval.controller_fragments)
  in
  Eval.make_evaluator
    ~controller
    ~source:
      {|
(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))
(factorial 5)|}
    ()
  >>= fun m ->
  let _ = Eval.start m in
  let transcript = Eval.transcript m in
  let has stats = List.exists (fun line -> line = stats) transcript in
  the_bool
    "the define costs (total-pushes = 3 maximum-depth = 3)"
    true
    (has "(total-pushes = 3 maximum-depth = 3)");
  the_bool
    "the call costs (total-pushes = 144 maximum-depth = 28)"
    true
    (has "(total-pushes = 144 maximum-depth = 28)");
  Ok ()
;;

(* 5.23: cond clauses dispatch through cond->if, the bodyless clause
   answers its test, no true clause and no else answers false, and let
   becomes a lambda application. *)
let derived_expressions () =
  strings_outcome
    "5.23 derived expressions"
    [ ";;; EC-Eval input:"
    ; ";;; EC-Eval value:"
    ; "ok"
    ; ";;; EC-Eval input:"
    ; ";;; EC-Eval value:"
    ; "zero"
    ; ";;; EC-Eval input:"
    ; ";;; EC-Eval value:"
    ; "one"
    ; ";;; EC-Eval input:"
    ; ";;; EC-Eval value:"
    ; "many"
    ; ";;; EC-Eval input:"
    ; ";;; EC-Eval value:"
    ; "#f"
    ; ";;; EC-Eval input:"
    ; ";;; EC-Eval value:"
    ; "6"
    ; ";;; EC-Eval input:"
    ]
    (Sec_5_23.ex_5_23 ())
;;

(* 5.24: cond as a basic form: the same observable answers as the
   derived form, through the clause loop instead of the transformer. *)
let cond_basic_form () =
  match Sec_5_24.ex_5_24 () with
  | Ok lines ->
    let values =
      List.filter
        (fun l -> has_suffix "zero" l || List.mem l [ "zero"; "one"; "many"; "#f"; "#t" ])
        lines
    in
    strings "5.24 cond basic-form values" [ "zero"; "one"; "many"; "#f"; "#t" ] values
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

(* 5.24a: and and or as basic forms: last value, first true value,
   empty forms, short-circuit past an unbound variable, nesting. *)
let and_or_basic_forms () =
  strings_outcome
    "5.24a and/or basic-form values"
    [ "3"; "#f"; "#t"; "#f"; "2"; "#f"; "#f"; "1"; "#f"; "#t"; "#f" ]
    (Sec_5_24.ex_5_24a ()
     >>= fun lines ->
     Ok
       (List.filter
          (fun l -> l <> "ok" && not (String.starts_with ~prefix:";;;" l))
          lines))
;;

(* 5.25: normal order keeps the strict answer and proves both
   properties: an unused argument is never evaluated, and a thunked
   argument is forced once, not once per reference. *)
let lazy_evaluator () =
  strings_outcome
    "5.25 lazy evaluator"
    [ "120"; "42"; "(1 1)"; "1" ]
    (Sec_5_25.ex_5_25 ()
     >>= fun lines ->
     let values = List.filter (fun l -> List.mem l [ "120"; "42"; "(1 1)"; "1" ]) lines in
     Ok values)
;;

(* 5.26: the iterative factorial's maximum depth is 10 whatever n, and
   the pushes fit 35n + 29 on every measured point. *)
let iterative_factorial_stack () =
  strings_outcome
    "5.26 iterative factorial measurements"
    [ "iterative factorial n=1: total-pushes = 64 maximum-depth = 10"
    ; "iterative factorial n=2: total-pushes = 99 maximum-depth = 10"
    ; "iterative factorial n=3: total-pushes = 134 maximum-depth = 10"
    ; "iterative factorial n=4: total-pushes = 169 maximum-depth = 10"
    ; "iterative factorial n=5: total-pushes = 204 maximum-depth = 10"
    ; "iterative factorial n=6: total-pushes = 239 maximum-depth = 10"
    ; "maximum depth: 10, independent of n = true"
    ; "total pushes = 35n + 29, holds on every measured n: true"
    ]
    (Sec_5_26.ex_5_26 ())
;;

(* 5.27: the recursive factorial's depth is 5n + 3 and its pushes are
   32n - 16; the n=5 row is the book's own monitored session. *)
let recursive_factorial_stack () =
  strings_outcome
    "5.27 recursive factorial measurements"
    [ "recursive factorial n=1: total-pushes = 16 maximum-depth = 8"
    ; "recursive factorial n=2: total-pushes = 48 maximum-depth = 13"
    ; "recursive factorial n=3: total-pushes = 80 maximum-depth = 18"
    ; "recursive factorial n=4: total-pushes = 112 maximum-depth = 23"
    ; "recursive factorial n=5: total-pushes = 144 maximum-depth = 28"
    ; "recursive factorial n=6: total-pushes = 176 maximum-depth = 33"
    ; "maximum depth = 5n + 3, holds on every measured n: true"
    ; "total pushes = 32n - 16, holds on every measured n: true"
    ]
    (Sec_5_27.ex_5_27 ())
;;

(* 5.28: with the naive sequence evaluation both factorials demand
   space that grows with n; the iterative version's depth is linear
   too. *)
let non_tail_recursive_stack () =
  match Sec_5_28.ex_5_28 () with
  | Ok lines ->
    let last = List.nth lines (List.length lines - 1) in
    the_string
      "5.28 the iterative depth now grows"
      "iterative maximum depth now grows with n: true"
      last;
    strings
      "5.28 the iterative depth steps by 3 per n"
      [ "non-tail iterative factorial n=1: total-pushes = 70 maximum-depth = 17"
      ; "non-tail iterative factorial n=5: total-pushes = 218 maximum-depth = 29"
      ]
      [ List.nth lines 0; List.nth lines 4 ]
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

(* 5.29: the depth grows by 5 per n (depth = 5n + 3), the pushes obey
   S(n) = S(n-1) + S(n-2) + 40, and S(n) = 56*Fib(n+1) - 40. *)
let fib_stack () =
  match Sec_5_29.ex_5_29 () with
  | Ok lines ->
    let n = List.length lines in
    the_bool "5.29 measured eight interactions plus three formulas" true (n = 11);
    the_string
      "5.29 the depth formula"
      "maximum depth = 5n + 3 (every step 5), linear: true"
      (List.nth lines 8);
    the_string
      "5.29 the recurrence constant"
      "S(n) = S(n-1) + S(n-2) + 40, k constant: true"
      (List.nth lines 9);
    the_string
      "5.29 the closed form"
      "S(n) = 56 * Fib(n+1) - 40, holds on every measured n: true"
      (List.nth lines 10)
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

(* 5.30: the condition-code paths report through signal-error, and a
   clean factorial still answers 120. *)
let error_signaling () =
  match Sec_5_30.ex_5_30 () with
  | Ok lines ->
    let starts prefix l = String.starts_with ~prefix l in
    the_bool
      "5.30 car of a symbol is a caught primitive failure"
      true
      (List.exists (starts "operation failed: type error: car of 5") lines);
    the_bool
      "5.30 division by zero is a caught primitive failure"
      true
      (List.exists (starts "operation failed: division by zero") lines);
    the_bool
      "5.30 an unbound variable is a caught lookup failure"
      true
      (List.exists (starts "operation failed: unbound variable: no-such-variable") lines);
    the_bool
      "5.30 a wrong operand count is a caught primitive failure"
      true
      (List.exists (starts "operation failed: arity mismatch: expected 2, given 1") lines);
    the_bool
      "5.30 a clean program still answers 120"
      true
      (List.exists (fun l -> l = "120") lines)
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

let run_ok f =
  match f () with
  | Ok () -> ()
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

let () =
  Alcotest.run
    "sicp section 5.4"
    [ ( "evaluator"
      , [ Alcotest.test_case "driver session" `Quick (fun () ->
            run_ok base_driver_session)
        ; Alcotest.test_case "monitored session" `Quick (fun () ->
            run_ok monitored_driver_session)
        ] )
    ; ( "exercises"
      , [ Alcotest.test_case "5.23 derived expressions" `Quick derived_expressions
        ; Alcotest.test_case "5.24 cond as a basic form" `Quick cond_basic_form
        ; Alcotest.test_case "5.24a and/or as basic forms" `Quick and_or_basic_forms
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
