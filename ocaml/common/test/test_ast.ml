(* SPDX-License-Identifier: GPL-3.0-only *)

module Ast = Sicp_common.Ast

let name = function
  | Ast.Var n -> n
  | Ast.Scalar (Ast.Int n) -> string_of_int n
  | _ -> "?"
;;

(* An effectful [f] must observe the children of [string_of_int 42] in
   source order: the operator first, then the operand. *)
let apply_children_in_source_order () =
  let e = Ast.apply (Ast.var "string_of_int") [ Ast.scalar (Ast.Int 42) ] in
  let seen = ref [] in
  let f child =
    seen := name (Ast.view child) :: !seen;
    child
  in
  let _ = Ast.map_children f e in
  Alcotest.(check (list string))
    "operator then operand"
    [ "string_of_int"; "42" ]
    (List.rev !seen)
;;

(* Binary nodes sequence left before right as well. *)
let arith_children_in_source_order () =
  let e = Ast.apply (Ast.var "f") [ Ast.var "a"; Ast.var "b" ] in
  let seen = ref [] in
  let f child =
    seen := name (Ast.view child) :: !seen;
    child
  in
  let _ = Ast.map_children f e in
  Alcotest.(check (list string)) "f a b" [ "f"; "a"; "b" ] (List.rev !seen)
;;

let () =
  Alcotest.run
    "ast"
    [ ( "map_children"
      , [ Alcotest.test_case "apply order" `Quick apply_children_in_source_order
        ; Alcotest.test_case "operand order" `Quick arith_children_in_source_order
        ] )
    ]
;;
