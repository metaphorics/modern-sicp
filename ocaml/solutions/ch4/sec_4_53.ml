(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.53: [permanent_set] and [if_fail] together. The evaluator
   carries both clauses of exercises 4.51 and 4.52. Every prime-sum pair
   the search finds is pushed onto [pairs] by a permanent assignment, and
   the branch then fails on purpose, so the first expression of
   [if_fail] never answers. Once its branches are exhausted the fallback
   reads [pairs], which the backtracking did not undo, and the program
   answers the list of every pair, newest first: [((8 35) (3 110) (3
   20))]. The book's [pairs] is a local [let] binding; the subset's
   permanent store holds named top-level references, so [pairs] is
   declared at top level. *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval = Sicp_ch4.Sec_4_3

let rec eval : Eval.eval_t =
  fun search env e ->
  match Ast.view e with
  | Ast.Apply (head, [ first; second ]) ->
    (match Ast.view head, Ast.view first with
     | Ast.Var "permanent_set", Ast.Var name ->
       let* value = eval search env second in
       Eval.permanent_assign search name value
     | Ast.Var "if_fail", _ ->
       let* taken = Eval.choose search 2 in
       eval search env (if taken = 0 then first else second)
     | _ -> Eval.open_eval ~self:eval search env e)
  | _ -> Eval.open_eval ~self:eval search env e
;;

let forms =
  [ "permanent_set", "let permanent_set r v = r := v"; "if_fail", "let if_fail x _y = x" ]
;;

let program =
  {|
let rec an_element_of items =
  match items with
  | [] -> require false; 0
  | x :: rest -> amb x (an_element_of rest)

let rec divides_none n d = d * d > n || (n mod d <> 0 && divides_none n (d + 1))
let prime n = n >= 2 && divides_none n 2

let prime_sum_pair list1 list2 =
  let a = an_element_of list1 in
  let b = an_element_of list2 in
  require (prime (a + b));
  (a, b)

let rec show_pairs pairs =
  match pairs with
  | [] -> ""
  | [ (a, b) ] -> "(" ^ string_of_int a ^ " " ^ string_of_int b ^ ")"
  | (a, b) :: rest -> "(" ^ string_of_int a ^ " " ^ string_of_int b ^ ") " ^ show_pairs rest

let pairs = ref []

let () =
  let found =
    if_fail
      (let p = prime_sum_pair [ 1; 3; 5; 8 ] [ 20; 35; 110 ] in
       permanent_set pairs (p :: !pairs);
       require false;
       !pairs)
      !pairs
  in
  print_endline ("(" ^ show_pairs found ^ ")")
|}
;;

let ex_4_53 () =
  Eval.run_with ~eval ~forms program
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;
