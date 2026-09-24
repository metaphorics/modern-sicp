(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.70 *)

(** Exercise 3.70: [merge-weighted] and [weighted-pairs], used for the
    two requested orders. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec merge_weighted weight s1 s2 =
  match s1, s2 with
  | Streams.Empty, s | s, Streams.Empty -> s
  | Streams.Cons (h1, t1), Streams.Cons (h2, t2) ->
    let w1 = weight h1 in
    let w2 = weight h2 in
    if w1 < w2
    then Streams.Cons (h1, lazy (merge_weighted weight (Lazy.force t1) s2))
    else if w1 > w2
    then Streams.Cons (h2, lazy (merge_weighted weight s1 (Lazy.force t2)))
    else
      (* Unlike 3.56's merge, equal weights are both kept: 3.71 and
         3.72 find their numbers as consecutive equal weights, which
         must survive the merge side by side. *)
      Streams.Cons
        ( h1
        , lazy
            (Streams.Cons
               (h2, lazy (merge_weighted weight (Lazy.force t1) (Lazy.force t2)))) )
;;

let rec weighted_pairs weight s t =
  Streams.Cons
    ( (Streams.stream_car s, Streams.stream_car t)
    , lazy
        (merge_weighted
           weight
           (Streams.stream_map (fun x -> Streams.stream_car s, x) (Streams.stream_cdr t))
           (weighted_pairs weight (Streams.stream_cdr s) (Streams.stream_cdr t))) )
;;

let weight_sum (i, j) = i + j
let ordered_by_sum = weighted_pairs weight_sum Infinite.integers Infinite.integers
let coprime_to_2_3_5 n = n mod 2 <> 0 && n mod 3 <> 0 && n mod 5 <> 0
let weight_2i_3j_5ij (i, j) = (2 * i) + (3 * j) + (5 * i * j)
let odd_sources = Streams.stream_filter coprime_to_2_3_5 Infinite.integers
let ordered_by_235 = weighted_pairs weight_2i_3j_5ij odd_sources odd_sources

let ex_3_70 () =
  Streams.stream_take 10 ordered_by_sum, Streams.stream_take 10 ordered_by_235
;;
