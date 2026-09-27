(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.5 exercise 3.67 *)

(** Exercise 3.67: all pairs, not only those above the diagonal: mix
    in the rest of the first column alongside the first row. The map
    class is [T]. *)

open Sicp_ch3.Sec_3_5

let rec pairs_all s t =
  Streams.Cons
    ( (Streams.stream_car s, Streams.stream_car t)
    , lazy
        (Pairs.interleave
           (Streams.stream_map (fun x -> Streams.stream_car s, x) (Streams.stream_cdr t))
           (Pairs.interleave
              (Streams.stream_map
                 (fun x -> x, Streams.stream_car t)
                 (Streams.stream_cdr s))
              (pairs_all (Streams.stream_cdr s) (Streams.stream_cdr t)))) )
;;

let ex_3_67 () =
  let first = Streams.stream_take 24 (pairs_all Infinite.integers Infinite.integers) in
  (* Every pair with i, j <= 3 has surfaced within the prefix. *)
  let covered =
    List.for_all
      (fun (i, j) -> List.mem (i, j) first)
      [ 1, 1; 1, 2; 1, 3; 2, 1; 2, 2; 2, 3; 3, 1; 3, 2; 3, 3 ]
  in
  first, covered
;;
