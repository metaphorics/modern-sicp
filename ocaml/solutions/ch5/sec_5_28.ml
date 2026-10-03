(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Eval = Sicp_ch5.Sec_5_4

let r name = M.Reg name
let dispatch = M.Goto "eval-dispatch"

(* The 5.4.2 footnote's sequence: the last expression, too, runs as a
   subproblem across a saved [continue]. *)
let naive_ev_sequence =
  [ M.Label "ev-sequence"
  ; M.Save "exp"
  ; M.Save "env"
  ; M.Save "continue"
  ; M.Assign ("continue", M.Label_ref "ev-sequence-rest")
  ; M.Assign_op ("exp", "first-expression", [ r "exp" ])
  ; dispatch
  ; M.Label "ev-sequence-rest"
  ; M.Restore "continue"
  ; M.Restore "env"
  ; M.Restore "exp"
  ; M.Assign_op ("exp", "second-expression", [ r "exp" ])
  ; M.Save "continue"
  ; M.Assign ("continue", M.Label_ref "ev-sequence-last-done")
  ; dispatch
  ; M.Label "ev-sequence-last-done"
  ; M.Restore "continue"
  ; M.Goto_reg "continue"
  ]
;;

(* A procedure body is the subset's last expression of a sequence: the
   same subproblem shape, so no call is in tail position.  It mirrors
   the naive sequence above: restore the caller's [continue], then run
   the body across it saved again. *)
let naive_compound_body =
  [ M.Label "compound-tail"
  ; M.Restore "continue"
  ; M.Save "continue"
  ; M.Assign ("continue", M.Label_ref "compound-body-done")
  ; dispatch
  ; M.Label "compound-body-done"
  ; M.Restore "continue"
  ; M.Goto_reg "continue"
  ]
;;

let controller =
  Eval.base_controller
  |> Sec_5_23.splice ~from:"ev-sequence" ~until:"ev-and" naive_ev_sequence
  |> Sec_5_23.splice ~from:"compound-tail" ~until:"compound-partial" naive_compound_body
;;

let grows samples =
  match Sec_5_26.fit_linear samples with
  | Some ((a, _) as fit) -> a > 0 && Sec_5_26.holds fit samples
  | None -> false
;;

let ex_5_28 () =
  let ns = [ 1; 2; 3; 4; 5 ] in
  let* iterative = Sec_5_26.measure ~controller Sec_5_26.iterative_source ns in
  let* recursive = Sec_5_26.measure ~controller Sec_5_27.recursive_source ns in
  Ok
    (List.map (Sec_5_26.render_point "non-tail iterative factorial") iterative
     @ List.map (Sec_5_26.render_point "non-tail recursive factorial") recursive
     @ [ Sec_5_26.formula_line "iterative maximum depth" (Sec_5_26.depths iterative)
       ; Sec_5_26.formula_line "recursive maximum depth" (Sec_5_26.depths recursive)
       ; Printf.sprintf
           "iterative maximum depth now grows with n: %b"
           (grows (Sec_5_26.depths iterative))
       ])
;;
