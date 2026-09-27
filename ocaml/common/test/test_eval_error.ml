(* SPDX-License-Identifier: GPL-3.0-only *)
module Eval_error = Sicp_common.Eval_error

(* Every constructor renders through one printer, subsystem and detail
   included, so a failing test names the failure. *)
let pp_renders () =
  let check detail actual expected =
    Alcotest.check Alcotest.string detail expected (Eval_error.to_string actual)
  in
  check "unbound variable" (Eval_error.Unbound_variable "x") "unbound variable x";
  check
    "arity mismatch"
    (Eval_error.Arity_mismatch { expected = 2; given = 3 })
    "arity mismatch: expected 2, given 3";
  check
    "type error"
    (Eval_error.Type_error "car: expected a pair, got 5")
    "type error: car: expected a pair, got 5";
  check
    "not applicable"
    (Eval_error.Not_applicable "5")
    "not applicable: 5 is not a procedure";
  check "division by zero" Eval_error.Division_by_zero "division by zero";
  check
    "invalid form"
    (Eval_error.Invalid_form "let needs a nonempty body")
    "invalid form: let needs a nonempty body";
  check
    "user error"
    (Eval_error.User_error
       "The object 3, passed as the first argument to car, is not the correct type.")
    "The object 3, passed as the first argument to car, is not the correct type."
;;

let pp_goes_to_formatter () =
  let buffer = Buffer.create 64 in
  let ppf = Format.formatter_of_buffer buffer in
  Eval_error.pp ppf Eval_error.Division_by_zero;
  Format.pp_print_flush ppf ();
  Alcotest.check
    Alcotest.string
    "pp writes the same text"
    "division by zero"
    (Buffer.contents buffer)
;;

let () =
  Alcotest.run
    "sicp_common.eval_error"
    [ ( "eval_error"
      , Alcotest.
          [ test_case "pp renders every constructor" `Quick pp_renders
          ; test_case "pp writes to a formatter" `Quick pp_goes_to_formatter
          ] )
    ]
;;
