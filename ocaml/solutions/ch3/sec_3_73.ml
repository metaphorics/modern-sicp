(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.73 *)

(** Exercise 3.73: the RC circuit as a signal processor: the voltage
    is R times the current plus the integral of current over C. The
    map class is [T]. *)

open Sicp_ch3.Sec_3_5

let rc r c dt current v0 =
  Infinite.add_streams_float
    (Streams.stream_map (fun i -> r *. i) current)
    (Signals.integral (Streams.stream_map (fun i -> i /. c) current) v0 dt)
;;

let ex_3_73 () =
  (* RC1 of the statement: R = 5 ohms, C = 1 farad, dt = 0.5 s, driven
     by a constant one-ampere current from a zero initial voltage:
     a half volt of capacitor charge plus five ohms of drop per step. *)
  let rec unit_current = Streams.Cons (1.0, lazy unit_current) in
  let rc1 = rc 5.0 1.0 0.5 unit_current 0.0 in
  Streams.stream_take 5 rc1
;;
