(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.64 *)

(** Exercise 3.64: [stream-limit], the convergence helper that walks a
    stream until two successive elements differ by less than the
    tolerance. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec stream_limit s tolerance =
  let a = Streams.stream_car s in
  let b = Streams.stream_car (Streams.stream_cdr s) in
  if Float.abs (a -. b) < tolerance
  then b
  else stream_limit (Streams.stream_cdr s) tolerance
;;

let sqrt x tolerance = stream_limit (Convergence.sqrt_stream x) tolerance
let ex_3_64 () = stream_limit (Convergence.sqrt_stream 2.0) 1e-4, sqrt 2.0 1e-8
