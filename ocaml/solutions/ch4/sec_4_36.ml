(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.36: unbounded Pythagorean triples. The section's search
   experiment walks every branch depth first, so an unbounded generator
   can only be run under a ceiling; the ceiling is what makes the
   fairness of an enumeration observable. The fair procedure grows the
   hypotenuse outermost and searches each finite hypotenuse completely:
   raising its ceiling only appends answers, so every triple sits at a
   position no ceiling changes and a search without ceiling reaches it
   after finitely many answers. The naive replacement of 4.35's
   generators by one starting from [low] keeps [i] outermost: raising
   the ceiling inserts new triples in front of old ones, and without a
   ceiling the search never leaves the first [i]. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program ceiling procedure =
  {|
let rec an_integer_between low high =
  require (low <= high);
  amb low (an_integer_between (low + 1) high)

let ceiling = |}
  ^ string_of_int ceiling
  ^ {|

let an_integer_starting_from n = an_integer_between n ceiling

let show_triple i j k =
  "(" ^ string_of_int i ^ " " ^ string_of_int j ^ " " ^ string_of_int k ^ ")"

let fair_triple_from low =
  let k = an_integer_starting_from low in
  let i = an_integer_between low (k - 1) in
  let j = an_integer_between i (k - 1) in
  require (i * i + j * j = k * k);
  show_triple i j k

let naive_triple_from low =
  let i = an_integer_starting_from low in
  let j = an_integer_starting_from i in
  let k = an_integer_starting_from j in
  require (i * i + j * j = k * k);
  show_triple i j k

let () = print_endline (|}
  ^ procedure
  ^ {| 1)
|}
;;

let ex_4_36 () =
  let run ceiling procedure =
    (procedure ^ " ceiling " ^ string_of_int ceiling)
    :: transcript (program ceiling procedure)
  in
  run 20 "fair_triple_from"
  @ run 30 "fair_triple_from"
  @ run 20 "naive_triple_from"
  @ run 30 "naive_triple_from"
;;
