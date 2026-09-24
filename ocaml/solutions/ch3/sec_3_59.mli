(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.59 *)

(** Exercise 3.59: power series as streams of coefficients:
    [integrate_series], the exponential series, and the sine and
    cosine series from the derivative relations. The map class is
    [T]. *)

(** [integrate_series s] is the stream a_0, a_1/2, a_2/3, ... of the
    non-constant integral's coefficients; cons the constant term on
    where one is wanted. *)
val integrate_series
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> float Sicp_ch3.Sec_3_5.Streams.stream

val exp_series : float Sicp_ch3.Sec_3_5.Streams.stream

(** The sine and cosine series: each is the integral of the other,
    with cosine negated, and constant terms 0 and 1. *)
val cosine_series : float Sicp_ch3.Sec_3_5.Streams.stream

val sine_series : float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_59 ()] is the first six coefficients of e^x
    ([1; 1; 0.5; 1/6; 1/24; 1/120]), of cos x ([1; 0; -0.5; 0; 1/24;
    0]), and of sin x ([0; 1; 0; -1/6; 0; 1/120]). *)
val ex_3_59 : unit -> float list * float list * float list
