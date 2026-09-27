(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.80 *)

(** Exercise 3.80: the series RLC circuit as a pair of coupled signal
    streams. The map class is [T]. *)

open Sicp_ch3.Sec_3_5

(* The coupled streams are knotted through two shared promises, one
   per derivative: each integrator consumes the promise that is
   backpatched with the map over the streams it feeds. Every demand
   advances both state streams one element in lockstep. *)
let rlc r l c dt vc0 il0 =
  let dvc = ref (lazy Streams.the_empty_stream) in
  let dil = ref (lazy Streams.the_empty_stream) in
  let vc = Sec_3_77.integral (lazy (Lazy.force !dvc)) vc0 dt in
  let il = Sec_3_77.integral (lazy (Lazy.force !dil)) il0 dt in
  dvc := lazy (Streams.stream_map (fun il -> -.il /. c) il);
  dil := lazy (Infinite.stream_map2 (fun vc il -> (vc /. l) -. (r /. l *. il)) vc il);
  vc, il
;;

let ex_3_80 () =
  (* The statement's circuit: R = 1 ohm, L = 1 henry, C = 0.2 farad,
     dt = 0.1 s, i_L0 = 0 amps, v_C0 = 10 volts. *)
  let vc, il = rlc 1.0 1.0 0.2 0.1 10.0 0.0 in
  Streams.stream_take 4 vc, Streams.stream_take 4 il
;;
