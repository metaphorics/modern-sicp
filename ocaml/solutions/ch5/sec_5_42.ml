(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1

let ( let* ) = Result.bind

let rec compile_lexical environments state e target linkage =
  let self = compile_lexical environments in
  match Ast.view e, Sec_5_40.find environments e with
  | Ast.Var name, Some frames ->
    (match Sec_5_41.find_variable name frames with
     | Some address ->
       Sec_5_38.end_with_linkage
         linkage
         (C.make_instruction_sequence
            [ "env" ]
            [ target ]
            [ Sec_5_39.lookup_instruction frames address name target ])
     | None -> C.compile_open ~self state e target linkage)
  | _ -> C.compile_open ~self state e target linkage
;;

let compile_program_lexical items =
  let environments = Sec_5_40.environments items in
  C.compile_program_with ~compile:(compile_lexical environments) (C.new_state ()) items
;;

let lexical_lookups (s : C.seq) =
  List.filter
    (function
      | M.Assign_op (_, "lexical-address-lookup", _) -> true
      | _ -> false)
    s.statements
;;

let ex_5_42 () =
  let* p = Sec_5_33.program ~filename:"ex_5_42.ml" Sec_5_40.nested_example in
  let code = compile_program_lexical (Check.items p) in
  let* r = Sec_5_33.run_code ~operations:[ Sec_5_39.operation ] code in
  Ok
    (List.map Sec_5_35.statement_to_string (lexical_lookups code)
     @ [ "lexical run: " ^ Value.to_string r.value ])
;;
