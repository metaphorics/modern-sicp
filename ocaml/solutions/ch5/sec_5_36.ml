(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let prepend_arg =
  ( "prepend-arg"
  , M.Value_op
      (function
        | [ W.V v; W.Args vs ] -> Ok (W.Args (v :: vs))
        | [ _; _ ] ->
          Error (Eval_error.Bad_instruction "prepend-arg expects a value and a list")
        | ws -> Error (Eval_error.Arity_mismatch { expected = 2; given = List.length ws }))
  )
;;

let construct_arglist_right_to_left compile state operands =
  let start =
    C.make_instruction_sequence [] [ "argl" ] [ M.Assign ("argl", M.Const (W.Args [])) ]
  in
  List.fold_left
    (fun acc operand ->
       C.preserving
         [ "env" ]
         acc
         (C.preserving
            [ "argl" ]
            (compile state operand "val" C.Next)
            (C.make_instruction_sequence
               [ "val"; "argl" ]
               [ "argl" ]
               [ M.Assign_op ("argl", "prepend-arg", [ M.Reg "val"; M.Reg "argl" ]) ])))
    start
    (List.rev operands)
;;

let rec compile_right_to_left state e target linkage =
  match Ast.view e with
  | Ast.Apply (operator, operands) ->
    let proc_code = compile_right_to_left state operator "proc" C.Next in
    let operand_codes =
      construct_arglist_right_to_left compile_right_to_left state operands
    in
    C.preserving
      [ "env"; "continue" ]
      proc_code
      (C.preserving
         [ "proc"; "continue" ]
         operand_codes
         (C.compile_procedure_call state target linkage))
  | _ -> C.compile_open ~self:compile_right_to_left state e target linkage
;;

let source =
  "let show n =\n\
  \  print_int n;\n\
  \  n\n\
   let add3 a b c = a + b + c\n\
   let result = add3 (show 1) (show 2) (show 3)\n"
;;

let ex_5_36 () =
  let* p = Sec_5_33.program ~filename:"ex_5_36.ml" source in
  let items = Check.items p in
  let default_code = C.compile_program (C.new_state ()) items in
  let reordered_code =
    C.compile_program_with ~compile:compile_right_to_left (C.new_state ()) items
  in
  let* default_run = Sec_5_33.run_code default_code in
  let* reordered_run = Sec_5_33.run_code ~operations:[ prepend_arg ] reordered_code in
  let count (s : C.seq) = List.length s.statements in
  Ok
    [ "default order: " ^ default_run.output
    ; "right-to-left order: " ^ reordered_run.output
    ; Printf.sprintf
        "statements: %d and %d; steps: %d and %d"
        (count default_code)
        (count reordered_code)
        default_run.steps
        reordered_run.steps
    ; "answers: "
      ^ Sicp_common.Value.to_string default_run.value
      ^ " and "
      ^ Sicp_common.Value.to_string reordered_run.value
    ]
;;
