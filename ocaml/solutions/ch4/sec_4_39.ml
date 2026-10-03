(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.39: the order of the restrictions. Reordering the
   restrictions after all five choices cannot change the answer, and it
   cannot change the search either: every complete assignment is still
   generated, so the answers, choice points, and failed branches are
   identical, and only the constant cost of rejecting one assignment
   moves. Order matters once a restriction is placed before the choices
   it does not mention: the reordered procedure picks Fletcher first
   under his own clauses, Cooper next under the adjacency clause that
   mentions them both, and [distinct] last, and the search experiment's
   counters show the pruned search is a fraction of the size. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let library =
  {|
let rec member floor floors =
  match floors with
  | [] -> false
  | other :: rest -> other - floor = 0 || member floor rest

let rec distinct items =
  match items with
  | [] -> true
  | x :: rest -> (not (member x rest)) && distinct rest

let abs n = if n < 0 then - n else n

let show_dwelling baker cooper fletcher miller smith =
  "((baker " ^ string_of_int baker ^ ") (cooper " ^ string_of_int cooper
  ^ ") (fletcher " ^ string_of_int fletcher ^ ") (miller " ^ string_of_int miller
  ^ ") (smith " ^ string_of_int smith ^ "))"

let multiple_dwelling top_floor =
  let baker = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let cooper = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let fletcher = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let miller = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let smith = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (distinct [ baker; cooper; fletcher; miller; smith ]);
  require (baker <> top_floor);
  require (cooper <> 1);
  require (fletcher <> top_floor);
  require (fletcher <> 1);
  require (miller > cooper);
  require (abs (smith - fletcher) <> 1);
  require (abs (fletcher - cooper) <> 1);
  show_dwelling baker cooper fletcher miller smith

let multiple_dwelling_cheap_first top_floor =
  let baker = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let cooper = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let fletcher = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let miller = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let smith = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (baker <> top_floor);
  require (cooper <> 1);
  require (fletcher <> top_floor);
  require (fletcher <> 1);
  require (miller > cooper);
  require (abs (smith - fletcher) <> 1);
  require (abs (fletcher - cooper) <> 1);
  require (distinct [ baker; cooper; fletcher; miller; smith ]);
  show_dwelling baker cooper fletcher miller smith

let multiple_dwelling_reordered top_floor =
  let fletcher = amb 1 (amb 2 (amb 3 4)) in
  require (fletcher <> top_floor);
  require (fletcher <> 1);
  let cooper = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (cooper <> 1);
  require (abs (fletcher - cooper) <> 1);
  let baker = amb 1 (amb 2 (amb 3 4)) in
  require (baker <> top_floor);
  let miller = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (miller > cooper);
  let smith = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (abs (smith - fletcher) <> 1);
  require (distinct [ baker; cooper; fletcher; miller; smith ]);
  show_dwelling baker cooper fletcher miller smith
|}
;;

let run procedure =
  procedure :: transcript (library ^ "\nlet () = print_endline (" ^ procedure ^ " 5)\n")
;;

let ex_4_39 () =
  run "multiple_dwelling"
  @ run "multiple_dwelling_cheap_first"
  @ run "multiple_dwelling_reordered"
;;
