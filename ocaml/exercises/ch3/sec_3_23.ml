(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.23 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

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

(** [items d] is the deque's items, front to rear; a testing observer
    only, since every mutation above runs in [O(1)] steps. *)

(** [ex_3_23 ()] is [(the items after rear-insert b, front-insert a,
    rear-insert c, the items after front-delete and rear-delete, the
    front, the rear, whether the deque is empty)] -- each operation a
    constant number of pointer writes. *)
let make_deque () = raise Sicp_common.Pending.Pending_solution

let empty_deque _d = raise Sicp_common.Pending.Pending_solution
let front_deque _d = raise Sicp_common.Pending.Pending_solution
let rear_deque _d = raise Sicp_common.Pending.Pending_solution
let front_insert_deque _d _x = raise Sicp_common.Pending.Pending_solution
let rear_insert_deque _d _x = raise Sicp_common.Pending.Pending_solution
let front_delete_deque _d = raise Sicp_common.Pending.Pending_solution
let rear_delete_deque _d = raise Sicp_common.Pending.Pending_solution
let items _d = raise Sicp_common.Pending.Pending_solution
let ex_3_23 () = raise Sicp_common.Pending.Pending_solution
