(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.68 *)

(** Exercise 2.68: [encode_symbol] and [encode]. [encode_symbol]
    walks toward the leaf holding [symbol], recording a [Zero] for
    every left turn and a [One] for every right turn, guided by
    whether [symbol] appears in the left or the right branch's symbol
    list; [invalid_arg] when [symbol] is in neither. *)

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

let sample_tree =
  make_code_tree
    (make_leaf "A" 4)
    (make_code_tree
       (make_leaf "B" 2)
       (make_code_tree (make_leaf "D" 1) (make_leaf "C" 1)))
;;

let sample_message =
  [ Zero; One; One; Zero; Zero; One; Zero; One; Zero; One; One; One; Zero ]
;;

(** [ex_2_68 ()] is Exercise 2.67's decoded message encoded again
    against [sample_tree], to check it matches [sample_message]. *)
let ex_2_68 () = encode (decode sample_message sample_tree) sample_tree
