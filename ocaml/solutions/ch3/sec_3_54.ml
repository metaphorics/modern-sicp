(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.54 *)

(** Exercise 3.54: [mul-streams], the elementwise product, and the
    factorial stream built from the integers and its own tail. The map
    class is [T]. *)

open Sicp_ch3.Sec_3_5

let mul_streams = Infinite.stream_map2 ( * )

(* The tail multiplies the integers from 1 into the factorials
   themselves, aligned: 1x1, 2x1, 3x2, 4x6, ... Pairing with the
   stream's own tail instead would demand the tail's head from the
   tail being forced, a promise cycle. *)
let rec factorials =
  Streams.Cons (1, lazy (mul_streams (Infinite.integers_starting_from 1) factorials))
;;

let ex_3_54 () = Streams.stream_take 8 factorials
