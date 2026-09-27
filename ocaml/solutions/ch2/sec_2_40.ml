(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.40: unique-pairs. *)

let rec enumerate_interval low high =
  if low > high then [] else low :: enumerate_interval (low + 1) high
;;

let flatmap proc seq = List.concat_map proc seq

let is_prime n =
  if n < 2
  then false
  else (
    let rec no_factor d = d * d > n || (n mod d <> 0 && no_factor (d + 1)) in
    no_factor 2)
;;

let ex_2_40_unique_pairs n =
  flatmap
    (fun i -> List.map (fun j -> i, j) (enumerate_interval 1 (i - 1)))
    (enumerate_interval 1 n)
;;

let ex_2_40_prime_sum_pairs n =
  List.filter_map
    (fun (i, j) -> if is_prime (i + j) then Some (i, j, i + j) else None)
    (ex_2_40_unique_pairs n)
;;
