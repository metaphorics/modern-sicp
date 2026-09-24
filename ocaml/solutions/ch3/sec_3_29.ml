(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.29 *)

(** Exercise 3.29: the or-gate built from two inverters and an and-gate,
    by de Morgan: not (not a and not b) = a or b. The critical path is
    inverter, and-gate, inverter, so the delay is twice the inverter
    delay plus one and-gate delay. *)

open Sicp_ch3.Sec_3_3
module C = Circuit

let or_gate sim a b out =
  let na = C.make_wire () in
  let nb = C.make_wire () in
  let both = C.make_wire () in
  C.inverter sim a na;
  C.inverter sim b nb;
  C.and_gate sim na nb both;
  C.inverter sim both out
;;

(* The delay of the compound or-gate, in time units, read off the
   transcript: the output probe's time stamp. *)
let measure_delay va vb =
  let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:99 () in
  let a = C.make_wire () in
  let b = C.make_wire () in
  let out = C.make_wire () in
  or_gate sim a b out;
  C.probe sim "out" out;
  a.set_signal va;
  b.set_signal vb;
  C.propagate sim;
  let lines = !(sim.trace) in
  match lines with
  | [] -> 0
  | newest :: _ ->
    (* "out <time>  New-value = <v>" -- the fields are separated by
       single or double spaces, so empty tokens from the split are
       dropped before reading the second field. *)
    (match List.filter (fun s -> s <> "") (String.split_on_char ' ' newest) with
     | [ _; t; _; _; _ ] -> int_of_string t
     | _ -> 0)
;;

let ex_3_29 () =
  let t10 = measure_delay 1 0 in
  let t01 = measure_delay 0 1 in
  let expected = 2 + 3 + 2 in
  (* behavior matches the primitive or-gate on all four inputs *)
  let behaviors =
    List.map
      (fun (va, vb) ->
         let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 () in
         let a = C.make_wire () in
         let b = C.make_wire () in
         let out = C.make_wire () in
         or_gate sim a b out;
         a.set_signal va;
         b.set_signal vb;
         C.propagate sim;
         out.get_signal ())
      [ 0, 0; 0, 1; 1, 0; 1, 1 ]
  in
  behaviors, t10, t01, expected
;;
