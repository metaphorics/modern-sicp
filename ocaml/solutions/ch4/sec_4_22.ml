(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.22 *)

module Ast = Sicp_common.Ast
module S = Sicp_ch4.Sec_4_1

let rec lower_lets e =
  match Ast.view e with
  | Ast.Let (false, _, _) ->
    (match Sec_4_6.let_to_combination e with
     | Ok combination -> lower_lets combination
     | Error _ -> Ast.map_children lower_lets e)
  | _ -> Ast.map_children lower_lets e
;;

let analyze e = S.analyze (lower_lets e)
let eval : S.eval_t = fun e env -> analyze e env

let rec count_lets e =
  let own =
    match Ast.view e with
    | Ast.Let (false, _, _) -> 1
    | _ -> 0
  in
  own + List.fold_left (fun n child -> n + count_lets child) 0 (Sec_4_2.subexpressions e)
;;

let programs =
  [ "let x = 3 and y = 4 in x + y"
  ; "let x = 2 in let x = 3 and y = x in x + y"
  ; "let rec fib n = if n < 2 then n else let a = fib (n - 1) and b = fib (n - 2) in a + \
     b in fib 15"
  ]
;;

let ex_4_22 () =
  List.map
    (fun source ->
       let lets =
         match Sec_4_1.open_expression [] source with
         | Ok e ->
           Printf.sprintf
             "%d lets before, %d after"
             (count_lets e)
             (count_lets (lower_lets e))
         | Error _ -> "not admitted"
       in
       Sec_4_1.run_source eval source ^ " (" ^ lets ^ ")")
    programs
;;
