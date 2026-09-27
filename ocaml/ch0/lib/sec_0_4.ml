(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Pattern matching and variants: the named definitions behind the
    listings of section 0.4. *)

type shape =
  | Circle of float
  | Rectangle of float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
;;

let abs_value n =
  match n with
  | n when n < 0 -> -n
  | n -> n
;;

let head = function
  | [] -> None
  | first :: _ -> Some first
;;

let safe_divide n d = if d = 0 then None else Some (n / d)

type int_list =
  | Nil
  | Cons of int * int_list

let rec total = function
  | Nil -> 0
  | Cons (head, tail) -> head + total tail
;;

type tree =
  | Leaf
  | Node of tree * int * tree

let rec tree_sum = function
  | Leaf -> 0
  | Node (left, label, right) -> tree_sum left + label + tree_sum right
;;
