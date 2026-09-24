(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.76 *)

(** Exercise 3.76: [smooth] as a reusable combinator, and the
    zero-crossing detector rebuilt on top of it. The map class is
    [T]. *)

(** [smooth s] is the stream of averages of successive elements. *)
val smooth
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [zero_crossings sense_data] runs Alyssa's detector over the
    smoothed signal -- the detector itself is unchanged, which is the
    modularity Eva Lu Ator asked for. *)
val zero_crossings
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_76 ()] is [smooth] over 1, 2, 3, 4 ([1.5; 2.5; 3.5]) and the
    twelve crossings of the sample signal after smoothing. *)
val ex_3_76 : unit -> float list * int list
