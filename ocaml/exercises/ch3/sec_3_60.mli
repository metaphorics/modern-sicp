(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.60 *)

(** Exercise 3.60: [mul-series], the product of two power series as a
    stream of coefficients, verified by sin^2 + cos^2 = 1. The map
    class is [T]. *)

(** [mul_series s1 s2] is the Cauchy product of the two coefficient
    streams: a_0 b_0 followed by a_0 times the rest of s2 added into
    the product of the rest of s1 with all of s2. *)
val mul_series
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> float Sicp_ch3.Sec_3_5.Streams.stream
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_60 ()] is the first five coefficients of sin^2 + cos^2 (the
    unit series, up to floating-point dust in the constant term) and
    whether that identity holds to the bit at the checked places. *)
val ex_3_60 : unit -> float list * bool
