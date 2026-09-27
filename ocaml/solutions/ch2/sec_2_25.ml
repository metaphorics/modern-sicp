(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.25: picking 7 from three nested pairs. *)

let ex_2_25 () =
  let v1 = 1, (3, ((5, 7), 9)) in
  let v2 = ((7, 0), ()), () in
  let v3 = 1, (2, (3, (4, (5, (6, 7))))) in
  snd (fst (snd (snd v1))), fst (fst (fst v2)), snd (snd (snd (snd (snd (snd v3)))))
;;
