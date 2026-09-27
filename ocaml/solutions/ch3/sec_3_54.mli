(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.54 *)

(** Exercise 3.54: [mul-streams], the elementwise product, and the
    factorial stream built from the integers and its own tail. The map
    class is [T]. *)

val mul_streams
  :  int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** [factorials] has 1 as its zeroth element and [n + 1]! as its
    [n]th: the integers multiplied elementwise into the stream's own
    tail. *)
val factorials : int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_54 ()] is the first eight elements of [factorials]:
    [1; 1; 2; 6; 24; 120; 720; 5040]. *)
val ex_3_54 : unit -> int list
