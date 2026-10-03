(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.38: the multiple-dwelling puzzle without the
   Smith-Fletcher clause. The search experiment prints every solution
   of the loosened puzzle in search order and reports how many answers
   the complete search found: five. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program =
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

let multiple_dwelling_loosened top_floor =
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
  require (abs (fletcher - cooper) <> 1);
  show_dwelling baker cooper fletcher miller smith

let () = print_endline (multiple_dwelling_loosened 5)
|}
;;

let ex_4_38 () = transcript program
