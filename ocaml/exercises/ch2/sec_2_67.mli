(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.67 *)

type bit =
  | Zero
  | One

type tree =
  | Leaf of string * int
  | Node of tree * tree * string list * int

val make_leaf : string -> int -> tree
val symbols : tree -> string list
val weight : tree -> int
val make_code_tree : tree -> tree -> tree
val left_branch : tree -> tree
val right_branch : tree -> tree
val choose_branch : bit -> tree -> tree

(** [decode bits tree] is the message [bits] names under [tree]. *)
val decode : bit list -> tree -> string list

(** The tree the statement defines: [make_code_tree (make_leaf "A" 4)
    (make_code_tree (make_leaf "B" 2) (make_code_tree (make_leaf "D" 1)
    (make_leaf "C" 1)))]. *)
val sample_tree : tree

val sample_message : bit list

(** [ex_2_67 ()] is [sample_message] decoded against [sample_tree]. *)
val ex_2_67 : unit -> string list
