(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.26 *)

(** Exercise 3.26: the (key, value) records of a table organized as a
    binary tree, compared by a caller-supplied key order. Nodes are
    mutable records, so an insert that extends a leaf rewrites a
    constant number of pointers -- the table stays a mutable object.
    (Compare exercise 2.66, where the tree was built by hand out of
    immutable pairs.) *)

open Sicp_ch3.Sec_3_3
module M = Mpairs

type tree =
  | Leaf
  | Node of node

and node =
  { key : M.mobj
  ; mutable value : M.mobj
  ; mutable left : tree
  ; mutable right : tree
  }

type btable =
  { mutable root : tree
  ; key_compare : M.mobj -> M.mobj -> int
  }

let make_table ~key_compare = { root = Leaf; key_compare }

let lookup key t =
  let rec go = function
    | Leaf -> None
    | Node n ->
      (match t.key_compare key n.key with
       | 0 -> Some n.value
       | c -> go (if c < 0 then n.left else n.right))
  in
  go t.root
;;

let insert key value t =
  let rec go = function
    | Leaf -> Node { key; value; left = Leaf; right = Leaf }
    | Node n ->
      (match t.key_compare key n.key with
       | 0 ->
         n.value <- value;
         Node n
       | c ->
         if c < 0
         then (
           n.left <- go n.left;
           Node n)
         else (
           n.right <- go n.right;
           Node n))
  in
  t.root <- go t.root
;;

let key_compare a b =
  match a, b with
  | M.Int x, M.Int y -> Int.compare x y
  | M.Sym x, M.Sym y -> String.compare x y
  | _ -> invalid_arg "key_compare: mixed key types"
;;

let ex_3_26 () =
  let t = make_table ~key_compare in
  (* insert out of order on purpose; the tree orders them *)
  List.iter
    (fun (k, v) -> insert (M.mint k) (M.msym v) t)
    [ 50, "fifty"
    ; 20, "twenty"
    ; 70, "seventy"
    ; 10, "ten"
    ; 30, "thirty"
    ; 60, "sixty"
    ; 80, "eighty"
    ];
  let hits =
    List.map
      (fun k ->
         match lookup (M.mint k) t with
         | Some v -> M.show v
         | None -> "false")
      [ 10; 30; 50; 70; 80 ]
  in
  (* an overwrite under the same key *)
  insert (M.mint 20) (M.msym "new") t;
  let twenty =
    match lookup (M.mint 20) t with
    | Some v -> M.show v
    | None -> "false"
  in
  let miss = lookup (M.mint 40) t in
  String.concat " " hits, twenty, miss
;;
