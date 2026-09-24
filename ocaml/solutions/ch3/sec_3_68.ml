(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.68 *)

(** Exercise 3.68: Louis Reasoner's pairs, whole first row and all.
    The map class is [T]: the version is implemented and the answer to
    "does this work?" is measured, not argued. *)

open Sicp_ch3.Sec_3_5

(* The book's version's positions among the first [limit] elements,
   for comparison. *)
let positions_book limit =
  let table = Hashtbl.create 64 in
  let rec go s idx =
    if idx <= limit
    then (
      Hashtbl.replace table (Streams.stream_car s) idx;
      go (Streams.stream_cdr s) (idx + 1))
  in
  go (Pairs.pairs Infinite.integers Infinite.integers) 1;
  table
;;

(* Louis's definition: the whole first row interleaved with the
   recursive remainder. *)
let rec pairs_louis s t =
  Pairs.interleave
    (Streams.stream_map (fun x -> Streams.stream_car s, x) t)
    (pairs_louis (Streams.stream_cdr s) (Streams.stream_cdr t))
;;

(* Louis's definition under a fuel budget. The construction is
   [interleave (row s t) (louis (cdr s) (cdr t))], and the recursive
   call is evaluated before [interleave] can return its first cons, so
   each level of descent burns one unit of fuel and no pair ever comes
   out. This is the executable answer to the exercise's "does this
   work?": it does not -- on infinite streams it diverges before the
   first element. *)
let louis_first_pair_within fuel =
  let rec go fuel s t =
    if !fuel <= 0
    then None
    else (
      decr fuel;
      go fuel (Streams.stream_cdr s) (Streams.stream_cdr t))
  in
  go (ref fuel) Infinite.integers Infinite.integers
;;

let ex_3_68 () =
  let budget = 200_000 in
  let louis_first = louis_first_pair_within budget in
  let book = Streams.stream_take 16 (Pairs.pairs Infinite.integers Infinite.integers) in
  louis_first, budget, book
;;
