(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.65 *)

(** Exercise 3.65: the alternating series for ln 2, with its plain,
    Euler-transformed, and super-accelerated approximations. The map
    class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec ln2_summands n =
  Streams.Cons
    ( 1.0 /. float_of_int n
    , lazy (Streams.stream_map (fun x -> -.x) (ln2_summands (n + 1))) )
;;

let ln2_stream = Convergence.partial_sums (ln2_summands 1)
let ln2_euler = Convergence.euler_transform ln2_stream

let ln2_accelerated =
  Convergence.accelerated_sequence Convergence.euler_transform ln2_stream
;;

let ex_3_65 () =
  ( Streams.stream_take 8 ln2_stream
  , Streams.stream_take 8 ln2_euler
  , Streams.stream_take 8 ln2_accelerated )
;;
