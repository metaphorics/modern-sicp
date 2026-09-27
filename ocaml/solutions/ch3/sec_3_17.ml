(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.17 *)

(** Exercise 3.17: [count_pairs_distinct] keeps an auxiliary list of the
    pairs already counted, tested by physical equality, so each pair is
    counted once no matter how many paths reach it -- and a cycle ends
    the walk instead of trapping it. OCaml's stdlib has no
    identity-keyed set, so the visited list is scanned by hand. *)

open Sicp_ch3.Sec_3_3.Mpairs

let count_pairs_distinct x =
  let visited = ref [] in
  let rec go o =
    if is_pair o
    then (
      let p = pair_of o in
      if List.exists (fun c -> c == p) !visited
      then 0
      else (
        visited := p :: !visited;
        go (car o) + go (cdr o) + 1))
    else 0
  in
  go x
;;

let ex_3_17 () =
  let three = mcons (msym "a") (mcons (msym "b") (mcons (msym "c") mnil)) in
  let shared = mcons (msym "b") mnil in
  let four = mcons shared (mcons shared mnil) in
  let y = mcons (msym "a") (mcons (msym "b") mnil) in
  let seven = mcons (Pair (pair_of y)) (Pair (pair_of y)) in
  let ring = mcons (msym "a") (mcons (msym "b") (mcons (msym "c") mnil)) in
  set_cdr (last_pair ring) ring;
  ( count_pairs_distinct three
  , count_pairs_distinct four
  , count_pairs_distinct seven
  , count_pairs_distinct ring )
;;
