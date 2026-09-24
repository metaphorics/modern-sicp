(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.12 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.12: [append] builds a fresh chain of pairs, [append_bang]
    splices [y] onto the last pair of [x]. [ex_3_12] replays the
    statement's interaction and answers the two missing responses. *)

(** [append_bang x y] is [x] after the [cdr] of its last pair has been
    set to [y]; the empty [x] is a programming error. *)

(** [ex_3_12 ()] is
    [(the value printed for z, the first missing response of (cdr x),
    the value printed for w, the second missing response of (cdr x))]. *)
let append_bang _x _y = raise Sicp_common.Pending.Pending_solution

let ex_3_12 () = raise Sicp_common.Pending.Pending_solution
