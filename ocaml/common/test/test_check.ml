(* SPDX-License-Identifier: GPL-3.0-only *)

module Check = Sicp_common.Check

let kind source =
  match Check.check ~filename:"test.ml" source with
  | Ok _ -> "admitted"
  | Error d -> Check.kind_to_string d.kind
;;

let admitted = "admitted"
let syntax = Check.kind_to_string Check.Syntax
let unsupported = Check.kind_to_string Check.Unsupported
let type_invalid = Check.kind_to_string Check.Type_error
let contract = Check.kind_to_string Check.Contract
let expect name expected source = Alcotest.(check string) name expected (kind source)

(* Each admission stage rejects its own class of source, in order. *)
let stages () =
  expect
    "a closed-grammar program"
    admitted
    "type t = Leaf | Node of t * t\n\
     let rec size t = match t with Leaf -> 0 | Node (l, r) -> 1 + size l + size r\n\
     let () = print_int (size (Node (Leaf, Node (Leaf, Leaf))))";
  expect "a parse failure" syntax "let x = ";
  expect "a non-ASCII byte" syntax "let s = \"\xc3\xa9\"";
  expect "a loop" unsupported "let () = while true do () done";
  expect "an exception" unsupported "let f x = raise Not_found";
  expect "a module path" unsupported "let n = List.nth [ 1 ] 0";
  expect "an ill-typed program" type_invalid "let () = print_int \"one\"";
  expect "a partial match" contract "let f x = match x with 0 -> 1";
  expect
    "equality on a structured type"
    contract
    "type p = P of int\nlet same = P 1 = P 2"
;;

(* The admitted operator forms include unary negation of both kinds and
   the fixed prelude, and exclude operators used as values. *)
let operators () =
  expect "unary minus" admitted "let f x = - x\nlet g y = -. y";
  expect "a prelude call" admitted "let () = print_endline (string_of_float (sqrt 2.0))";
  expect "an operator as a value" unsupported "let add = ( + )";
  expect
    "a recursive closure knot"
    admitted
    "type v = C of (int -> v)\nlet rec k = C (fun _ -> k)"
;;

let () =
  Alcotest.run
    "check"
    [ ( "admission"
      , [ Alcotest.test_case "stages" `Quick stages
        ; Alcotest.test_case "operators" `Quick operators
        ] )
    ]
;;
