(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.64 *)

(** Exercise 3.64: [stream-limit], the convergence helper that walks a
    stream until two successive elements differ by less than the
    tolerance. The map class is [T]. *)

(** [stream_limit s tolerance] returns the first element of [s] whose
    predecessor differs from it by less than [tolerance] in absolute
    value. *)
val stream_limit : float Sicp_ch3.Sec_3_5.Streams.stream -> float -> float

(** [sqrt x tolerance] computes square roots to a tolerance through
    the stream of Newton guesses. *)
val sqrt : float -> float -> float

(** [ex_3_64 ()] is the root of 2 at tolerance 1e-4
    (1.41421568627450969...) and at 1e-8 (1.41421356237468991...). *)
val ex_3_64 : unit -> float * float
