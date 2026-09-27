(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.67 *)

(** Exercise 2.67: decode a sample message with a hand-built encoding
    tree. The tree and the machinery it needs are copied here rather
    than shared from [Sicp_ch2.Sec_2_3.Huffman], the way every section
    2.3 exercise is self-contained. *)

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

(** The tree the statement defines: [A] weighs 4, [B] weighs 2, and
    [C] and [D] each weigh 1. *)
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

(** [ex_2_67 ()] is [sample_message] decoded against [sample_tree]. *)
let ex_2_67 () = decode sample_message sample_tree
