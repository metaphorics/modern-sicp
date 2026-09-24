(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.13 *)

(** Exercise 3.13 (and the edition's addition 3.13a): [make_cycle]
    closes the last pair onto the first; the addition prints cyclic
    structures without diverging. *)

(** [make_cycle x] is [x] after its last pair's [cdr] has been set to
    the first pair of [x] itself. *)

(** [last_pair_bounded n x] is the last pair of [x], or [None] when the
    walk has taken more than [n] cdrs -- the bounded stand-in for the
    book's [last-pair], which diverges on a cycle. *)

(** [ex_3_13 ()] is [(whether a many-step walk around the cycle of
    (a b c), stepping a multiple of the ring's length, lands back on
    the first pair, whether [last_pair_bounded 100] gets no answer on
    it)]. *)

(** [show_cycle o] is the book's printed notation for [o], with any
    re-entered pair shown as [#cycle] instead of being walked again;
    [fuel] bounds the number of pairs visited. *)

(** [ex_3_13a ()] is [(the printed form of the (a b c) ring, of the
    (a b) ring, and of the plain list (a b))]. *)
let make_cycle = raise Sicp_common.Pending.Pending_solution

let last_pair_bounded = raise Sicp_common.Pending.Pending_solution
let ex_3_13 = raise Sicp_common.Pending.Pending_solution
let show_cycle = raise Sicp_common.Pending.Pending_solution
let ex_3_13a = raise Sicp_common.Pending.Pending_solution
