(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.71 *)

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
val encode_symbol : string -> tree -> bit list
val adjoin_set : tree -> tree list -> tree list
val make_leaf_set : (string * int) list -> tree list
val successive_merge : tree list -> tree
val generate_huffman_tree : (string * int) list -> tree

(** [skewed_alphabet n] is [n] symbols [s1] through [sn] with relative
    frequencies [1, 2, 4, ..., 2^(n - 1)]. *)
val skewed_alphabet : int -> (string * int) list

(** [skewed_tree n] is the Huffman tree [skewed_alphabet n] generates. *)
val skewed_tree : int -> tree

(** [ex_2_71 n] is the bit count for the heaviest symbol paired with
    the bit count for the lightest symbol of the skewed [n]-symbol
    alphabet. *)
val ex_2_71 : int -> int * int
