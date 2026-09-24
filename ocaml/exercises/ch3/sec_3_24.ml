(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.24 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.24: a table keyed by a caller-supplied equality. *)
open Sicp_ch3.Sec_3_3

type table =
  { lookup : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj option
  ; insert : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj -> unit
  }

(** [make_table same_key] is a two-dimensional table object whose keys
    are tested with [same_key]. *)

(** [near tol] is a [same_key] for integer keys: two keys match when
    they differ by at most [tol]. *)

(** [ex_3_24 ()] is [(the value under key 10 named "a", the lookup with
    key 11 one step off, the lookup with key 12 two steps off, the
    value under key 21 named "b")], the tolerance being 1. *)
let make_table _same_key = raise Sicp_common.Pending.Pending_solution

let near _tol _x _y = raise Sicp_common.Pending.Pending_solution
let ex_3_24 () = raise Sicp_common.Pending.Pending_solution
