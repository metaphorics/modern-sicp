(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.26 *)

(** Exercise 3.26: a table whose records live in a binary tree. *)

open Sicp_ch3.Sec_3_3

type tree =
  | Leaf
  | Node of node

and node =
  { key : Mpairs.mobj
  ; mutable value : Mpairs.mobj
  ; mutable left : tree
  ; mutable right : tree
  }

type btable =
  { mutable root : tree
  ; key_compare : Mpairs.mobj -> Mpairs.mobj -> int
  }

(** [make_table ~key_compare] is an empty tree table ordered by
    [key_compare]. *)
val make_table : key_compare:(Mpairs.mobj -> Mpairs.mobj -> int) -> btable

val lookup : Mpairs.mobj -> btable -> Mpairs.mobj option
val insert : Mpairs.mobj -> Mpairs.mobj -> btable -> unit

(** [key_compare] orders integer and symbol keys; mixed key types are
    a programming error. *)
val key_compare : Mpairs.mobj -> Mpairs.mobj -> int

(** [ex_3_26 ()] is [(the values found under 10 30 50 70 80 after an
    out-of-order build, the value under 20 after an overwrite, and the
    lookup of the missing key 40)]. *)
val ex_3_26 : unit -> string * string * Mpairs.mobj option
