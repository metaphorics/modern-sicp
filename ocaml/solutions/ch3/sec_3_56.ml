(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.56 *)

(** Exercise 3.56: Hamming's numbers -- the positive integers whose
    only prime factors are 2, 3, and 5 -- enumerated in order without
    repetitions by merging the three scaled copies of the stream
    itself. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec merge s1 s2 =
  match s1, s2 with
  | Streams.Empty, s | s, Streams.Empty -> s
  | Streams.Cons (h1, t1), Streams.Cons (h2, t2) ->
    if h1 < h2
    then Streams.Cons (h1, lazy (merge (Lazy.force t1) s2))
    else if h1 > h2
    then Streams.Cons (h2, lazy (merge s1 (Lazy.force t2)))
    else Streams.Cons (h1, lazy (merge (Lazy.force t1) (Lazy.force t2)))
;;

let rec hamming =
  Streams.Cons
    ( 1
    , lazy
        (merge
           (Infinite.scale_stream hamming 2)
           (merge (Infinite.scale_stream hamming 3) (Infinite.scale_stream hamming 5))) )
;;

let ex_3_56 () = Streams.stream_take 12 hamming
