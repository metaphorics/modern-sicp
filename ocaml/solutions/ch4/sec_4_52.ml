(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.52: [if_fail]. The clause, added to the search evaluator
   by open recursion, is a choice point of two alternatives: the first
   expression, and the fallback that the search tries only when every
   branch of the first has failed. That is the book's [if-fail]: the
   fallback runs as the failure continuation of the first expression.
   The search runs to exhaustion, so each demonstration shows every
   answer. When every element is odd the first expression has no answer
   and [if_fail] answers ["all-odd"]. When 8 is present the first
   expression answers [8], and the fallback still follows once the
   first expression's branches are exhausted. *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval = Sicp_ch4.Sec_4_3

let rec eval : Eval.eval_t =
  fun search env e ->
  match Ast.view e with
  | Ast.Apply (head, [ first; fallback ]) ->
    (match Ast.view head with
     | Ast.Var "if_fail" ->
       let* taken = Eval.choose search 2 in
       eval search env (if taken = 0 then first else fallback)
     | _ -> Eval.open_eval ~self:eval search env e)
  | _ -> Eval.open_eval ~self:eval search env e
;;

let forms = [ "if_fail", "let if_fail x _y = x" ]

let program elements =
  {|
let rec an_element_of items =
  match items with
  | [] -> require false; 0
  | x :: rest -> amb x (an_element_of rest)

let even n = n mod 2 = 0

let () =
  print_endline
    (if_fail
       (let x = an_element_of |}
  ^ elements
  ^ {| in
        require (even x);
        string_of_int x)
       "all-odd")
|}
;;

let transcript elements =
  Eval.run_with ~eval ~forms (program elements)
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let ex_4_52 () = transcript "[ 1; 3; 5 ]" @ transcript "[ 1; 3; 5; 8 ]"
