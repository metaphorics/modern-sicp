(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.62 *)

(** Exercise 3.62: [div-series] over exercise 3.60's product and
    exercise 3.61's reciprocal, and the tangent series as sin/cos.
    The map class is [T]. *)

(** [div_series numerator denominator] is numerator/denominator as a
    coefficient stream; it raises [Invalid_argument] when the
    denominator's constant term is zero, where no reciprocal exists. *)
val div_series
  :  float Sicp_ch3.Sec_3_5.Streams.stream
  -> float Sicp_ch3.Sec_3_5.Streams.stream
  -> float Sicp_ch3.Sec_3_5.Streams.stream

(** The tangent series x + x^3/3 + 2x^5/15 + ... = sin x / cos x. *)
val tangent_series : float Sicp_ch3.Sec_3_5.Streams.stream

(** [ex_3_62 ()] is the first six tangent coefficients
    ([0; 1; 0; 1/3; 0; 2/15]) and whether dividing by the sine series
    (constant term 0) is rejected. *)
val ex_3_62 : unit -> float list * bool
