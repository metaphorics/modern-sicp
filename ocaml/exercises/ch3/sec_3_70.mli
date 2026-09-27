(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.70 *)

(** Exercise 3.70: [merge-weighted] and [weighted-pairs], used for the
    two requested orders. The map class is [T]. *)

(** [merge_weighted weight s1 s2] merges two ordered streams by their
    weights, dropping repetitions as the unweighted [merge] does. *)
val merge_weighted
  :  ('a -> int)
  -> 'a Sicp_ch3.Sec_3_5.Streams.stream
  -> 'a Sicp_ch3.Sec_3_5.Streams.stream
  -> 'a Sicp_ch3.Sec_3_5.Streams.stream

(** [weighted_pairs weight s t] is the on-or-above-diagonal stream of
    pairs ordered by [weight]. *)
val weighted_pairs
  :  (int * int -> int)
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> (int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** The two orders the statement asks for: all pairs with i <= j by
    the sum i + j, and pairs of integers not divisible by 2, 3, or 5
    by the weight 2i + 3j + 5ij. *)
val ordered_by_sum : (int * int) Sicp_ch3.Sec_3_5.Streams.stream

val ordered_by_235 : (int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_70 ()] is the first ten pairs of each order. *)
val ex_3_70 : unit -> (int * int) list * (int * int) list
