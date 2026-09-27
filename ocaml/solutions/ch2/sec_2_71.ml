(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.71 *)

(** Exercise 2.71: the skewed tree of an alphabet whose [n] symbols
    have relative frequencies [1, 2, 4, ..., 2^(n - 1)]. Every merge
    combines the single heaviest tree built so far with the next
    symbol, which is always at least as heavy, so the result is a
    single long spine: the heaviest symbol sits one level down, and
    the lightest sits [n - 1] levels down. Encoding the most frequent
    symbol therefore always takes 1 bit, and the least frequent always
    takes [n - 1] bits, whatever [n] is. *)

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

(** [skewed_alphabet n] is [n] symbols named [s1] through [sn] with
    relative frequencies [1, 2, 4, ..., 2^(n - 1)]: [sn] is the
    heaviest. *)
let skewed_alphabet n = List.init n (fun i -> Printf.sprintf "s%d" (i + 1), 1 lsl i)

let skewed_tree n = generate_huffman_tree (skewed_alphabet n)

(** [ex_2_71 n] is the number of bits [encode_symbol] needs for the
    heaviest symbol and for the lightest symbol of the skewed
    [n]-symbol alphabet. *)
let ex_2_71 n =
  let tree = skewed_tree n in
  let heaviest = Printf.sprintf "s%d" n in
  let lightest = "s1" in
  List.length (encode_symbol heaviest tree), List.length (encode_symbol lightest tree)
;;
