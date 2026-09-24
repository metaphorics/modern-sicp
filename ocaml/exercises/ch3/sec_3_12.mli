(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.12 *)

(** Exercise 3.12: [append] builds a fresh chain of pairs, [append_bang]
    splices [y] onto the last pair of [x]. [ex_3_12] replays the
    statement's interaction and answers the two missing responses. *)

open Sicp_ch3.Sec_3_3.Mpairs

(** [append_bang x y] is [x] after the [cdr] of its last pair has been
    set to [y]; the empty [x] is a programming error. *)
val append_bang : mobj -> mobj -> mobj

(** [ex_3_12 ()] is
    [(the value printed for z, the first missing response of (cdr x),
    the value printed for w, the second missing response of (cdr x))]. *)
val ex_3_12 : unit -> string * string * string * string
