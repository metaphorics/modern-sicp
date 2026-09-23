(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.29: binary mobiles, both representations. *)

type structure =
  | Weight of int
  | Hanging of mobile

and mobile = Mobile of branch * branch
and branch = Branch of int * structure

let left_branch (Mobile (left, _)) = left
let right_branch (Mobile (_, right)) = right
let branch_length (Branch (length, _)) = length
let branch_structure (Branch (_, structure)) = structure

let rec total_weight mobile =
  let weight structure =
    match structure with
    | Weight w -> w
    | Hanging sub -> total_weight sub
  in
  weight (branch_structure (left_branch mobile))
  + weight (branch_structure (right_branch mobile))
;;

let rec balanced mobile =
  let branch_torque branch =
    branch_length branch
    *
    match branch_structure branch with
    | Weight w -> w
    | Hanging sub -> total_weight sub
  in
  let subtree_balanced branch =
    match branch_structure branch with
    | Weight _ -> true
    | Hanging sub -> balanced sub
  in
  let left = left_branch mobile in
  let right = right_branch mobile in
  branch_torque left = branch_torque right
  && subtree_balanced left
  && subtree_balanced right
;;

let ex_2_29 mobile = total_weight mobile, balanced mobile

module Cons_repr = struct
  type structure =
    | Weight of int
    | Hanging of mobile

  and mobile = branch * branch
  and branch = int * structure

  let rec branch_weight branch =
    match snd branch with
    | Weight w -> w
    | Hanging sub -> total_weight sub

  and branch_torque branch = fst branch * branch_weight branch
  and total_weight (left, right) = branch_weight left + branch_weight right

  and subtree_balanced branch =
    match snd branch with
    | Weight _ -> true
    | Hanging sub -> balanced sub

  and balanced (left, right) =
    branch_torque left = branch_torque right
    && subtree_balanced left
    && subtree_balanced right
  ;;
end
