(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Every interleaving of [lists] that keeps each list's own order,
   built by choosing which list contributes the next event and
   recursing on what remains. *)
let rec interleavings lists =
  if List.for_all (( = ) []) lists
  then [ [] ]
  else
    List.concat_map (fun index -> extend_at index lists) (List.mapi (fun i _ -> i) lists)

and extend_at index lists =
  match List.nth lists index with
  | [] -> []
  | head :: tail ->
    let lists' = List.mapi (fun j l -> if j = index then tail else l) lists in
    List.map (fun rest -> head :: rest) (interleavings lists')
;;
