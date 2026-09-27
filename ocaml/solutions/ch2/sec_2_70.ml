(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.70 *)

(** Exercise 2.70: encode the rock-song message with the Huffman tree
    the eight-symbol alphabet generates, and compare its length with a
    fixed-length code's.

    Exercise 2.70a (this edition's addition, extending 2.70): decode
    and encode are meant to invert each other for any tree the
    algorithm can build and any message drawn from its alphabet. A
    handful of examples cannot show that; [roundtrip] states the
    property so that [Prop_sec_2_70] can check it against many
    randomly generated trees and messages, the way a single example
    never could. *)

type bit =
  | Zero
  | One

type tree =
  | Leaf of string * int
  | Node of tree * tree * string list * int

let make_leaf symbol weight = Leaf (symbol, weight)

let symbols = function
  | Leaf (s, _) -> [ s ]
  | Node (_, _, ss, _) -> ss
;;

let weight = function
  | Leaf (_, w) -> w
  | Node (_, _, _, w) -> w
;;

let make_code_tree left right =
  Node (left, right, symbols left @ symbols right, weight left + weight right)
;;

let left_branch = function
  | Node (l, _, _, _) -> l
  | Leaf _ -> invalid_arg "left_branch: a leaf has no branches"
;;

let right_branch = function
  | Node (_, r, _, _) -> r
  | Leaf _ -> invalid_arg "right_branch: a leaf has no branches"
;;

let choose_branch bit branch =
  match bit with
  | Zero -> left_branch branch
  | One -> right_branch branch
;;

let decode bits tree =
  let rec decode_1 bits current_branch =
    match bits with
    | [] -> []
    | bit :: rest ->
      (match choose_branch bit current_branch with
       | Leaf (symbol, _) -> symbol :: decode_1 rest tree
       | next_branch -> decode_1 rest next_branch)
  in
  decode_1 bits tree
;;

let rec encode_symbol symbol tree =
  match tree with
  | Leaf (s, _) when s = symbol -> []
  | Leaf _ -> invalid_arg "encode_symbol: symbol not in tree"
  | Node (left, right, _, _) ->
    if List.mem symbol (symbols left)
    then Zero :: encode_symbol symbol left
    else if List.mem symbol (symbols right)
    then One :: encode_symbol symbol right
    else invalid_arg "encode_symbol: symbol not in tree"
;;

let rec encode message tree =
  match message with
  | [] -> []
  | symbol :: rest -> encode_symbol symbol tree @ encode rest tree
;;

let rec adjoin_set x = function
  | [] -> [ x ]
  | first :: rest as set ->
    if weight x < weight first then x :: set else first :: adjoin_set x rest
;;

let rec make_leaf_set pairs =
  match pairs with
  | [] -> []
  | (symbol, frequency) :: rest ->
    adjoin_set (make_leaf symbol frequency) (make_leaf_set rest)
;;

let rec successive_merge = function
  | [] -> invalid_arg "successive_merge: an empty leaf set has no tree"
  | [ only ] -> only
  | first :: second :: rest ->
    successive_merge (adjoin_set (make_code_tree first second) rest)
;;

let generate_huffman_tree pairs = successive_merge (make_leaf_set pairs)

(** The eight-symbol alphabet designed for 1950s rock lyrics. *)
let rock_pairs =
  [ "A", 2; "BOOM", 1; "GET", 2; "JOB", 2; "NA", 16; "SHA", 3; "WAH", 1; "YIP", 9 ]
;;

(** The song, one symbol per word. *)
let rock_song =
  List.concat
    [ [ "GET"; "A"; "JOB"; "SHA" ]
    ; List.init 8 (fun _ -> "NA")
    ; [ "GET"; "A"; "JOB"; "SHA" ]
    ; List.init 8 (fun _ -> "NA")
    ; [ "WAH" ]
    ; List.init 9 (fun _ -> "YIP")
    ; [ "SHA"; "BOOM" ]
    ]
;;

let rock_tree = generate_huffman_tree rock_pairs

(** [fixed_length_bits alphabet_size message_length] is the number of
    bits a fixed-length code over [alphabet_size] symbols needs to
    encode [message_length] symbols: [ceil (log2 alphabet_size)] bits
    per symbol, times the message length. *)
let fixed_length_bits alphabet_size message_length =
  let rec bits_per_symbol n = if n <= 1 then 0 else 1 + bits_per_symbol ((n + 1) / 2) in
  bits_per_symbol alphabet_size * message_length
;;

(** [ex_2_70 ()] is the Huffman-encoded bit count for [rock_song],
    paired with the bit count a fixed-length code over the eight-symbol
    alphabet would need. *)
let ex_2_70 () =
  ( List.length (encode rock_song rock_tree)
  , fixed_length_bits (List.length rock_pairs) (List.length rock_song) )
;;

(** [roundtrip tree message] holds when encoding [message] against
    [tree] and decoding the result gives back [message]. Every symbol
    of [message] must appear in [tree]. *)
let roundtrip tree message = decode (encode message tree) tree = message

(** [ex_2_70a ()] checks [roundtrip] against a deterministic battery:
    the rock song and its tree, and Exercise 2.67's sample tree with a
    message drawn from its own symbols. *)
let ex_2_70a () =
  roundtrip rock_tree rock_song
  && roundtrip
       (generate_huffman_tree [ "A", 4; "B", 2; "C", 1; "D", 1 ])
       [ "A"; "B"; "C"; "D"; "A"; "A" ]
;;
