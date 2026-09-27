(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.69 *)

(** Exercise 2.69: [successive_merge] repeatedly takes the two
    lightest elements off the front of the weight-ordered leaf set,
    merges them into one code tree, and uses [adjoin_set] to put the
    merge back in its ordered place, until one tree is left. Because
    the set stays ordered, the two lightest elements are always its
    first two, so no search for them is needed. *)

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

(** [ex_2_69 ()] is the Huffman tree the statement's pairs generate:
    the same tree Exercise 2.67 hand-builds. *)
let ex_2_69 () = generate_huffman_tree [ "A", 4; "B", 2; "C", 1; "D", 1 ]
