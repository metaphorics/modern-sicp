(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.18 *)

(** Exercise 3.18: [contains_cycle] walks the cdr chain keeping a list
    of the pairs already seen, tested by physical identity; a repeat
    means the chain loops. OCaml's stdlib has no identity-keyed set, so
    the scan is a hand-written physical-equality search. *)

open Sicp_ch3.Sec_3_3.Mpairs

let contains_cycle x =
  let visited = ref [] in
  let rec go = function
    | Pair p ->
      if List.exists (fun c -> c == p) !visited
      then true
      else (
        visited := p :: !visited;
        go p.cdr)
    | _ -> false
  in
  go x
;;

let ex_3_18 () =
  let plain = from_symbols [ "a"; "b"; "c" ] in
  (* the ring of exercise 3.13, built fresh here so closing it does not
     also close [plain]: they must not be the same pair chain *)
  let ring = from_symbols [ "a"; "b"; "c" ] in
  set_cdr (last_pair ring) ring;
  let self = { car = msym "a"; cdr = Nil } in
  set_cdr (Pair self) (Pair self);
  ( contains_cycle plain
  , contains_cycle ring
  , contains_cycle (Pair self)
  , contains_cycle mnil )
;;
