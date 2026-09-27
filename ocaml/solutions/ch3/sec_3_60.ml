(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.60 *)

(** Exercise 3.60: [mul-series], the product of two power series as a
    stream of coefficients, verified by sin^2 + cos^2 = 1. The map
    class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec mul_series s1 s2 =
  Streams.Cons
    ( Streams.stream_car s1 *. Streams.stream_car s2
    , lazy
        (Infinite.add_streams_float
           (Streams.stream_map
              (fun b -> Streams.stream_car s1 *. b)
              (Streams.stream_cdr s2))
           (mul_series (Streams.stream_cdr s1) s2)) )
;;

let ex_3_60 () =
  let sin2 = mul_series Sec_3_59.sine_series Sec_3_59.sine_series in
  let cos2 = mul_series Sec_3_59.cosine_series Sec_3_59.cosine_series in
  let unit_series = Infinite.add_streams_float sin2 cos2 in
  let coefficients = Streams.stream_take 5 unit_series in
  (* The identity holds to floating-point dust: products and sums of
     the series leave ~1e-16 residue in the zero terms. *)
  let ok =
    Float.abs (List.hd coefficients -. 1.0) < 1e-12
    && List.for_all2
         (fun a b -> Float.abs (a -. b) < 1e-12)
         (List.tl coefficients)
         [ 0.0; 0.0; 0.0; 0.0 ]
  in
  coefficients, ok
;;
