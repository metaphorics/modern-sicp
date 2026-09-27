(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.72 *)

(** Exercise 2.72: the order of growth of Exercise 2.68's
    [encode_symbol], including the symbol-list scan
    [List.mem]/[symbols] performs at every node. [encode_symbol_steps]
    counts one step per element the membership scan visits, plus one
    step per branch taken, so its count is exactly the cost the book
    asks about.

    For the skewed alphabet of Exercise 2.71 (frequencies
    [1, 2, ..., 2^(n - 1)]), [make_code_tree] always folds the
    lighter accumulated tree in on the left, so the accumulator's
    symbols lead every [symbols left @ symbols right] list; the
    lightest symbol, [s1], therefore sits first in the left-branch
    list at every one of the [n - 1] levels down to its leaf, and
    [List.mem] finds it in one step each time: [Theta(n)] overall.
    The heaviest symbol, [sn], sits alone one level down; finding it
    means first exhausting the size-[(n - 1)] left list to rule it
    out, so encoding it costs [Theta(n)] concentrated in that single
    step. Both therefore grow as [Theta(n)], by different
    mechanisms: one long negative scan for the frequent symbol,
    [n - 1] short positive ones for the rare symbol. *)

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

(** [encode_symbol_steps symbol tree] is the number of steps
    [encode_symbol symbol tree] takes: one step for every element
    [List.mem] examines while it decides whether to move left or
    right, and one further step for the move itself. *)
let rec encode_symbol_steps symbol tree =
  match tree with
  | Leaf (s, _) when s = symbol -> 1
  | Leaf _ -> invalid_arg "encode_symbol_steps: symbol not in tree"
  | Node (left, right, _, _) ->
    let in_left, left_scan = mem_steps symbol (symbols left) in
    if in_left
    then 1 + left_scan + encode_symbol_steps symbol left
    else (
      let in_right, right_scan = mem_steps symbol (symbols right) in
      if in_right
      then 1 + left_scan + right_scan + encode_symbol_steps symbol right
      else invalid_arg "encode_symbol_steps: symbol not in tree")

(** [mem_steps x items] is whether [x] is in [items], paired with the
    number of elements examined to decide. *)
and mem_steps x items =
  match items with
  | [] -> false, 0
  | first :: rest ->
    if first = x
    then true, 1
    else (
      let found, steps = mem_steps x rest in
      found, steps + 1)
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
let skewed_alphabet n = List.init n (fun i -> Printf.sprintf "s%d" (i + 1), 1 lsl i)
let skewed_tree n = generate_huffman_tree (skewed_alphabet n)

(** [ex_2_72 n] is the step count for the heaviest symbol paired with
    the step count for the lightest symbol of the skewed [n]-symbol
    alphabet. *)
let ex_2_72 n =
  let tree = skewed_tree n in
  encode_symbol_steps (Printf.sprintf "s%d" n) tree, encode_symbol_steps "s1" tree
;;
