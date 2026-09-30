(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1

let ( let* ) = Result.bind

let iterative =
  "let factorial n =\n\
  \  let rec iter product counter =\n\
  \    if counter > n then product else iter (counter * product) (counter + 1)\n\
  \  in\n\
  \  iter 1 1"
;;

let is_compiled_branch l = String.length l > 15 && String.sub l 0 15 = "compiled-branch"

let rec until_jump acc = function
  | [] -> List.rev acc, []
  | (M.Goto "compiled-apply" as jump) :: rest -> List.rev (jump :: acc), rest
  | i :: rest -> until_jump (i :: acc) rest
;;

let compiled_branches statements =
  let rec go acc = function
    | [] -> List.rev acc
    | M.Label l :: _ as rest when is_compiled_branch l ->
      let branch, rest = until_jump [] rest in
      go (branch :: acc) rest
    | _ :: rest -> go acc rest
  in
  go [] statements
;;

let depth definition n =
  let* p =
    Sec_5_33.program
      ~filename:"ex_5_34.ml"
      (definition ^ "\nlet result = factorial " ^ string_of_int n ^ "\n")
  in
  let* r = Sec_5_33.run_program p in
  Ok r.depth
;;

let ex_5_34 () =
  let* p = Sec_5_33.program ~filename:"ex_5_34.ml" iterative in
  let* rhs =
    match Check.items p with
    | [ Ast.Value_item (false, [ b ]) ] -> Ok b.rhs
    | _ -> Error (Eval_error.Invalid_form "one definition")
  in
  let code = C.compile (C.new_state ()) rhs "val" C.Next in
  let* recursive = Sec_5_33.definition_code Sec_5_33.factorial in
  let render call = String.concat "; " (List.map Sec_5_35.statement_to_string call) in
  let calls name (s : C.seq) =
    List.map (fun c -> name ^ " call: " ^ render c) (compiled_branches s.statements)
  in
  let* depths =
    List.fold_right
      (fun n acc ->
         let* lines = acc in
         let* d_iter = depth iterative n in
         let* d_rec = depth Sec_5_33.factorial n in
         Ok
           (Printf.sprintf "depth at n = %d: iterative %d, recursive %d" n d_iter d_rec
            :: lines))
      [ 3; 4; 5 ]
      (Ok [])
  in
  let saves =
    List.length
      (List.filter
         (function
           | M.Save _ -> true
           | _ -> false)
         code.statements)
  in
  Ok
    (calls "iterative" code
     @ calls "recursive" recursive
     @ [ Printf.sprintf "saves in the iterative compilation: %d" saves ]
     @ depths)
;;
