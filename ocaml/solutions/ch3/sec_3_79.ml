(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.79 *)

(** Exercise 3.79: [solve-2nd] generalized to y'' = f(y', y). The map
    class is [T]. *)

open Sicp_ch3.Sec_3_5

(* The same shared-promise knot as exercise 3.78, with f in place of
   the a/b combination. *)
let solve_2nd_general f dt y0 dy0 =
  let ddy = ref (lazy Streams.the_empty_stream) in
  let dyp = ref (lazy Streams.the_empty_stream) in
  let dy = Sec_3_77.integral (lazy (Lazy.force !ddy)) dy0 dt in
  let y = Sec_3_77.integral (lazy (Lazy.force !dyp)) y0 dt in
  dyp := lazy dy;
  ddy := lazy (Infinite.stream_map2 f dy y);
  y
;;

let ex_3_79 () =
  (* The general solver with f(dy, y) = -y reproduces the harmonic
     oscillator of exercise 3.78; the two streams agree at the checked
     indices. *)
  let ys_general = solve_2nd_general (fun _ y -> -.y) 0.001 0.0 1.0 in
  let ys_specific = Sec_3_78.solve_2nd 0.0 (-1.0) 0.001 0.0 1.0 in
  List.for_all
    (fun k ->
       Float.equal (Streams.stream_ref ys_general k) (Streams.stream_ref ys_specific k))
    [ 1; 500; 1000; 1570 ]
;;
