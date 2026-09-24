(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.22 *)

(** Exercise 3.22: the queue as a record of closures over local pointer
    cells. *)

open Sicp_ch3.Sec_3_3.Mpairs

type queue_object =
  { q_empty : unit -> bool
  ; q_front : unit -> mobj
  ; q_insert : mobj -> unit
  ; q_delete : unit -> unit
  }

(** [make_queue_object ()] is a fresh queue object; each call allocates
    its own front and rear cells. *)
val make_queue_object : unit -> queue_object

(** [ex_3_22 ()] is [(the items shown for q1 after insert a, insert b,
    delete, whether q1 is then empty, the front of a second, fresh
    queue after its own insert, whether that queue is empty)]. *)
val ex_3_22 : unit -> string * bool * string * bool
