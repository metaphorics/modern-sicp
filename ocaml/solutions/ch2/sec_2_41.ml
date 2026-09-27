(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.41: ordered triples with a given sum. *)

let rec enumerate_interval low high =
  if low > high then [] else low :: enumerate_interval (low + 1) high
;;

let flatmap proc seq = List.concat_map proc seq

let ex_2_41 n s =
  let triples =
    flatmap
      (fun i ->
         flatmap
           (fun j -> List.map (fun k -> i, j, k) (enumerate_interval 1 (j - 1)))
           (enumerate_interval 1 (i - 1)))
      (enumerate_interval 1 n)
  in
  List.filter (fun (i, j, k) -> i + j + k = s) triples
;;
