(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.60 *)

(** Exercise 2.60: sets as lists that allow duplicates. [element_of_set]
    is unchanged: a duplicate cannot hide a missing element from a
    linear scan. [adjoin_set] no longer has to check first, so it is
    the plain cons in constant time. [union_set] no longer has to
    filter, so it is the plain append, in the size of the first
    argument rather than its product with the second.
    [intersection_set] keeps every occurrence in [set1] that [set2]
    also carries, so it is unchanged in shape from the non-duplicate
    version, but the counting duplicates change its answer. *)

let rec element_of_set x = function
  | [] -> false
  | first :: rest -> first = x || element_of_set x rest
;;

let adjoin_set x set = x :: set
let union_set set1 set2 = set1 @ set2

let rec intersection_set set1 set2 =
  match set1 with
  | [] -> []
  | first :: rest ->
    if element_of_set first set2
    then first :: intersection_set rest set2
    else intersection_set rest set2
;;

(** The book's example: [\{1; 2; 3\}] as [(2 3 2 1 3 2 2)]. *)
let sample_set = [ 2; 3; 2; 1; 3; 2; 2 ]

(** [ex_2_60 ()] is [sample_set] with [1] adjoined, then unioned with
    itself, in that order. Adjoining onto a set that already carries
    the element grows it, unlike the non-duplicate representation;
    union grows every element's count instead of merging it away. *)
let ex_2_60 () =
  let adjoined = adjoin_set 1 sample_set in
  adjoined, union_set sample_set sample_set
;;
