(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.43: the yacht puzzle. The daughters are a closed variant
   compared by constructor, as the subset requires for structured data.
   Each father draws his daughter from those not yet taken, so the
   distinctness restriction never generates a doomed branch, and every
   restriction is imposed as soon as the father it mentions has a
   daughter, which is 4.40's efficiency advice. The four named yachts
   keep their names; Parker's yacht takes the one remaining name; no
   yacht carries its owner's daughter's name; and Gabrielle's father's
   yacht is named after Dr. Parker's daughter. Told that Mary Ann is
   Moore's daughter the puzzle has one solution, Colonel Downing. Without
   that premise the complete search finds two solutions and Lorna's
   father differs between them. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program ~told =
  {|
type daughter = Mary_ann | Gabrielle | Melissa | Rosalind | Lorna

let same a b =
  match a, b with
  | Mary_ann, Mary_ann -> true
  | Gabrielle, Gabrielle -> true
  | Melissa, Melissa -> true
  | Rosalind, Rosalind -> true
  | Lorna, Lorna -> true
  | _ -> false

let rec without d ds =
  match ds with
  | [] -> []
  | other :: rest -> if same other d then without d rest else other :: without d rest

let rec an_element_of items =
  match items with
  | [] -> require false; Lorna
  | x :: rest -> amb x (an_element_of rest)

let told = |}
  ^ string_of_bool told
  ^ {|

let yacht_puzzle daughters =
  let moore = an_element_of daughters in
  require (not (same moore Lorna));
  require ((not told) || same moore Mary_ann);
  let downing = an_element_of (without moore daughters) in
  require (not (same downing Melissa));
  let hall = an_element_of (without downing (without moore daughters)) in
  require (not (same hall Rosalind));
  let barnacle = an_element_of (without hall (without downing (without moore daughters))) in
  require (same barnacle Melissa);
  let parker =
    an_element_of (without barnacle (without hall (without downing (without moore daughters))))
  in
  let yacht_parker =
    an_element_of (without Lorna (without Melissa (without Rosalind (without Gabrielle daughters))))
  in
  require (not (same yacht_parker parker));
  let gabrielles_fathers_yacht =
    if same moore Gabrielle then Lorna
    else if same downing Gabrielle then Melissa
    else if same hall Gabrielle then Rosalind
    else if same barnacle Gabrielle then Gabrielle
    else yacht_parker
  in
  require (same gabrielles_fathers_yacht parker);
  let lornas_father =
    if same moore Lorna then "moore"
    else if same downing Lorna then "downing"
    else if same hall Lorna then "hall"
    else if same barnacle Lorna then "barnacle"
    else "parker"
  in
  "(lornas-father " ^ lornas_father ^ ")"

let () = print_endline (yacht_puzzle [ Mary_ann; Gabrielle; Melissa; Rosalind; Lorna ])
|}
;;

let ex_4_43 () =
  ("told" :: transcript (program ~told:true))
  @ ("open" :: transcript (program ~told:false))
;;
