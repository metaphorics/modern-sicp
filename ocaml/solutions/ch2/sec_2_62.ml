(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.62 *)

(** Exercise 2.62: a [Theta(n)] [union_set] for ordered lists. Merging
    advances whichever set carries the smaller head, so no element is
    compared more than a constant number of times, unlike the
    unordered representation's [Theta(n^2)]. *)

let rec union_set set1 set2 =
  match set1, set2 with
  | [], set | set, [] -> set
  | first1 :: rest1, first2 :: rest2 ->
    if first1 = first2
    then first1 :: union_set rest1 rest2
    else if first1 < first2
    then first1 :: union_set rest1 set2
    else first2 :: union_set set1 rest2
;;

(** [ex_2_62 ()] is the union of [\{1; 3; 5; 7\}] and [\{2; 3; 6; 7\}]. *)
let ex_2_62 () = union_set [ 1; 3; 5; 7 ] [ 2; 3; 6; 7 ]
