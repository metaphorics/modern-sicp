(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.66 *)

(** Exercise 2.66: [lookup] for a set of records structured as a
    binary tree ordered by key, in place of the unordered
    [Sicp_ch2.Sec_2_3.Record_db.lookup]. The search is
    [element_of_set?]'s tree walk with the record's key standing in
    for the tree entry. *)

type record =
  { key : int
  ; name : string
  }

type tree =
  | Empty
  | Node of tree * record * tree

let rec lookup given_key = function
  | Empty -> None
  | Node (left, record, right) ->
    if given_key = record.key
    then Some record
    else if given_key < record.key
    then lookup given_key left
    else lookup given_key right
;;

let sample_tree =
  Node
    ( Node (Empty, { key = 1; name = "ada" }, Empty)
    , { key = 2; name = "grace" }
    , Node (Empty, { key = 3; name = "margaret" }, Empty) )
;;

(** [ex_2_66 ()] is [lookup 3] against [sample_tree]. *)
let ex_2_66 () = lookup 3 sample_tree
