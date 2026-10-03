(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.42: the Liars puzzle. Each girl's letter holds exactly one
   true statement, which [exactly_one] requires; the search experiment
   prints every placement that survives and reports that the complete
   search found exactly one. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program =
  {|
let rec member place places =
  match places with
  | [] -> false
  | other :: rest -> other - place = 0 || member place rest

let rec distinct items =
  match items with
  | [] -> true
  | x :: rest -> (not (member x rest)) && distinct rest

let rec an_integer_between low high =
  require (low <= high);
  amb low (an_integer_between (low + 1) high)

let exactly_one x y = if x then require (not y) else require y

let liars low =
  let betty = an_integer_between low 5 in
  let ethel = an_integer_between low 5 in
  let joan = an_integer_between low 5 in
  let kitty = an_integer_between low 5 in
  let mary = an_integer_between low 5 in
  require (distinct [ betty; ethel; joan; kitty; mary ]);
  exactly_one (kitty = 2) (betty = 3);
  exactly_one (ethel = 1) (joan = 2);
  exactly_one (joan = 3) (ethel = 5);
  exactly_one (kitty = 2) (mary = 4);
  exactly_one (mary = 4) (betty = 1);
  "((betty " ^ string_of_int betty ^ ") (ethel " ^ string_of_int ethel
  ^ ") (joan " ^ string_of_int joan ^ ") (kitty " ^ string_of_int kitty
  ^ ") (mary " ^ string_of_int mary ^ "))"

let () = print_endline (liars 1)
|}
;;

let ex_4_42 () = transcript program
