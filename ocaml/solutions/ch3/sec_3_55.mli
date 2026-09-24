(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.55 *)

(** Exercise 3.55: [partial-sums], the running total of a stream,
    defined the way the text defines it: the head of the input followed
    by the input's tail added into the partial sums themselves. The map
    class is [T]. *)

val partial_sums
  :  int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_55 ()] is the first five elements of [partial-sums integers]
    ([1; 3; 6; 10; 15]) and of [partial-sums ones] ([1; 2; 3; 4; 5]). *)
val ex_3_55 : unit -> int list * int list
