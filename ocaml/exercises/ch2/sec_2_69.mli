(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.69 *)

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
val decode : bit list -> tree -> string list
val encode_symbol : string -> tree -> bit list
val encode : string list -> tree -> bit list

(** [adjoin_set x set] inserts the leaf or tree [x] into the
    weight-ordered [set]. *)
val adjoin_set : tree -> tree list -> tree list

(** [make_leaf_set pairs] is the ordered set of leaves for the
    symbol-frequency pairs [pairs]. *)
val make_leaf_set : (string * int) list -> tree list

(** [successive_merge set] merges the two lightest elements of the
    weight-ordered [set] until one tree remains. [invalid_arg] on an
    empty set. *)
val successive_merge : tree list -> tree

(** [generate_huffman_tree pairs] is the Huffman tree the algorithm
    builds from the symbol-frequency pairs [pairs]. *)
val generate_huffman_tree : (string * int) list -> tree

(** [ex_2_69 ()] is the tree the statement's pairs generate. *)
val ex_2_69 : unit -> tree
