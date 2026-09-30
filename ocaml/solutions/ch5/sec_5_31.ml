(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1

let ( let* ) = Result.bind

let combinations =
  [ "f 1 2", "let f a b = a + b\nlet combination = f 1 2"
  ; ( "(pick true) 1 2"
    , "let pick flag = if flag then fun a b -> a + b else fun a b -> a - b\n\
       let combination = (pick true) 1 2" )
  ; "f (g 1) y", "let f a b = a + b\nlet g a = a\nlet y = 2\nlet combination = f (g 1) y"
  ; "f (g 1) 2", "let f a b = a + b\nlet g a = a\nlet combination = f (g 1) 2"
  ]
;;

let last_right_hand_side program =
  match List.rev (Check.items program) with
  | Ast.Value_item (_, [ binding ]) :: _ -> Ok binding.rhs
  | _ -> Error (Sicp_common.Eval_error.Invalid_form "the unit ends in one binding")
;;

let stack_operations (seq : C.seq) =
  List.filter_map
    (function
      | M.Save r -> Some ("save " ^ r)
      | M.Restore r -> Some ("restore " ^ r)
      | _ -> None)
    seq.statements
;;

let ex_5_31 () =
  let state = C.new_state () in
  List.fold_right
    (fun (shown, source) acc ->
       let* lines = acc in
       let* program = Sec_5_33.program ~filename:"ex_5_31.ml" source in
       let* e = last_right_hand_side program in
       let seq = C.compile state e "val" C.Next in
       Ok ((shown ^ ": " ^ String.concat "; " (stack_operations seq)) :: lines))
    combinations
    (Ok [])
;;
