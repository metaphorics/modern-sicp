(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the register machines of 5.1 as typed
   controllers, run on the simulator, each result proved with
   [expect]. *)

module Machine = Sicp_ch5.Sec_5_1
module Replay = Sicp_ch1.Replay
open Machine

let show = function
  | Ok v -> value_to_string v
  | Error e -> "error: " ^ Sicp_common.Eval_error.to_string e
;;

(* Figure 5.4: the GCD machine. *)
let gcd_controller =
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

(* Figure 5.6: remainder computed by repeated subtraction. *)
let gcd_subtraction_controller =
  [ Label "test-b"
  ; Test ("=", [ Reg "b"; Const (Int 0) ])
  ; Branch "gcd-done"
  ; Assign ("t", Reg "a")
  ; Label "rem-loop"
  ; Test ("<", [ Reg "t"; Reg "b" ])
  ; Branch "rem-done"
  ; Assign_op ("t", "-", [ Reg "t"; Reg "b" ])
  ; Goto "rem-loop"
  ; Label "rem-done"
  ; Assign ("a", Reg "b")
  ; Assign ("b", Reg "t")
  ; Goto "test-b"
  ; Label "gcd-done"
  ]
;;

(* Figure 5.11: recursive factorial with the stack and [continue]. *)
let factorial_controller =
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

let gcd controller a b =
  run
    ~registers:[ "a"; "b"; "t" ]
    ~operations:arith_operations
    ~inputs:[ "a", Int a; "b", Int b ]
    ~controller
    "a"
;;

let () =
  Replay.expect (show (gcd gcd_controller 206 40)) "2";
  Replay.expect (show (gcd gcd_subtraction_controller 206 40)) "2";
  Replay.expect
    (show
       (run
          ~registers:[ "n"; "val"; "continue" ]
          ~operations:arith_operations
          ~inputs:[ "n", Int 5 ]
          ~controller:factorial_controller
          "val"))
    "120";
  (* The machine refuses an undeclared register before it runs. *)
  Replay.expect
    (show
       (run
          ~registers:[ "a" ]
          ~operations:arith_operations
          ~inputs:[]
          ~controller:gcd_controller
          "a"))
    "error: unknown register b"
;;
