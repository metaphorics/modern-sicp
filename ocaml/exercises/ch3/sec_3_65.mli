(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.65 *)

(** Exercise 3.65: the alternating series for ln 2, with its plain,
    Euler-transformed, and super-accelerated approximations. The map
    class is [T]. *)

(** The summands 1, -1/2, 1/3, -1/4, ... and their running sums. *)
val ln2_summands : int -> float Sicp_ch3.Sec_3_5.Streams.stream

val ln2_stream : float Sicp_ch3.Sec_3_5.Streams.stream

(** The Euler-transformed sequence, and the sequence of first terms of
    the tableau built from it. *)
val ln2_euler : float Sicp_ch3.Sec_3_5.Streams.stream

val ln2_accelerated : float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_65 ()] is the first eight elements of each sequence. The
    plain one is still between 0.634 and 0.759 after eight terms, the
    transformed one within 0.002, and the accelerated one has ln 2 to
    about thirteen decimal places. *)
val ex_3_65 : unit -> float list * float list * float list
