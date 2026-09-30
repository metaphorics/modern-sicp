(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.40: pruning before the restrictions. The two counting
   identities are measured by the search itself: a search that only
   assigns floors answers once per assignment, [5^5 = 3125] times, and
   the same search under the distinctness requirement answers [5! =
   120] times. The pruned procedure generates only possibilities no
   earlier restriction has ruled out: each person is drawn from the
   floors still free and the floors their own clauses allow, and every
   restriction is imposed as soon as the people it mentions have
   floors. The search experiment's counters compare the naive and the
   pruned searches; both answer the book's assignment. *)

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

let rec an_element_of items =
  match items with
  | [] -> require false; 0
  | x :: rest -> amb x (an_element_of rest)

let rec without floor floors =
  match floors with
  | [] -> []
  | other :: rest ->
    if other - floor = 0 then without floor rest else other :: without floor rest

let show_dwelling baker cooper fletcher miller smith =
  "((baker " ^ string_of_int baker ^ ") (cooper " ^ string_of_int cooper
  ^ ") (fletcher " ^ string_of_int fletcher ^ ") (miller " ^ string_of_int miller
  ^ ") (smith " ^ string_of_int smith ^ "))"

let any_floor top_floor = an_element_of [ 1; 2; 3; 4; top_floor ]

let assignments top_floor =
  let baker = any_floor top_floor in
  let cooper = any_floor top_floor in
  let fletcher = any_floor top_floor in
  let miller = any_floor top_floor in
  let smith = any_floor top_floor in
  [ baker; cooper; fletcher; miller; smith ]

let distinct_assignments top_floor =
  require (distinct (assignments top_floor))

let multiple_dwelling top_floor =
  let baker = any_floor top_floor in
  let cooper = any_floor top_floor in
  let fletcher = any_floor top_floor in
  let miller = any_floor top_floor in
  let smith = any_floor top_floor in
  require (distinct [ baker; cooper; fletcher; miller; smith ]);
  require (baker <> top_floor);
  require (cooper <> 1);
  require (fletcher <> top_floor);
  require (fletcher <> 1);
  require (miller > cooper);
  require (abs (smith - fletcher) <> 1);
  require (abs (fletcher - cooper) <> 1);
  show_dwelling baker cooper fletcher miller smith

let multiple_dwelling_pruned top_floor =
  let floors = [ 1; 2; 3; 4; top_floor ] in
  let fletcher = an_element_of (without 1 (without top_floor floors)) in
  let cooper = an_element_of (without 1 (without fletcher floors)) in
  require (abs (fletcher - cooper) <> 1);
  let miller = an_element_of (without cooper (without fletcher floors)) in
  require (miller > cooper);
  let baker =
    an_element_of (without top_floor (without miller (without cooper (without fletcher floors))))
  in
  let smith =
    an_element_of (without baker (without miller (without cooper (without fletcher floors))))
  in
  require (abs (smith - fletcher) <> 1);
  show_dwelling baker cooper fletcher miller smith
|}
;;

let run label body = label :: transcript (library ^ "\nlet () = " ^ body ^ "\n")

let ex_4_40 () =
  run "before distinct" "let _ = assignments 5 in ()"
  @ run "after distinct" "distinct_assignments 5"
  @ run "multiple_dwelling" "print_endline (multiple_dwelling 5)"
  @ run "multiple_dwelling_pruned" "print_endline (multiple_dwelling_pruned 5)"
;;
