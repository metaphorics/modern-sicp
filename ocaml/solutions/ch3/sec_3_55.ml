(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.55 *)

(** Exercise 3.55: [partial-sums], the running total of a stream,
    defined the way the text defines it: the head of the input followed
    by the input's tail added into the partial sums themselves. The map
    class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec partial_sums s =
  Streams.Cons
    ( Streams.stream_car s
    , lazy (Infinite.add_streams (Streams.stream_cdr s) (partial_sums s)) )
;;

let ex_3_55 () =
  ( Streams.stream_take 5 (partial_sums Infinite.integers)
  , Streams.stream_take 5 (partial_sums Infinite.ones) )
;;
