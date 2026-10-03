(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5

let ( let* ) = Result.bind

let shadowed environments e name =
  match Sec_5_40.find environments e with
  | Some frames -> Sec_5_41.find_variable name frames <> None
  | None -> false
;;

let rec compile_scoped environments state e target linkage =
  let self = compile_scoped environments in
  match Ast.view e with
  | Ast.Apply (operator, operands) ->
    (match Sec_5_38.open_coded_name operator operands with
     | Some name when not (shadowed environments operator name) ->
       Sec_5_38.open_coded_call self state name operands target linkage
     | Some _ | None -> C.compile_open ~self state e target linkage)
  | _ -> C.compile_open ~self state e target linkage
;;

let shadowing =
  "let describe string_of_int n = string_of_int (n + 1)\n\
   let result = describe (fun n -> if n > 40 then \"large\" else \"small\") 41\n"
;;

let free = "let describe n = string_of_int (n + 1)\nlet result = describe 41\n"

let ex_5_44 () =
  let run source =
    let* p = Sec_5_33.program ~filename:"ex_5_44.ml" source in
    let items = Check.items p in
    let naive =
      C.compile_program_with ~compile:Sec_5_38.compile_open_coding (C.new_state ()) items
    in
    let scoped =
      C.compile_program_with
        ~compile:(compile_scoped (Sec_5_40.environments items))
        (C.new_state ())
        items
    in
    let* naive_run = Sec_5_33.run_code ~operations:Sec_5_38.operations naive in
    let* scoped_run = Sec_5_33.run_code ~operations:Sec_5_38.operations scoped in
    Ok
      (Printf.sprintf
         "5.38 compiler: %d open-coded, answers %s; scoped: %d open-coded, answers %s"
         (Sec_5_38.open_coded_count naive)
         (Value.to_string naive_run.value)
         (Sec_5_38.open_coded_count scoped)
         (Value.to_string scoped_run.value))
  in
  let* shadowed_line = run shadowing in
  let* free_line = run free in
  Ok [ "shadowed parameter: " ^ shadowed_line; "free name: " ^ free_line ]
;;
