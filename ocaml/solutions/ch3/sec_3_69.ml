(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.69 *)

(** Exercise 3.69: [triples] over three streams and the Pythagorean
    triples filtered from them. The map class is [T]: each row of the
    triple array is the exercise 3.67 shape prepended with S_0, so the
    new stream mixes in exactly one additional stream. *)

open Sicp_ch3.Sec_3_5

let rec triples s t u =
  Streams.Cons
    ( (Streams.stream_car s, Streams.stream_car t, Streams.stream_car u)
    , lazy
        (Pairs.interleave
           (Streams.stream_map
              (fun (a, b) -> Streams.stream_car s, a, b)
              (Pairs.pairs (Streams.stream_cdr t) (Streams.stream_cdr u)))
           (triples (Streams.stream_cdr s) (Streams.stream_cdr t) (Streams.stream_cdr u)))
    )
;;

let pythagorean =
  Streams.stream_filter
    (fun (i, j, k) -> (i * i) + (j * j) = k * k)
    (triples Infinite.integers Infinite.integers Infinite.integers)
;;

let ex_3_69 () = Streams.stream_take 3 pythagorean
