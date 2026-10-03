(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.51: [permanent_set]. The clause, added to the search
   evaluator by open recursion, evaluates the new value and writes it
   into the named top-level reference through the experiment's permanent
   store: the write survives backtracking, so the trials that fail still
   leave their count behind. The search runs to exhaustion, so the
   demonstration shows every answer. With [permanent_set] the counter
   records every trial made so far: [(a b 2)] (the rejected [x = a, y =
   a] trial counted), then [(a c 3)], [(b a 4)], and so on. With the
   plain [:=] each failed trial's assignment is rolled back with its
   branch, and every answer reports [1], the book's answer to the
   exercise's question. *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval = Sicp_ch4.Sec_4_3

let rec eval : Eval.eval_t =
  fun search env e ->
  match Ast.view e with
  | Ast.Apply (head, [ target; value ]) ->
    (match Ast.view head, Ast.view target with
     | Ast.Var "permanent_set", Ast.Var name ->
       let* value = eval search env value in
       Eval.permanent_assign search name value
     | _ -> Eval.open_eval ~self:eval search env e)
  | _ -> Eval.open_eval ~self:eval search env e
;;

let forms = [ "permanent_set", "let permanent_set r v = r := v" ]

let program assignment =
  {|
let rec an_element_of items =
  match items with
  | [] -> require false; ""
  | x :: rest -> amb x (an_element_of rest)

let count = ref 0

let () =
  let x = an_element_of [ "a"; "b"; "c" ] in
  let y = an_element_of [ "a"; "b"; "c" ] in
  |}
  ^ assignment
  ^ {|;
  require (x <> y);
  print_endline ("(" ^ x ^ " " ^ y ^ " " ^ string_of_int !count ^ ")")
|}
;;

let transcript assignment =
  Eval.run_with ~eval ~forms (program assignment)
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let ex_4_51 () =
  ("permanent_set" :: transcript "permanent_set count (!count + 1)")
  @ (":=" :: transcript "count := !count + 1")
;;
