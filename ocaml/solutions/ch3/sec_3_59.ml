(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.59 *)

(** Exercise 3.59: power series as streams of coefficients:
    [integrate_series], the exponential series, and the sine and
    cosine series from the derivative relations. The map class is
    [T]. *)

open Sicp_ch3.Sec_3_5

let integrate_series s =
  Infinite.stream_map2
    (fun a k -> a /. float_of_int k)
    s
    (Infinite.integers_starting_from 1)
;;

let rec exp_series = Streams.Cons (1.0, lazy (integrate_series exp_series))

let rec cosine_series =
  Streams.Cons
    (1.0, lazy (integrate_series (Streams.stream_map (fun x -> -.x) sine_series)))

and sine_series = Streams.Cons (0.0, lazy (integrate_series cosine_series))

let ex_3_59 () =
  ( Streams.stream_take 6 exp_series
  , Streams.stream_take 6 cosine_series
  , Streams.stream_take 6 sine_series )
;;
