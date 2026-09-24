(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.22 *)

(** Exercise 3.22: the queue as an object with local state -- a record
    of closures over the two pointer cells, instead of a pair handed
    around. *)

open Sicp_ch3.Sec_3_3.Mpairs

type queue_object =
  { q_empty : unit -> bool
  ; q_front : unit -> mobj
  ; q_insert : mobj -> unit
  ; q_delete : unit -> unit
  }

let make_queue_object () =
  let front_ptr = ref Nil in
  let rear_ptr = ref Nil in
  let q_insert item =
    let new_pair = { car = item; cdr = Nil } in
    if Nil = !front_ptr
    then (
      front_ptr := Pair new_pair;
      rear_ptr := Pair new_pair)
    else (
      set_cdr !rear_ptr (Pair new_pair);
      rear_ptr := Pair new_pair)
  in
  let q_delete () =
    if Nil = !front_ptr
    then invalid_arg "DELETE called with an empty queue"
    else front_ptr := cdr !front_ptr
  in
  { q_empty = (fun () -> Nil = !front_ptr)
  ; q_front = (fun () -> car !front_ptr)
  ; q_insert
  ; q_delete
  }
;;

let ex_3_22 () =
  let q1 = make_queue_object () in
  q1.q_insert (msym "a");
  q1.q_insert (msym "b");
  q1.q_delete ();
  let q1_items = [ q1.q_front () ] in
  let q1_shown = String.concat " " (List.map (fun o -> show o) q1_items) in
  let q2 = make_queue_object () in
  q2.q_insert (msym "c");
  (* separate objects, separate cells: q2's traffic leaves q1 alone *)
  q1_shown, q1.q_empty (), show (q2.q_front ()), q2.q_empty ()
;;
