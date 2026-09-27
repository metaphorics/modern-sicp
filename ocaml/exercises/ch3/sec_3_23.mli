(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.23 *)

(** Exercise 3.23: a double-ended queue in constant time at both ends. *)

open Sicp_ch3.Sec_3_3.Mpairs

type deque =
  { mutable front : node option
  ; mutable rear : node option
  }

and node =
  { mutable prev : node option
  ; mutable item : mobj
  ; mutable next : node option
  }

val make_deque : unit -> deque
val empty_deque : deque -> bool
val front_deque : deque -> mobj
val rear_deque : deque -> mobj
val front_insert_deque : deque -> mobj -> unit
val rear_insert_deque : deque -> mobj -> unit
val front_delete_deque : deque -> unit
val rear_delete_deque : deque -> unit

(** [items d] is the deque's items, front to rear; a testing observer
    only, since every mutation above runs in [O(1)] steps. *)
val items : deque -> mobj list

(** [ex_3_23 ()] is [(the items after rear-insert b, front-insert a,
    rear-insert c, the items after front-delete and rear-delete, the
    front, the rear, whether the deque is empty)] -- each operation a
    constant number of pointer writes. *)
val ex_3_23 : unit -> string * string * string * string * bool
