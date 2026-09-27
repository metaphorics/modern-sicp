(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.78 *)

(** Exercise 3.78: [solve-2nd], the feedback network for
    y'' - a y' - b y = 0. The map class is [T]: both integrators take
    delayed inputs, exercise 3.77's shape. *)

open Sicp_ch3.Sec_3_5

(* The three streams are knotted through one shared promise for dyy:
   both integrators consume it, and it is the elementwise a*dy + b*y
   over the very streams it feeds. The promise is planted before either
   integrator runs and backpatched with the real map2 before anything
   is forced, so every demand advances all three streams one element in
   lockstep instead of building fresh copies. *)
let solve_2nd a b dt y0 dy0 =
  let ddy = ref (lazy Streams.the_empty_stream) in
  let dyp = ref (lazy Streams.the_empty_stream) in
  (* dy integrates ddy; y integrates dy. Each integrator holds a
     promise that reads its ref when forced, so the backpatches below
     are what they end up consuming. *)
  let dy = Sec_3_77.integral (lazy (Lazy.force !ddy)) dy0 dt in
  let y = Sec_3_77.integral (lazy (Lazy.force !dyp)) y0 dt in
  dyp := lazy dy;
  ddy := lazy (Infinite.stream_map2 (fun d yy -> (a *. d) +. (b *. yy)) dy y);
  y
;;

let ex_3_78 () =
  (* The harmonic oscillator a = 0, b = -1 with y(0) = 0, y'(0) = 1
     solves y(t) = sin t; the stream at t = pi/2 - 0.0005 is within
     one step of 1. *)
  let ys = solve_2nd 0.0 (-1.0) 0.001 0.0 1.0 in
  Streams.stream_ref ys 1570
;;
