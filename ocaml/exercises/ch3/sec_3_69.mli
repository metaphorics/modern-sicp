(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.69 *)

(** Exercise 3.69: [triples] over three streams and the Pythagorean
    triples filtered from them. The map class is [T]: each row of the
    triple array is the exercise 3.67 shape prepended with S_0, so the
    new stream mixes in exactly one additional stream. *)

(** [triples s t u] is the stream of triples (S_i, T_j, U_k) with
    i <= j <= k, ordered so every triple eventually appears. *)
val triples
  :  int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> int Sicp_ch3.Sec_3_5.Streams.stream
  -> (int * int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** The triples with i^2 + j^2 = k^2. *)
val pythagorean : (int * int * int) Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_69 ()] is the first three Pythagorean triples:
    (3, 4, 5), (6, 8, 10), (5, 12, 13). *)
val ex_3_69 : unit -> (int * int * int) list
