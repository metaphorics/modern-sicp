(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.31 *)

(** Exercise 3.31: why [accept_action_procedure!] runs the new
    procedure once immediately. Without that first run, a device
    attached to a wire never sees the value the wire already carries:
    its own output is computed for the first time only at the next
    change, and until then the circuit is inconsistent. [ex_3_31] shows
    both worlds: an inverter on a settled 0-wire answers 1 when the
    action runs at attach, and keeps a stale 0 when it does not. *)

open Sicp_ch3.Sec_3_3
open Circuit
module C = Circuit

(* The variant the exercise proposes: no initial run. *)
let make_wire_without_initial_run () : C.wire =
  let signal_value = ref 0 in
  let action_procedures = ref [] in
  let set_my_signal new_value =
    if !signal_value <> new_value
    then (
      signal_value := new_value;
      List.iter (fun p -> p ()) !action_procedures)
  in
  let accept_action_procedure proc = action_procedures := proc :: !action_procedures in
  { get_signal = (fun () -> !signal_value)
  ; set_signal = set_my_signal
  ; add_action = accept_action_procedure
  }
;;

let ex_3_31 () =
  (* with the section's make-wire: the inverter's action runs once at
     attach and schedules the 1 *)
  let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 () in
  let input = C.make_wire () in
  let out = C.make_wire () in
  C.inverter sim input out;
  C.propagate sim;
  let with_initial_run = out.get_signal () in
  (* with the exercise's variant: nothing is scheduled at attach, and
     the input never changes, so the output stays stale at 0 *)
  let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 () in
  let input = make_wire_without_initial_run () in
  let out = C.make_wire () in
  C.inverter sim input out;
  C.propagate sim;
  let without_initial_run = out.get_signal () in
  with_initial_run, without_initial_run
;;
