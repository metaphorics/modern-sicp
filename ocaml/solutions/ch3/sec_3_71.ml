(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.71 *)

(** Exercise 3.71: Ramanujan numbers, the sums of two cubes writable
    in more than one way, from consecutive equal weights in the cube-
    weighted pair stream. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let cube_weight (i, j) = (i * i * i) + (j * j * j)
let cube_pairs = Sec_3_70.weighted_pairs cube_weight Infinite.integers Infinite.integers

(* Two consecutive pairs of equal weight name one such number, with
   its two representations. *)
let rec ramanujan_stream s =
  let i1, j1 = Streams.stream_car s in
  let s' = Streams.stream_cdr s in
  let i2, j2 = Streams.stream_car s' in
  let w1 = cube_weight (i1, j1) in
  let w2 = cube_weight (i2, j2) in
  if w1 = w2
  then
    Streams.Cons
      ((w1, (i1, j1), (i2, j2)), lazy (ramanujan_stream (Streams.stream_cdr s')))
  else ramanujan_stream s'
;;

let ramanujan_numbers =
  Streams.stream_map (fun (w, _, _) -> w) (ramanujan_stream cube_pairs)
;;

let ex_3_71 () = Streams.stream_take 6 ramanujan_numbers
