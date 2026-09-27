(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.61 *)

(** Exercise 3.61: [invert-unit-series], the reciprocal of a power
    series with constant term 1, from X = 1 - S_R X. The map class is
    [T]; it needs [mul-series] from exercise 3.60. *)

(** [invert_unit_series s] is 1/S for a series whose constant term is
    1: the constant 1 followed by the negation of S_R times the
    reciprocal itself. *)
val invert_unit_series
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_61 ()] is the first five coefficients of e^x times its
    reciprocal: [1; 0; 0; 0; 0], the unit series. *)
val ex_3_61 : unit -> float list
