(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.72 *)

type tree =
  | Leaf of string * int
  | Node of tree * tree * string list * int

val make_leaf : string -> int -> tree
val symbols : tree -> string list
val weight : tree -> int
val make_code_tree : tree -> tree -> tree

(** [encode_symbol_steps symbol tree] is the number of steps
    encoding [symbol] against [tree] takes, counting every element a
    symbol-list membership test examines and every branch taken.
    [invalid_arg] when [symbol] is not in [tree]. *)
val encode_symbol_steps : string -> tree -> int

(** [mem_steps x items] is whether [x] is in [items], paired with the
    number of elements examined to decide. *)
val mem_steps : string -> string list -> bool * int

val adjoin_set : tree -> tree list -> tree list
val make_leaf_set : (string * int) list -> tree list
val successive_merge : tree list -> tree
val generate_huffman_tree : (string * int) list -> tree
val skewed_alphabet : int -> (string * int) list
val skewed_tree : int -> tree

(** [ex_2_72 n] is the step count for the heaviest symbol paired with
    the step count for the lightest symbol of the skewed [n]-symbol
    alphabet. *)
val ex_2_72 : int -> int * int
