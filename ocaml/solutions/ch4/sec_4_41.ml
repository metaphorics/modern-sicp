(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.41: an ordinary program for the multiple-dwelling
    puzzle. This exercise is an A row: the statement asks for a program
    in the reader's working language, which is now OCaml, so the solver
    below is a plain host function. It enumerates the permutations of
    the five floors over the five people and filters them with the
    puzzle's restrictions -- no amb, no search machinery -- and answers
    the solutions in the printed shape the amb evaluator's answers
    take. *)

(** One assignment: the floors of Baker, Cooper, Fletcher, Miller, and
    Smith, in that order. *)
type assignment =
  { baker : int
  ; cooper : int
  ; fletcher : int
  ; miller : int
  ; smith : int
  }

let satisfies a =
  let distinct =
    List.length
      (List.sort_uniq compare [ a.baker; a.cooper; a.fletcher; a.miller; a.smith ])
    = 5
  in
  distinct
  && a.baker <> 5
  && a.cooper <> 1
  && a.fletcher <> 5
  && a.fletcher <> 1
  && a.miller > a.cooper
  && abs (a.smith - a.fletcher) <> 1
  && abs (a.fletcher - a.cooper) <> 1
;;

(** Every permutation of [xs]. *)
let remove x ys = List.filter (fun y -> y <> x) ys

let rec permutations = function
  | [] -> [ [] ]
  | xs ->
    List.concat_map (fun x -> List.map (fun p -> x :: p) (permutations (remove x xs))) xs
;;

let solutions =
  let floors = [ 1; 2; 3; 4; 5 ] in
  permutations floors
  |> List.filter_map (function
    | [ baker; cooper; fletcher; miller; smith ] ->
      let a = { baker; cooper; fletcher; miller; smith } in
      if satisfies a then Some a else None
    | _ -> None)
;;

let render (a : assignment) =
  let entry name v = "(" ^ name ^ " " ^ string_of_int v ^ ")" in
  "("
  ^ String.concat
      " "
      [ entry "baker" a.baker
      ; entry "cooper" a.cooper
      ; entry "fletcher" a.fletcher
      ; entry "miller" a.miller
      ; entry "smith" a.smith
      ]
  ^ ")"
;;

let ex_4_41 () =
  List.map render solutions @ [ "solutions=" ^ string_of_int (List.length solutions) ]
;;
