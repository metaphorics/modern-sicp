(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1

(* Each level saves [continue], the one register the subproblem
   clobbers. [b] needs no save: the subproblem never changes it. *)
let expt_recursive_controller =
  M.
    [ Assign ("continue", Label_ref "expt-done")
    ; Label "expt-loop"
    ; Test ("=", [ Reg "n"; Const (Int 0) ])
    ; Branch "base-case"
    ; Save "continue"
    ; Assign_op ("n", "-", [ Reg "n"; Const (Int 1) ])
    ; Assign ("continue", Label_ref "after-expt")
    ; Goto "expt-loop"
    ; Label "after-expt"
    ; Restore "continue"
    ; Assign_op ("val", "*", [ Reg "b"; Reg "val" ])
    ; Goto_reg "continue"
    ; Label "base-case"
    ; Assign ("val", Const (Int 1))
    ; Goto_reg "continue"
    ; Label "expt-done"
    ]
;;

let expt_iterative_controller =
  M.
    [ Label "expt-iter"
    ; Test ("=", [ Reg "counter"; Const (Int 0) ])
    ; Branch "expt-done"
    ; Assign_op ("counter", "-", [ Reg "counter"; Const (Int 1) ])
    ; Assign_op ("product", "*", [ Reg "b"; Reg "product" ])
    ; Goto "expt-iter"
    ; Label "expt-done"
    ]
;;

let recursive_expt b n =
  M.run
    ~registers:[ "b"; "n"; "val"; "continue" ]
    ~operations:M.arith_operations
    ~inputs:[ "b", M.Int b; "n", M.Int n ]
    ~controller:expt_recursive_controller
    "val"
;;

let iterative_expt b n =
  M.run
    ~registers:[ "b"; "counter"; "product" ]
    ~operations:M.arith_operations
    ~inputs:[ "b", M.Int b; "product", M.Int 1; "counter", M.Int n ]
    ~controller:expt_iterative_controller
    "product"
;;

let ex_5_04 () =
  let* r1 = recursive_expt 2 10 in
  let* r2 = recursive_expt 3 5 in
  let* i1 = iterative_expt 2 10 in
  let* i2 = iterative_expt 3 5 in
  Ok (List.map M.value_to_string [ r1; r2; i1; i2 ])
;;
