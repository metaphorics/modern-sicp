(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.72 *)

(** Exercise 3.72: numbers writable as a sum of two squares in three
    different ways, from runs of three consecutive equal weights in
    the square-weighted pair stream. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let square_weight (i, j) = (i * i) + (j * j)

let square_pairs =
  Sec_3_70.weighted_pairs square_weight Infinite.integers Infinite.integers
;;

(* Three consecutive pairs of equal weight name one number and show
   its three writings. *)
let rec three_square_stream s =
  let p1 = Streams.stream_car s in
  let s2 = Streams.stream_cdr s in
  let p2 = Streams.stream_car s2 in
  let s3 = Streams.stream_cdr s2 in
  let p3 = Streams.stream_car s3 in
  let w1 = square_weight p1 in
  let w2 = square_weight p2 in
  let w3 = square_weight p3 in
  if w1 = w2 && w2 = w3
  then
    Streams.Cons ((w1, (p1, p2, p3)), lazy (three_square_stream (Streams.stream_cdr s3)))
  else three_square_stream s2
;;

let three_square_numbers =
  Streams.stream_map (fun (w, _) -> w) (three_square_stream square_pairs)
;;

let ex_3_72 () =
  let numbers = Streams.stream_take 5 three_square_numbers in
  let writings =
    Streams.stream_take 5 (Streams.stream_map snd (three_square_stream square_pairs))
  in
  numbers, writings
;;
