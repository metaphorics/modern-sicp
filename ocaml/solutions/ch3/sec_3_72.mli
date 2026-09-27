(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.72 *)

(** Exercise 3.72: numbers writable as a sum of two squares in three
    different ways, from runs of three consecutive equal weights in
    the square-weighted pair stream. The map class is [T]. *)

(** The weight i^2 + j^2 and the pair stream ordered by it. *)
val square_weight : int * int -> int

val square_pairs : (int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** [three_square_stream s] pairs each run of three consecutive equal
    weights in [s] with the number and its three representations. *)
val three_square_stream
  :  (int * int) Sicp_ch3.Sec_3_5.Streams.stream
  -> (int * ((int * int) * (int * int) * (int * int))) Sicp_ch3.Sec_3_5.Streams.stream

(** The numbers alone, in ascending order. *)
val three_square_numbers : int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_72 ()] is the first five such numbers -- beginning at 325 --
    and the three writings that name each one. *)
val ex_3_72 : unit -> int list * ((int * int) * (int * int) * (int * int)) list
