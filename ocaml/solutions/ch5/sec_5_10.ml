(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Machine = Sicp_ch5.Sec_5_2
module Eval_error = Sicp_common.Eval_error

type term =
  | R of string
  | N of int
  | L of string
  | Call of string * term list

type stmt =
  | Name of string
  | Set of string * term
  | Check of string * term list
  | Jump_if of string
  | Jump of term
  | Push of string
  | Pop of string
  | Do of string * term list

let bad detail = Error (Eval_error.Bad_instruction detail)

let operand = function
  | R r -> Ok (M.Reg r)
  | N n -> Ok (M.Const (M.Int n))
  | L l -> Ok (M.Label_ref l)
  | Call (name, _) -> bad ("the call " ^ name ^ " cannot be an operand")
;;

let rec operands = function
  | [] -> Ok []
  | t :: rest ->
    let* s = operand t in
    let* ss = operands rest in
    Ok (s :: ss)
;;

let instruction = function
  | Name l -> Ok (M.Label l)
  | Set (r, Call (name, args)) ->
    let* inputs = operands args in
    Ok (M.Assign_op (r, name, inputs))
  | Set (r, t) ->
    let* s = operand t in
    Ok (M.Assign (r, s))
  | Check (name, args) ->
    let* inputs = operands args in
    Ok (M.Test (name, inputs))
  | Jump_if l -> Ok (M.Branch l)
  | Jump (L l) -> Ok (M.Goto l)
  | Jump (R r) -> Ok (M.Goto_reg r)
  | Jump (N _ | Call _) -> bad "a jump goes to a label or through a register"
  | Push r -> Ok (M.Save r)
  | Pop r -> Ok (M.Restore r)
  | Do (name, args) ->
    let* inputs = operands args in
    Ok (M.Perform (name, inputs))
;;

let rec syntax = function
  | [] -> Ok []
  | s :: rest ->
    let* i = instruction s in
    let* is = syntax rest in
    Ok (i :: is)
;;

let gcd_controller =
  M.
    [ Label "test-b"
    ; Test ("=", [ Reg "b"; Const (Int 0) ])
    ; Branch "gcd-done"
    ; Assign_op ("t", "rem", [ Reg "a"; Reg "b" ])
    ; Assign ("a", Reg "b")
    ; Assign ("b", Reg "t")
    ; Goto "test-b"
    ; Label "gcd-done"
    ]
;;

let gcd_new_syntax =
  [ Name "test-b"
  ; Check ("=", [ R "b"; N 0 ])
  ; Jump_if "gcd-done"
  ; Set ("t", Call ("rem", [ R "a"; R "b" ]))
  ; Set ("a", R "b")
  ; Set ("b", R "t")
  ; Jump (L "test-b")
  ; Name "gcd-done"
  ]
;;

let run_gcd controller =
  let* m =
    Machine.make_machine
      ~registers:[ "a"; "b"; "t" ]
      ~operations:M.arith_operations
      ~controller
  in
  let* () = Machine.set_register m "a" (M.Int 12) in
  let* () = Machine.set_register m "b" (M.Int 8) in
  let* () = Machine.start m in
  Machine.get_register m "a"
;;

let ex_5_10 () =
  let* translated = syntax gcd_new_syntax in
  let* old_answer = run_gcd gcd_controller in
  let* new_answer = run_gcd translated in
  let* old_program = M.assemble gcd_controller in
  let* new_program = M.assemble translated in
  let show_third (program : M.value M.program) =
    M.instruction_to_string M.value_to_string program.code.(2)
  in
  Ok
    [ M.value_to_string old_answer
    ; M.value_to_string new_answer
    ; show_third old_program
    ; show_third new_program
    ]
;;
