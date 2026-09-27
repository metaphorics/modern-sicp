(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.28 *)

(** Exercise 3.28: the or-gate as a primitive function box, mirroring
    the and-gate of the main text. [ex_3_28] drives all four input
    combinations through one gate and checks the output value at the
    moment the or-gate delay elapses. *)

open Sicp_ch3.Sec_3_3
open Circuit
module C = Circuit

let logical_or a b = if a = 1 || b = 1 then 1 else 0

let or_gate sim a1 a2 output =
  let or_action_procedure () =
    let new_value = logical_or (a1.get_signal ()) (a2.get_signal ()) in
    C.after_delay sim sim.or_gate_delay (fun () -> output.set_signal new_value)
  in
  a1.add_action or_action_procedure;
  a2.add_action or_action_procedure
;;

(* Drive [a] and [b] to the given values, run the agenda, and answer
   the signal on [out] with the transcript length as a witness. *)
let drive va vb =
  let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 () in
  let a = C.make_wire () in
  let b = C.make_wire () in
  let out = C.make_wire () in
  or_gate sim a b out;
  C.probe sim "out" out;
  a.set_signal va;
  b.set_signal vb;
  C.propagate sim;
  let lines = List.rev !(sim.trace) in
  let out_value = out.get_signal () in
  out_value, lines
;;

let ex_3_28 () =
  let out00, _ = drive 0 0 in
  let out01, _ = drive 0 1 in
  let out10, _ = drive 1 0 in
  let out11, _ = drive 1 1 in
  ( (out00, out01, out10, out11)
  , logical_or 0 0
  , logical_or 1 0
  , logical_or 0 1
  , logical_or 1 1 )
;;
