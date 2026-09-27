(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.58 *)

(** Exercise 3.58: [expand] produces the successive digits of the
    fraction [num / den] written in the given radix. The map class is
    [T]: [quotient] and [remainder] become [/] and [mod] on integers. *)

open Sicp_ch3.Sec_3_5

let rec expand num den radix =
  Streams.Cons (num * radix / den, lazy (expand (num * radix mod den) den radix))
;;

let ex_3_58 () =
  Streams.stream_take 8 (expand 1 7 10), Streams.stream_take 5 (expand 3 8 10)
;;
