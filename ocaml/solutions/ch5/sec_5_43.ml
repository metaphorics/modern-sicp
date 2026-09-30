(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1

let ( let* ) = Result.bind

let source =
  "let parity n =\n\
  \  let rec even k = if k = 0 then true else odd (k - 1)\n\
  \  and odd k = if k = 0 then false else even (k - 1) in\n\
  \  if even n then 1 else 3\n\
   let result = parity 7\n"
;;

let group_operations (s : C.seq) =
  List.filter_map
    (function
      | M.Assign_op (_, (("let-rec-group" | "group-environment") as op), _)
      | M.Perform (("fill-first-pending" as op), _) -> Some op
      | _ -> None)
    s.statements
;;

let ex_5_43 () =
  let* p = Sec_5_33.program ~filename:"ex_5_43.ml" source in
  let items = Check.items p in
  let* body =
    match items with
    | Ast.Value_item (false, [ b ]) :: _ ->
      Ok (C.compile (C.new_state ()) b.rhs "val" C.Next)
    | _ -> Error (Eval_error.Invalid_form "parity first")
  in
  let* plain = Sec_5_33.run_program p in
  let* lexical =
    Sec_5_33.run_code
      ~operations:[ Sec_5_39.operation ]
      (Sec_5_42.compile_program_lexical items)
  in
  Ok
    [ "group code in order: " ^ String.concat ", " (group_operations body)
    ; "plain run: " ^ Value.to_string plain.value
    ; "lexical run: " ^ Value.to_string lexical.value
    ]
;;
