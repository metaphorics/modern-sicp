(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

module M = Sicp_ch5.Sec_5_1
module Machine = Sicp_ch5.Sec_5_2
module Eval_error = Sicp_common.Eval_error

let ambiguous_controller =
  M.
    [ Assign ("a", Const (Int 1))
    ; Label "again"
    ; Assign_op ("a", "+", [ Reg "a"; Reg "a" ])
    ; Goto "again"
    ; Label "again"
    ; Assign_op ("a", "-", [ Reg "a"; Const (Int 1) ])
    ; Goto "again"
    ]
;;

let ex_5_08 () =
  let report =
    match
      Machine.make_machine
        ~registers:[ "a" ]
        ~operations:M.arith_operations
        ~controller:ambiguous_controller
    with
    | Ok _ -> "assembled (the ambiguity went undetected)"
    | Error e -> "Error: " ^ Eval_error.to_string e
  in
  Ok [ report ]
;;
