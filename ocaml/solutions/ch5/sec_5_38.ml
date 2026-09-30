(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let open_coded =
  [ "string_of_int", 1
  ; "float_of_int", 1
  ; "sqrt", 1
  ; "Array.length", 1
  ; "List.length", 1
  ; "Array.get", 2
  ]
;;

let argument_registers = [ "arg1"; "arg2" ]

let linkage_code = function
  | C.Next -> C.empty_instruction_sequence
  | C.Return -> C.make_instruction_sequence [ "continue" ] [] [ M.Goto_reg "continue" ]
  | C.Goto_label l -> C.make_instruction_sequence [] [] [ M.Goto l ]
;;

let end_with_linkage linkage s = C.preserving [ "continue" ] s (linkage_code linkage)

let spread_arguments compile state operands (operation : C.seq) =
  let rec go filled = function
    | [] -> operation
    | (operand, register) :: rest ->
      C.preserving
        ("env" :: filled)
        (compile state operand register C.Next)
        (go (register :: filled) rest)
  in
  go
    []
    (List.combine
       operands
       (List.filteri (fun i _ -> i < List.length operands) argument_registers))
;;

let open_coded_call compile state name operands target linkage =
  let registers = List.filteri (fun i _ -> i < List.length operands) argument_registers in
  end_with_linkage
    linkage
    (spread_arguments
       compile
       state
       operands
       (C.make_instruction_sequence
          registers
          [ target ]
          [ M.Assign_op (target, name, List.map (fun r -> M.Reg r) registers) ]))
;;

let open_coded_name e operands =
  match Ast.view e with
  | Ast.Var name when List.assoc_opt name open_coded = Some (List.length operands) ->
    Some name
  | _ -> None
;;

let rec compile_open_coding state e target linkage =
  match Ast.view e with
  | Ast.Apply (operator, operands) ->
    (match open_coded_name operator operands with
     | Some name -> open_coded_call compile_open_coding state name operands target linkage
     | None -> C.compile_open ~self:compile_open_coding state e target linkage)
  | _ -> C.compile_open ~self:compile_open_coding state e target linkage
;;

let no_callbacks _ _ =
  Error (Eval_error.Invalid_form "an open-coded primitive calls no guest procedure")
;;

let operations =
  let prelude = Prelude.initial_env ~emit:ignore () in
  List.filter_map
    (fun (name, _) ->
       match Option.map Value.view (Env.find prelude name) with
       | Some (Value.Primitive p) ->
         Some
           ( name
           , M.Value_op
               (fun ws ->
                 let* vs =
                   List.fold_right
                     (fun w acc ->
                        let* vs = acc in
                        match w with
                        | W.V v -> Ok (v :: vs)
                        | w ->
                          Error
                            (Eval_error.Bad_instruction
                               (name ^ " reads " ^ W.word_to_string w)))
                     ws
                     (Ok [])
                 in
                 Result.map (fun v -> W.V v) (p.prim_apply no_callbacks vs)) )
       | _ -> None)
    open_coded
;;

let open_coded_count (s : C.seq) =
  List.length
    (List.filter
       (function
         | M.Assign_op (_, op, _) -> List.mem_assoc op open_coded
         | _ -> false)
       s.statements)
;;

let source =
  "let rec sum_array a i =\n\
  \  if i = Array.length a then 0 else Array.get a i + sum_array a (i + 1)\n\
   let result = string_of_int (sum_array (Array.make 5 7) 0)\n"
;;

let ex_5_38 () =
  let* p = Sec_5_33.program ~filename:"ex_5_38.ml" source in
  let items = Check.items p in
  let plain = C.compile_program (C.new_state ()) items in
  let opened =
    C.compile_program_with ~compile:compile_open_coding (C.new_state ()) items
  in
  let* plain_run = Sec_5_33.run_code plain in
  let* opened_run = Sec_5_33.run_code ~operations opened in
  let line name (s : C.seq) (r : Sec_5_33.run) =
    Printf.sprintf
      "%s: %d statements, %d open-coded operations; answers %s in %d steps"
      name
      (List.length s.statements)
      (open_coded_count s)
      (Value.to_string r.value)
      r.steps
  in
  let* call =
    match Check.items p with
    | Ast.Value_item (true, [ b ]) :: _ ->
      let code = compile_open_coding (C.new_state ()) b.rhs "val" C.Next in
      Ok
        (List.filter_map
           (function
             | M.Assign_op (_, op, _) as i when List.mem_assoc op open_coded ->
               Some (Sec_5_35.statement_to_string i)
             | _ -> None)
           code.statements)
    | _ -> Error (Eval_error.Invalid_form "sum_array first")
  in
  Ok ([ line "plain" plain plain_run; line "open-coded" opened opened_run ] @ call)
;;
