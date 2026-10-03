(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Machine = Sicp_ch5.Sec_5_2
module Eval_error = Sicp_common.Eval_error

let label_input =
  List.find_map (function
    | M.Label_ref l -> Some l
    | M.Reg _ | M.Const _ -> None)
;;

let operation_inputs = function
  | M.Assign_op (_, _, inputs) | M.Test (_, inputs) | M.Perform (_, inputs) -> inputs
  | M.Label _ | M.Assign _ | M.Branch _ | M.Goto _ | M.Goto_reg _ | M.Save _ | M.Restore _
    -> []
;;

let check_operands controller =
  match List.find_map (fun i -> label_input (operation_inputs i)) controller with
  | None -> Ok ()
  | Some l ->
    Error
      (Eval_error.Bad_instruction
         ("an operation input is a register or a constant, not the label " ^ l))
;;

let make_machine ~registers ~operations ~controller =
  let* () = check_operands controller in
  Machine.make_machine ~registers ~operations ~controller
;;

let ex_5_09 () =
  let legal_controller = M.[ Assign_op ("a", "+", [ Reg "b"; Reg "c" ]) ] in
  let label_operand_controller =
    M.[ Assign_op ("a", "+", [ Reg "b"; Label_ref "there" ]); Label "there" ]
  in
  let outcome controller =
    let answer =
      let* m =
        make_machine
          ~registers:[ "a"; "b"; "c" ]
          ~operations:M.arith_operations
          ~controller
      in
      let* () = Machine.set_register m "b" (M.Int 2) in
      let* () = Machine.set_register m "c" (M.Int 3) in
      let* () = Machine.start m in
      Machine.get_register m "a"
    in
    match answer with
    | Ok v -> M.value_to_string v
    | Error e -> "Error: " ^ Eval_error.to_string e
  in
  Ok [ outcome legal_controller; outcome label_operand_controller ]
;;
