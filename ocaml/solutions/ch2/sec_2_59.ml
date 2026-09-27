(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.59 *)

(** Exercise 2.59: [union_set] for the unordered-list representation.
    An element of [set1] joins the union only when [set2] does not
    already carry it; every element of [set2] joins unconditionally,
    since [set2] itself carries no duplicate. *)

let rec element_of_set x = function
  | [] -> false
  | first :: rest -> first = x || element_of_set x rest
;;

let adjoin_set x set = if element_of_set x set then set else x :: set

let rec union_set set1 set2 =
  match set1 with
  | [] -> set2
  | first :: rest ->
    if element_of_set first set2
    then union_set rest set2
    else first :: union_set rest set2
;;

(** [ex_2_59 ()] is the union of [\{1; 2; 3\}] and [\{3; 4; 5\}]. *)
let ex_2_59 () = union_set [ 1; 2; 3 ] [ 3; 4; 5 ]
