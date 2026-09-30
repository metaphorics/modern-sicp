(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Eval_error = Sicp_common.Eval_error

let sqrt_stage1_controller =
  M.
    [ Label "sqrt-loop"
    ; Test ("good-enough?", [ Reg "guess"; Reg "x" ])
    ; Branch "sqrt-done"
    ; Assign_op ("guess", "improve", [ Reg "guess"; Reg "x" ])
    ; Goto "sqrt-loop"
    ; Label "sqrt-done"
    ; Perform ("print", [ Reg "guess" ])
    ]
;;

(* An operation's inputs are registers and constants only, so the two
   temporaries [t] and [u] carry the intermediate values. *)
let sqrt_stage2_controller =
  M.
    [ Label "sqrt-loop"
    ; Assign_op ("t", "*.", [ Reg "guess"; Reg "guess" ])
    ; Assign_op ("t", "-.", [ Reg "t"; Reg "x" ])
    ; Assign_op ("t", "abs", [ Reg "t" ])
    ; Test ("<", [ Reg "t"; Const (Float 0.001) ])
    ; Branch "sqrt-done"
    ; Assign_op ("t", "/.", [ Reg "x"; Reg "guess" ])
    ; Assign_op ("u", "+.", [ Reg "t"; Reg "guess" ])
    ; Assign_op ("u", "/.", [ Reg "u"; Const (Float 2.0) ])
    ; Assign ("guess", Reg "u")
    ; Goto "sqrt-loop"
    ; Label "sqrt-done"
    ; Perform ("print", [ Reg "guess" ])
    ]
;;

let good_enough_operation =
  ( "good-enough?"
  , M.Test_op
      (function
        | [ M.Float g; M.Float x ] -> Ok (Float.abs ((g *. g) -. x) < 0.001)
        | _ -> Error (Eval_error.Type_error "good-enough? needs two floats")) )
;;

let improve_operation =
  ( "improve"
  , M.Value_op
      (function
        | [ M.Float g; M.Float x ] -> Ok (M.Float ((g +. (x /. g)) /. 2.0))
        | _ -> Error (Eval_error.Type_error "improve needs two floats")) )
;;

let run_sqrt controller operations x =
  let printed = Buffer.create 32 in
  let* m =
    M.make_machine
      ~registers:[ "guess"; "x"; "t"; "u" ]
      ~operations:(M.print_operation (Buffer.add_string printed) :: operations)
      ~controller
  in
  let* () = M.set_register m "guess" (M.Float 1.0) in
  let* () = M.set_register m "x" (M.Float x) in
  let* () = M.start m in
  Ok (String.trim (Buffer.contents printed))
;;

let ex_5_03 () =
  let stage1 = [ good_enough_operation; improve_operation ] in
  let* a2 = run_sqrt sqrt_stage1_controller stage1 2.0 in
  let* a9 = run_sqrt sqrt_stage1_controller stage1 9.0 in
  let* b2 = run_sqrt sqrt_stage2_controller M.arith_operations 2.0 in
  let* b9 = run_sqrt sqrt_stage2_controller M.arith_operations 9.0 in
  Ok [ a2; a9; b2; b9 ]
;;
