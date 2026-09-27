(* SPDX-License-Identifier: GPL-3.0-only *)
module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error

let check_view detail actual expected =
  if actual <> expected then Alcotest.fail (detail ^ ": view mismatch") else ()
;;

let self_evaluating () =
  check_view "int" (Ast.view (Ast.int 3)) (Ast.Int 3);
  check_view "float" (Ast.view (Ast.float 2.5)) (Ast.Float 2.5);
  check_view "bool" (Ast.view (Ast.bool true)) (Ast.Bool true);
  check_view "string" (Ast.view (Ast.string "hi")) (Ast.String "hi");
  check_view "variable" (Ast.view (Ast.variable "x")) (Ast.Variable "x")
;;

let compound_forms () =
  check_view
    "quote"
    (Ast.view (Ast.quote (Ast.DPair (Ast.DSymbol "a", Ast.DNil))))
    (Ast.Quote (Ast.DPair (Ast.DSymbol "a", Ast.DNil)));
  check_view "set!" (Ast.view (Ast.set "x" (Ast.int 1))) (Ast.Set ("x", Ast.int 1));
  check_view
    "if without alternative"
    (Ast.view (Ast.if_ (Ast.bool true) (Ast.int 1) None))
    (Ast.If (Ast.bool true, Ast.int 1, None));
  check_view
    "if with alternative"
    (Ast.view (Ast.if_ (Ast.bool true) (Ast.int 1) (Some (Ast.int 2))))
    (Ast.If (Ast.bool true, Ast.int 1, Some (Ast.int 2)));
  check_view "and" (Ast.view (Ast.and_ [])) (Ast.And []);
  check_view "or" (Ast.view (Ast.or_ [ Ast.variable "a" ])) (Ast.Or [ Ast.variable "a" ]);
  check_view
    "application"
    (Ast.view (Ast.application (Ast.variable "f") [ Ast.int 1 ]))
    (Ast.Application (Ast.variable "f", [ Ast.int 1 ]))
;;

let sequences_and_lets () =
  check_view
    "begin"
    (Ast.view
       (match Ast.sequence [ Ast.int 1; Ast.int 2 ] with
        | Ok e -> e
        | Error e -> Alcotest.fail (Eval_error.to_string e)))
    (Ast.Sequence [ Ast.int 1; Ast.int 2 ]);
  check_view
    "let"
    (Ast.view
       (match Ast.let_ [ "x", Ast.int 1 ] [ Ast.variable "x" ] with
        | Ok e -> e
        | Error e -> Alcotest.fail (Eval_error.to_string e)))
    (Ast.Let ([ "x", Ast.int 1 ], [ Ast.variable "x" ]));
  check_view
    "lambda"
    (Ast.view
       (match Ast.lambda [ "x" ] [ Ast.variable "x" ] with
        | Ok e -> e
        | Error e -> Alcotest.fail (Eval_error.to_string e)))
    (Ast.Lambda ([ "x" ], [ Ast.variable "x" ]))
;;

let definitions () =
  let open Ast in
  let variable_definition =
    match view (Ast.definition (define_variable "x" (Ast.int 1))) with
    | Definition d -> Ast.view_definition d
    | _ -> Alcotest.fail "expected a definition, got a different view"
  in
  Alcotest.check
    Alcotest.bool
    "define_variable view"
    true
    (match variable_definition with
     | Define_variable ("x", e) -> Ast.view e = Int 1
     | _ -> false);
  let function_definition =
    match
      view
        (match
           define_function
             "sq"
             [ "x" ]
             [ Ast.application (Ast.variable "*") [ Ast.variable "x"; Ast.variable "x" ] ]
         with
         | Ok d -> Ast.definition d
         | Error e -> Alcotest.fail (Eval_error.to_string e))
    with
    | Definition d -> Ast.view_definition d
    | _ -> Alcotest.fail "expected a definition"
  in
  Alcotest.check
    Alcotest.bool
    "define_function view"
    true
    (match function_definition with
     | Define_function { name; parameters; body } ->
       String.equal name "sq" && parameters = [ "x" ] && List.length body = 1
     | _ -> false)
;;

let rejects_malformed_construction () =
  let check_invalid detail = function
    | Error (Eval_error.Invalid_form _) -> ()
    | Error e -> Alcotest.fail (detail ^ ": wrong error " ^ Eval_error.to_string e)
    | Ok _ -> Alcotest.fail (detail ^ ": expected a rejection")
  in
  check_invalid "empty lambda body" (Ast.lambda [ "x" ] []);
  check_invalid "empty begin" (Ast.sequence []);
  check_invalid "empty let body" (Ast.let_ [] []);
  check_invalid "empty cond" (Ast.cond [] None);
  check_invalid "empty else body" (Ast.cond [ Ast.bool true, [] ] (Some []));
  (match Ast.define_function "f" [] [] with
   | Error (Eval_error.Invalid_form _) -> ()
   | Error e ->
     Alcotest.fail ("empty define_function body: wrong error " ^ Eval_error.to_string e)
   | Ok _ -> Alcotest.fail "define_function accepted an empty body");
  Alcotest.check
    Alcotest.bool
    "single-clause cond without else is fine"
    true
    (match Ast.cond [ Ast.bool true, [] ] None with
     | Ok _ -> true
     | Error _ -> false)
;;

let () =
  Alcotest.run
    "sicp_common.ast"
    [ ( "ast"
      , Alcotest.
          [ test_case "self-evaluating forms" `Quick self_evaluating
          ; test_case "compound forms" `Quick compound_forms
          ; test_case "sequences and lets" `Quick sequences_and_lets
          ; test_case "definitions" `Quick definitions
          ; test_case
              "rejects malformed construction"
              `Quick
              rejects_malformed_construction
          ] )
    ]
;;
