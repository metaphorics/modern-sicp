(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.66 *)

type record =
  { key : int
  ; name : string
  }

type tree =
  | Empty
  | Node of tree * record * tree

(** [lookup given_key tree] is the record whose key is [given_key], or
    [None] when no record in [tree] has it. *)
val lookup : int -> tree -> record option

val sample_tree : tree

(** [ex_2_66 ()] is [lookup 3] against [sample_tree]. *)
val ex_2_66 : unit -> record option
