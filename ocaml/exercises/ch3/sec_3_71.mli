(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.71 *)

(** Exercise 3.71: Ramanujan numbers, the sums of two cubes writable
    in more than one way, from consecutive equal weights in the cube-
    weighted pair stream. The map class is [T]. *)

(** The weight i^3 + j^3 and the pair stream ordered by it. *)
val cube_weight : int * int -> int

val cube_pairs : (int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** [ramanujan_stream s] pairs each run of two consecutive equal
    weights in [s] with the number and its two representations. *)
val ramanujan_stream
  :  (int * int) Sicp_ch3.Sec_3_5.Streams.stream
  -> (int * (int * int) * (int * int)) Sicp_ch3.Sec_3_5.Streams.stream

(** The numbers alone, in ascending order. *)
val ramanujan_numbers : int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_71 ()] is 1729 followed by the next five:
    [1729; 4104; 13832; 20683; 32832; 39312]. *)
val ex_3_71 : unit -> int list
