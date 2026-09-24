(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.63 *)

(** Exercise 3.63: why does [sqrt-stream] bind [guesses] locally? The
    map class is [T]: both versions run with a counting improvement
    step, and the two prefixes answer the question -- and show it is
    about more than efficiency. *)

val sqrt_stream_local : float -> int ref -> float Sicp_ch3.Sec_3_5.Streams.stream
val sqrt_stream_open : float -> int ref -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_63 ()] is the first five guesses of the local version, its
    improvement count, the first five guesses of the open version, and
    its improvement count. The local version computes each Newton
    refinement once and advances toward the root; the open version
    spends every call re-refining the initial guess 1.0, so it repeats
    work without ever building on the previous guess. *)
val ex_3_63 : unit -> float list * int * float list * int
