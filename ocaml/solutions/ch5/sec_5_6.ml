(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Handsim = Sec_5_5.Handsim

(* Figure 5.12 without the [Restore "continue"] at afterfib-n-1 and the
   [Save "continue"] of the second call's setup: each level's return
   address is pushed once, before the first call, and popped once, at
   afterfib-n-2. The removed pair carried the same word twice. *)
let fib_modified_controller =
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
    ; Assign_op ("n", "-", [ Reg "n"; Const (Int 2) ])
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

let run_fib controller n =
  M.run
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:M.arith_operations
    ~inputs:[ "n", M.Int n ]
    ~controller
    "val"
;;

let counts controller n =
  let* program = M.assemble controller in
  let* _, st =
    Handsim.run program (Handsim.initial [ "n", M.Int n; "val", M.Int 0 ]) []
  in
  Ok (Printf.sprintf "steps=%d saves=%d" st.steps st.saves)
;;

let ex_5_06 () =
  let* before = run_fib Sec_5_5.fib_controller 10 in
  let* after = run_fib fib_modified_controller 10 in
  let* before_counts = counts Sec_5_5.fib_controller 6 in
  let* after_counts = counts fib_modified_controller 6 in
  Ok [ M.value_to_string before; M.value_to_string after; before_counts; after_counts ]
;;
