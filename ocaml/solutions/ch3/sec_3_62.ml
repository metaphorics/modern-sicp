(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.62 *)

(** Exercise 3.62: [div-series] over exercise 3.60's product and
    exercise 3.61's reciprocal, and the tangent series as sin/cos.
    The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let div_series numerator denominator =
  if Streams.stream_car denominator = 0.0
  then invalid_arg "div-series: zero constant term"
  else Sec_3_60.mul_series numerator (Sec_3_61.invert_unit_series denominator)
;;

let tangent_series = div_series Sec_3_59.sine_series Sec_3_59.cosine_series

let ex_3_62 () =
  let rejected =
    try
      ignore (div_series Sec_3_59.cosine_series Sec_3_59.sine_series);
      false
    with
    | Invalid_argument _ -> true
  in
  Streams.stream_take 6 tangent_series, rejected
;;
