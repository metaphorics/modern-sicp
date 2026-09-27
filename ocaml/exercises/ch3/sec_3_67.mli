(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.67 *)

(** Exercise 3.67: all pairs, not only those above the diagonal: mix
    in the rest of the first column alongside the first row. The map
    class is [T]. *)

(** [pairs_all s t] is the stream of every pair from the two streams,
    the full array rather than the on-or-above-diagonal half. *)
val pairs_all
  :  int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> (int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_67 ()] is the first twenty-four pairs and whether every pair
    with both components at most 3 has already appeared there. *)
val ex_3_67 : unit -> (int * int) list * bool
