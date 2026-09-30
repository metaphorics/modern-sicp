(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module M = Sicp_ch5.Sec_5_1

let ( let* ) = Result.bind
let fib = "let rec fib n = if n < 2 then n else fib (n - 1) + fib (n - 2)"

let fib_machine =
  [ M.Assign ("continue", M.Label_ref "fib-done")
  ; M.Label "fib-loop"
  ; M.Test ("<", [ M.Reg "n"; M.Const (M.Int 2) ])
  ; M.Branch "immediate-answer"
  ; M.Save "continue"
  ; M.Assign ("continue", M.Label_ref "afterfib-n-1")
  ; M.Save "n"
  ; M.Assign_op ("n", "-", [ M.Reg "n"; M.Const (M.Int 1) ])
  ; M.Goto "fib-loop"
  ; M.Label "afterfib-n-1"
  ; M.Restore "n"
  ; M.Restore "continue"
  ; M.Assign_op ("n", "-", [ M.Reg "n"; M.Const (M.Int 2) ])
  ; M.Save "continue"
  ; M.Assign ("continue", M.Label_ref "afterfib-n-2")
  ; M.Save "val"
  ; M.Goto "fib-loop"
  ; M.Label "afterfib-n-2"
  ; M.Assign ("n", M.Reg "val")
  ; M.Restore "val"
  ; M.Restore "continue"
  ; M.Assign_op ("val", "+", [ M.Reg "val"; M.Reg "n" ])
  ; M.Goto_reg "continue"
  ; M.Label "immediate-answer"
  ; M.Assign ("val", M.Reg "n")
  ; M.Goto_reg "continue"
  ; M.Label "fib-done"
  ]
;;

let ex_5_46 () =
  List.fold_right
    (fun n acc ->
       let* lines = acc in
       let* line =
         Sec_5_45.comparison ~definition:fib ~call:"fib" ~machine:fib_machine n
       in
       Ok (line :: lines))
    [ 5; 6; 7 ]
    (Ok [])
;;
