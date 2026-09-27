(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.68 *)

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

(** [encode_symbol symbol tree] is the bit sequence [tree] assigns to
    [symbol]. [invalid_arg] when [symbol] is not in [tree]. *)
val encode_symbol : string -> tree -> bit list

(** [encode message tree] is the bit sequence [tree] assigns to
    [message], one symbol's encoding after another. *)
val encode : string list -> tree -> bit list

val sample_tree : tree
val sample_message : bit list

(** [ex_2_68 ()] is Exercise 2.67's message decoded then encoded
    again, to check the round trip matches [sample_message]. *)
val ex_2_68 : unit -> bit list
