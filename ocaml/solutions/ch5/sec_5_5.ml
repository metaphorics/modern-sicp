(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Eval_error = Sicp_common.Eval_error

let factorial_recursive_controller =
  M.
    [ Assign ("continue", Label_ref "fact-done")
    ; Label "fact-loop"
    ; Test ("=", [ Reg "n"; Const (Int 1) ])
    ; Branch "base-case"
    ; Save "continue"
    ; Save "n"
    ; Assign_op ("n", "-", [ Reg "n"; Const (Int 1) ])
    ; Assign ("continue", Label_ref "after-fact")
    ; Goto "fact-loop"
    ; Label "after-fact"
    ; Restore "n"
    ; Restore "continue"
    ; Assign_op ("val", "*", [ Reg "n"; Reg "val" ])
    ; Goto_reg "continue"
    ; Label "base-case"
    ; Assign ("val", Const (Int 1))
    ; Goto_reg "continue"
    ; Label "fact-done"
    ]
;;

let fib_controller =
  M.
    [ Assign ("continue", Label_ref "fib-done")
    ; Label "fib-loop"
    ; Test ("<", [ Reg "n"; Const (Int 2) ])
    ; Branch "immediate-answer"
    ; Save "continue"
    ; Assign ("continue", Label_ref "afterfib-n-1")
    ; Save "n"
    ; Assign_op ("n", "-", [ Reg "n"; Const (Int 1) ])
    ; Goto "fib-loop"
    ; Label "afterfib-n-1"
    ; Restore "n"
    ; Restore "continue"
    ; Assign_op ("n", "-", [ Reg "n"; Const (Int 2) ])
    ; Save "continue"
    ; Assign ("continue", Label_ref "afterfib-n-2")
    ; Save "val"
    ; Goto "fib-loop"
    ; Label "afterfib-n-2"
    ; Assign ("n", Reg "val")
    ; Restore "val"
    ; Restore "continue"
    ; Assign_op ("val", "+", [ Reg "val"; Reg "n" ])
    ; Goto_reg "continue"
    ; Label "immediate-answer"
    ; Assign ("val", Reg "n")
    ; Goto_reg "continue"
    ; Label "fib-done"
    ]
;;

(* The hand model shares only the operation table with the simulator:
   its pc, flag, registers, and stack are its own, so its traces are the
   hand simulation's answers, not the simulator's. *)
module Handsim = struct
  type state =
    { pc : int
    ; flag : bool
    ; regs : (string * M.value) list
    ; stack : M.value list
    ; steps : int
    ; saves : int
    }

  type event =
    | Saved of string * M.value list
    | Restored of string * M.value * M.value list
    | Branch_taken of string
    | Returned_to of string

  let initial regs = { pc = 0; flag = false; regs; stack = []; steps = 0; saves = 0 }

  let find regs r =
    match List.assoc_opt r regs with
    | Some v -> Ok v
    | None -> Error (Eval_error.Unknown_register r)
  ;;

  let bind regs r v = (r, v) :: List.remove_assoc r regs

  let eval regs = function
    | M.Reg r -> find regs r
    | M.Const v -> Ok v
    | M.Label_ref l -> Ok (M.Addr l)
  ;;

  let rec eval_all regs = function
    | [] -> Ok []
    | s :: rest ->
      let* v = eval regs s in
      let* vs = eval_all regs rest in
      Ok (v :: vs)
  ;;

  let operation name =
    match List.assoc_opt name M.arith_operations with
    | Some op -> Ok op
    | None -> Error (Eval_error.Unknown_operation name)
  ;;

  let wrong_kind name wanted =
    Error (Eval_error.Bad_instruction ("operation " ^ name ^ " is not " ^ wanted))
  ;;

  let value_op name =
    let* op = operation name in
    match op with
    | M.Value_op f -> Ok f
    | _ -> wrong_kind name "a value operation"
  ;;

  let test_op name =
    let* op = operation name in
    match op with
    | M.Test_op f -> Ok f
    | _ -> wrong_kind name "a test operation"
  ;;

  let action_op name =
    let* op = operation name in
    match op with
    | M.Action_op f -> Ok f
    | _ -> wrong_kind name "an action"
  ;;

  let label_index (program : M.value M.program) l =
    match List.assoc_opt l program.labels with
    | Some i -> Ok i
    | None -> Error (Eval_error.Unknown_label l)
  ;;

  let rec run (program : M.value M.program) st events =
    if st.pc >= Array.length program.code
    then Ok (List.rev events, st)
    else (
      let next = { st with pc = st.pc + 1; steps = st.steps + 1 } in
      let jump l event =
        let* i = label_index program l in
        run program { next with pc = i } (event :: events)
      in
      match program.code.(st.pc) with
      | M.Label _ -> run program { st with pc = st.pc + 1 } events
      | M.Assign (r, src) ->
        let* v = eval st.regs src in
        run program { next with regs = bind st.regs r v } events
      | M.Assign_op (r, name, inputs) ->
        let* args = eval_all st.regs inputs in
        let* f = value_op name in
        let* v = f args in
        run program { next with regs = bind st.regs r v } events
      | M.Test (name, inputs) ->
        let* args = eval_all st.regs inputs in
        let* f = test_op name in
        let* b = f args in
        run program { next with flag = b } events
      | M.Branch l -> if st.flag then jump l (Branch_taken l) else run program next events
      | M.Goto l ->
        let* i = label_index program l in
        run program { next with pc = i } events
      | M.Goto_reg r ->
        let* v = find st.regs r in
        (match v with
         | M.Addr l -> jump l (Returned_to l)
         | other ->
           Error
             (Eval_error.Bad_instruction
                ("goto through " ^ r ^ ", which holds " ^ M.value_to_string other)))
      | M.Perform (name, inputs) ->
        let* args = eval_all st.regs inputs in
        let* f = action_op name in
        let* () = f args in
        run program next events
      | M.Save r ->
        let* v = find st.regs r in
        let stack = v :: st.stack in
        run program { next with stack; saves = st.saves + 1 } (Saved (r, stack) :: events)
      | M.Restore r ->
        (match st.stack with
         | v :: rest ->
           run
             program
             { next with regs = bind st.regs r v; stack = rest }
             (Restored (r, v, rest) :: events)
         | [] ->
           Error (Eval_error.Bad_instruction ("restore " ^ r ^ " from an empty stack"))))
  ;;

  let stack_to_string stack =
    "[" ^ String.concat "; " (List.map M.value_to_string stack) ^ "]"
  ;;

  let render = function
    | Saved (r, stack) -> Printf.sprintf "Save %S stack=%s" r (stack_to_string stack)
    | Restored (r, v, stack) ->
      Printf.sprintf
        "Restore %S %s=%s stack=%s"
        r
        r
        (M.value_to_string v)
        (stack_to_string stack)
    | Branch_taken l -> "branch taken to " ^ l
    | Returned_to l -> "return to " ^ l
  ;;
end

let hand_trace controller initial answer =
  let* program = M.assemble controller in
  let* events, st = Handsim.run program (Handsim.initial initial) [] in
  let* v = Handsim.find st.regs answer in
  Ok (List.map Handsim.render events @ [ "answer " ^ M.value_to_string v ])
;;

let ex_5_05 () =
  let initial = [ "n", M.Int 3; "val", M.Int 0 ] in
  let* fact_trace = hand_trace factorial_recursive_controller initial "val" in
  let* fib_trace = hand_trace fib_controller initial "val" in
  Ok (fact_trace @ fib_trace)
;;
