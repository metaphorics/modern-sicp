(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.24 *)

(** Exercise 3.24: a table keyed by a caller-supplied equality. *)

open Sicp_ch3.Sec_3_3

type table =
  { lookup : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj option
  ; insert : Mpairs.mobj -> Mpairs.mobj -> Mpairs.mobj -> unit
  }

(** [make_table same_key] is a two-dimensional table object whose keys
    are tested with [same_key]. *)
val make_table : (Mpairs.mobj -> Mpairs.mobj -> bool) -> table

(** [near tol] is a [same_key] for integer keys within [tol] of each
    other, falling back to exact matching for symbol keys so it also
    serves the second level of a two-dimensional table. *)
val near : int -> Mpairs.mobj -> Mpairs.mobj -> bool

(** [ex_3_24 ()] is [(the value under key 10 named "a", the lookup with
    key 11 one step off, the lookup with key 12 two steps off, the
    value under key 21 named "b")], the tolerance being 1. *)
val ex_3_24
  :  unit
  -> Mpairs.mobj option * Mpairs.mobj option * Mpairs.mobj option * string
