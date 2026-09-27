(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.8: one label naming two locations is a defect the
    assembler reports before the machine can start. *)

module Machine = Sicp_ch5.Sec_5_2

(** A controller whose label [again] is written twice: the two writes
    compete for the same name, and the book's ambiguity. *)
let ambiguous_controller =
  {|(controller
   (assign a (const 1))
 again
   (assign a (op +) (reg a) (reg a))
   (goto (label again))
 again
   (assign a (op -) (reg a) (const 1))
   (goto (label again)))|}
;;

(** [ex_5_08 ()] is the assembler's typed report for the ambiguous
    controller: the duplicate is named, nothing is assembled, and the
    machine is never started. *)
let ex_5_08 () =
  let report = function
    | Ok _ -> "assembled (the ambiguity went undetected)"
    | Error e -> "Error: " ^ Machine.error_to_string e
  in
  Machine.make_machine
    ~registers:[ "a" ]
    ~operations:Machine.arith_operations
    ~controller:ambiguous_controller
  |> fun outcome -> Ok [ report outcome ]
;;
