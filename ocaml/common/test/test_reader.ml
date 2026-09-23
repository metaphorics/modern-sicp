(* SPDX-License-Identifier: GPL-3.0-only *)

module A = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module R = Sicp_common.Reader

let read_error = Alcotest.testable R.pp ( = )

let expr_of = function
  | Ok e -> e
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

let ok_view detail text expected =
  match R.read text with
  | Ok e ->
    if A.view e <> A.view expected
    then Alcotest.fail (detail ^ ": read a different form than expected")
    else ()
  | Error e -> Alcotest.fail (detail ^ ": " ^ R.to_string e)
;;

let is_error detail text expected =
  match R.read text with
  | Error e -> Alcotest.check read_error detail expected e
  | Ok _ -> Alcotest.fail (detail ^ ": expected a reader error")
;;

let list1 d = A.DPair (d, A.DNil)

let rec list_of = function
  | [] -> A.DNil
  | d :: rest -> A.DPair (d, list_of rest)
;;

let reads_atoms () =
  ok_view "zero" "0" (A.int 0);
  ok_view "negative int" "-42" (A.int (-42));
  ok_view "largest in-range int" "4611686018427387903" (A.int 4611686018427387903);
  ok_view "float" "2.5" (A.float 2.5);
  ok_view "negative float" "-0.5" (A.float (-0.5));
  ok_view "float with exponent" "1.0e22" (A.float 1e22);
  ok_view "float with signed exponent" "1.5e-3" (A.float 0.0015);
  ok_view "true" "#t" (A.bool true);
  ok_view "false" "#f" (A.bool false);
  ok_view "string" "\"a\\\"b\\\\c\"" (A.string "a\"b\\c");
  ok_view "symbol with subset characters" "set-car!?" (A.variable "set-car!?");
  ok_view "plus alone is a symbol" "+" (A.variable "+");
  ok_view "minus alone is a symbol" "-" (A.variable "-")
;;

let reads_numbers_strictly () =
  let p line col = { R.line; R.column = col } in
  is_error "1e5 has no decimal point" "1e5" (R.Bad_number ("1e5", p 1 0));
  is_error "3. has no digits after the dot" "3." (R.Bad_number ("3.", p 1 0));
  is_error "exponent without digits" "1.5e" (R.Bad_number ("1.5e", p 1 0));
  is_error
    "int beyond 63 bits"
    "4611686018427387904"
    (R.Bad_number ("4611686018427387904", p 1 0));
  ok_view "0.5 without leading digits is a symbol" ".5" (A.variable ".5")
;;

let reads_quote_and_pairs () =
  ok_view "quote sugar" "'a" (A.quote (A.DSymbol "a"));
  ok_view
    "nested quote"
    "''a"
    (A.quote (A.DPair (A.DSymbol "quote", list1 (A.DSymbol "a"))));
  ok_view
    "quoted list"
    "'(a b c)"
    (A.quote (list_of [ A.DSymbol "a"; A.DSymbol "b"; A.DSymbol "c" ]));
  ok_view
    "quoted dotted pair"
    "'(a . b)"
    (A.quote (A.DPair (A.DSymbol "a", A.DSymbol "b")));
  ok_view "quoted empty list" "'()" (A.quote A.DNil);
  ok_view
    "quoted datum inside a call"
    "(car '(a b))"
    (A.application
       (A.variable "car")
       [ A.quote (list_of [ A.DSymbol "a"; A.DSymbol "b" ]) ])
;;

let reads_calls () =
  ok_view "nullary call" "(newline)" (A.application (A.variable "newline") []);
  ok_view
    "call with operands"
    "(f x 1 \"s\")"
    (A.application (A.variable "f") [ A.variable "x"; A.int 1; A.string "s" ]);
  ok_view
    "higher-order application"
    "((lambda (x) x) 5)"
    (A.application (expr_of (A.lambda [ "x" ] [ A.variable "x" ])) [ A.int 5 ]);
  ok_view
    "operator in operator position"
    "((if c f g) x)"
    (A.application
       (A.if_ (A.variable "c") (A.variable "f") (Some (A.variable "g")))
       [ A.variable "x" ])
;;

let reads_definitions () =
  ok_view
    "variable define"
    "(define answer 42)"
    (A.definition (A.define_variable "answer" (A.int 42)));
  let sq_body = A.application (A.variable "*") [ A.variable "x"; A.variable "x" ] in
  ok_view
    "single-body function define"
    "(define (sq x) (* x x))"
    (A.definition (expr_of (A.define_function "sq" [ "x" ] [ sq_body ])));
  (* A two-body function define, as the corpus writes memq. *)
  ok_view
    "two-body function define"
    "(define (f x) (set! x 1) x)"
    (A.definition
       (expr_of (A.define_function "f" [ "x" ] [ A.set "x" (A.int 1); A.variable "x" ])))
;;

let reads_special_forms () =
  ok_view "if two-operand" "(if #t 1)" (A.if_ (A.bool true) (A.int 1) None);
  ok_view
    "if three-operand"
    "(if #f 1 2)"
    (A.if_ (A.bool false) (A.int 1) (Some (A.int 2)));
  ok_view "set!" "(set! x 5)" (A.set "x" (A.int 5));
  ok_view "and empty" "(and)" (A.and_ []);
  ok_view "or operands" "(or 1 2)" (A.or_ [ A.int 1; A.int 2 ]);
  ok_view "begin" "(begin 1 2)" (expr_of (A.sequence [ A.int 1; A.int 2 ]));
  ok_view
    "let"
    "(let ((x 1) (y 2)) (+ x y))"
    (expr_of
       (A.let_
          [ "x", A.int 1; "y", A.int 2 ]
          [ A.application (A.variable "+") [ A.variable "x"; A.variable "y" ] ]));
  ok_view "lambda nullary" "(lambda () 1)" (expr_of (A.lambda [] [ A.int 1 ]));
  ok_view
    "cond with test-only clause and else"
    "(cond (#f) (x 1) (else 2))"
    (expr_of
       (A.cond [ A.bool false, []; A.variable "x", [ A.int 1 ] ] (Some [ A.int 2 ])))
;;

let rejects_malformed_forms () =
  let p col = { R.line = 1; R.column = col } in
  is_error "empty input" "" (R.Unexpected_eof (p 0));
  is_error "only a comment" "; nothing here" (R.Unexpected_eof (p 14));
  is_error "open paren" "(a" (R.Unexpected_eof (p 2));
  is_error "stray close paren" ")" (R.Unexpected_char (')', p 0));
  is_error "hash junk" "#x" (R.Bad_literal ("#x", p 0));
  is_error "unterminated string" "\"abc" (R.Bad_string ("unterminated string", p 4));
  is_error "unknown escape" "\"a\\nb\"" (R.Bad_string ("unknown escape \\n", p 3));
  is_error "empty application at read time" "()" (R.Bad_form ("empty application", p 0));
  is_error "dot outside a list" "." (R.Bad_form ("`.` outside a list", p 0));
  is_error "dot at list start" "(. x)" (R.Bad_form ("`.` with no preceding element", p 0));
  is_error "dot with empty tail" "(a . )" (R.Bad_form ("`.` with an empty tail", p 5));
  is_error "dot with two tail elements" "(a . b c)" (R.Unexpected_char ('c', p 7));
  is_error "quote arity" "(quote)" (R.Bad_form ("quote: expects one datum", p 0));
  is_error
    "set! arity"
    "(set! x)"
    (R.Bad_form ("set!: expects (set! name expression)", p 0));
  is_error "if arity" "(if #t)" (R.Bad_form ("if: expects two or three operands", p 0));
  is_error "empty begin" "(begin)" (R.Bad_form ("begin: needs a nonempty body", p 0));
  is_error "empty cond" "(cond)" (R.Bad_form ("cond: needs clauses", p 0));
  is_error
    "else with empty body"
    "(cond (else))"
    (R.Bad_form ("cond: else clause needs a body", p 0));
  is_error
    "let without body"
    "(let ((x 1)))"
    (R.Bad_form ("let: expects (let ((name value) ...) body ...)", p 0));
  is_error
    "lambda without body"
    "(lambda (x))"
    (R.Bad_form ("lambda: expects (lambda (x ...) body ...)", p 0));
  is_error
    "bad binding shape"
    "(let (x) x)"
    (R.Bad_form ("let: binding is not (name value)", p 0));
  is_error
    "non-symbol parameter"
    "(lambda (1) 1)"
    (R.Bad_form ("lambda: expects a parameter list of symbols", p 0));
  is_error
    "define without operands"
    "(define)"
    (R.Bad_form
       ("define: expects (define name expression) or (define (name ...) body ...)", p 0));
  is_error "unexpected character" "$" (R.Unexpected_char ('$', p 0))
;;

let skips_comments () =
  ok_view "comment before form" "; header\n42" (A.int 42);
  ok_view "comment after form stays ignored" "1 ; tail" (A.int 1);
  let program = ";; SPDX-License-Identifier: GPL-3.0-only\n(define a 1)\na\n" in
  match R.read_program program with
  | Error e -> Alcotest.fail (R.to_string e)
  | Ok forms ->
    if List.length forms <> 2
    then Alcotest.fail "comment-only header line produced a form"
    else ()
;;

let read_program_reads_every_form () =
  match R.read_program "(define x 1)\n(+ x 2)\n'end\n" with
  | Error e -> Alcotest.fail (R.to_string e)
  | Ok forms ->
    (match List.map A.view forms with
     | [ A.Definition _; A.Application _; A.Quote _ ] -> ()
     | _ -> Alcotest.fail "read_program returned the wrong shapes");
    (match R.read_program "  ; nothing\n" with
     | Ok [] -> ()
     | Ok _ -> Alcotest.fail "expected an empty program"
     | Error e -> Alcotest.fail (R.to_string e))
;;

let () =
  Alcotest.run
    "sicp_common.reader"
    [ ( "reader"
      , Alcotest.
          [ test_case "reads atoms" `Quick reads_atoms
          ; test_case "reads numbers strictly" `Quick reads_numbers_strictly
          ; test_case "reads quote and pairs" `Quick reads_quote_and_pairs
          ; test_case "reads calls" `Quick reads_calls
          ; test_case "reads definitions" `Quick reads_definitions
          ; test_case "reads special forms" `Quick reads_special_forms
          ; test_case "rejects malformed forms" `Quick rejects_malformed_forms
          ; test_case "skips comments" `Quick skips_comments
          ; test_case "read_program reads every form" `Quick read_program_reads_every_form
          ] )
    ]
;;
