(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.61 *)

(** Exercise 3.61: [invert-unit-series], the reciprocal of a power
    series with constant term 1, from X = 1 - S_R X. The map class is
    [T]; it needs [mul-series] from exercise 3.60. *)

open Sicp_ch3.Sec_3_5

let invert_unit_series s =
  let rec x =
    Streams.Cons
      ( 1.0
      , lazy
          (Streams.stream_map
             (fun coefficient -> -.coefficient)
             (Sec_3_60.mul_series (Streams.stream_cdr s) x)) )
  in
  x
;;

let ex_3_61 () =
  let product =
    Sec_3_60.mul_series Sec_3_59.exp_series (invert_unit_series Sec_3_59.exp_series)
  in
  Streams.stream_take 5 product
;;
