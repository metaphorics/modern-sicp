(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.32 *)

(** Exercise 3.32: why a time segment's procedures must run first in,
    first out. The statement's case is an and-gate whose inputs change
    from (0, 1) to (1, 0) in the same segment: both changes schedule a
    set of the output wire into the same later segment, and only FIFO
    applies them in causal order. The same scenario runs on the
    section's agenda and on the stack-segmented agenda defined here. *)

open Sicp_ch3.Sec_3_3
open Circuit
module C = Circuit

(* A minimal agenda whose segments are stacks: add pushes at the front,
   remove pops at the front -- last in, first out. *)
type lifo_sim =
  { mutable now : int
  ; mutable segs : (int * (unit -> unit) list ref) list
  }

let make_lifo () = { now = 0; segs = [] }

let lifo_add sim time action =
  let rec insert = function
    | [] -> [ time, ref [ action ] ]
    | (t, stack) :: rest as segs ->
      if t = time
      then (
        stack := action :: !stack;
        segs)
      else if time < t
      then (time, ref [ action ]) :: segs
      else (t, stack) :: insert rest
  in
  sim.segs <- insert sim.segs
;;

let rec lifo_propagate sim =
  match sim.segs with
  | [] -> ()
  | (t, stack) :: rest ->
    sim.now <- t;
    sim.segs <- rest;
    let items = !stack in
    stack := [];
    List.iter (fun action -> action ()) items;
    (* anything scheduled while this segment ran joins a later one *)
    lifo_propagate sim
;;

let lifo_and_gate sim a1 a2 output =
  let and_action () =
    let nv = if a1.get_signal () = 1 && a2.get_signal () = 1 then 1 else 0 in
    lifo_add sim (sim.now + 3) (fun () -> output.set_signal nv)
  in
  a1.add_action and_action;
  a2.add_action and_action
;;

(* Settle (0, 1), then change to (1, 0) in one segment; answer the
   output after the propagation. *)
let scenario () =
  let a = C.make_wire () in
  let b = C.make_wire () in
  let out = C.make_wire () in
  let changed () =
    a.set_signal 1;
    b.set_signal 0;
    out.get_signal ()
  in
  a, b, out, changed
;;

let fifo_result () =
  let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 () in
  let a, b, out, changed = scenario () in
  C.and_gate sim a b out;
  a.set_signal 0;
  b.set_signal 1;
  C.propagate sim;
  let _ = changed in
  a.set_signal 1;
  b.set_signal 0;
  C.propagate sim;
  out.get_signal ()
;;

let lifo_result () =
  let sim = make_lifo () in
  let a = C.make_wire () in
  let b = C.make_wire () in
  let out = C.make_wire () in
  lifo_and_gate sim a b out;
  a.set_signal 0;
  b.set_signal 1;
  lifo_propagate sim;
  a.set_signal 1;
  b.set_signal 0;
  lifo_propagate sim;
  out.get_signal ()
;;

let ex_3_32 () = fifo_result (), lifo_result ()
