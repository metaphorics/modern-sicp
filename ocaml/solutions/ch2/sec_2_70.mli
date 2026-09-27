(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.70 *)

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
val adjoin_set : tree -> tree list -> tree list
val make_leaf_set : (string * int) list -> tree list
val successive_merge : tree list -> tree
val generate_huffman_tree : (string * int) list -> tree

(** The eight-symbol alphabet designed for 1950s rock lyrics. *)
val rock_pairs : (string * int) list

(** The song, one symbol per word. *)
val rock_song : string list

val rock_tree : tree

(** [fixed_length_bits alphabet_size message_length] is the bit count
    a fixed-length code over [alphabet_size] symbols needs to encode
    [message_length] symbols. *)
val fixed_length_bits : int -> int -> int

(** [ex_2_70 ()] is the Huffman-encoded bit count for [rock_song],
    paired with the fixed-length bit count over the same alphabet. *)
val ex_2_70 : unit -> int * int

(** [roundtrip tree message] holds when encoding then decoding
    [message] against [tree] gives back [message]. *)
val roundtrip : tree -> string list -> bool

(** [ex_2_70a ()] (this edition's addition, extending 2.70) checks
    [roundtrip] against a deterministic battery of trees and
    messages. *)
val ex_2_70a : unit -> bool
