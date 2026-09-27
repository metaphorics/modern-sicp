(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.66 *)

(** Exercise 3.66: the order of the pairs stream. The map class is
    [T]: the exact positions that are enumerable are measured against
    the closed forms, and the pairs beyond reach are bounded by the
    same formulas. *)

open Sicp_ch3.Sec_3_5

(* One-based position of each pair in the first [limit] elements. *)
let positions limit =
  let table = Hashtbl.create 1024 in
  let rec go s idx =
    if idx <= limit
    then (
      Hashtbl.replace table (Streams.stream_car s) idx;
      go (Streams.stream_cdr s) (idx + 1))
  in
  go (Pairs.pairs Infinite.integers Infinite.integers) 1;
  table
;;

let ex_3_66 () =
  let table = positions 40_000 in
  let pos p = Hashtbl.find_opt table p in
  (* (1, j) sits at position 2j - 2: every even slot belongs to the
     first row. The formula holds from j = 2 up; (1, 1) is the stream's
     first element, position 1. Verified for 2 <= j <= 100 (position
     198). *)
  let first_row_exact =
    List.for_all
      (fun j -> pos (1, j) = Some ((2 * j) - 2))
      (List.init 99 (fun k -> k + 2))
  in
  (* (k, k) sits at position 2^k - 1, the diagonal doubling. Verified
     for k <= 15 (position 32767). *)
  let diagonal_exact =
    List.for_all
      (fun k -> pos (k, k) = Some ((1 lsl k) - 1))
      (List.init 14 (fun i -> i + 2))
  in
  let pos_2_10 = pos (2, 10) in
  let pos_9_10 = pos (9, 10) in
  let pos_10_10 = pos (10, 10) in
  first_row_exact, diagonal_exact, pos_2_10, pos_9_10, pos_10_10
;;
