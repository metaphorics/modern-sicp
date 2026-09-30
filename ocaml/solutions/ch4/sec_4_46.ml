(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.46: left-to-right operands. OCaml leaves the evaluation
   order of constructor and tuple operands unspecified, but the section's
   search experiment, like every teaching engine of the edition,
   evaluates them left to right, and the parser depends on it. The first
   demonstration makes the order observable: the left operand's choice
   point is made first, so it is the older one and the search varies it
   last, which gives the answer order (1 3) (1 4) (2 3) (2 4). The
   second runs the section's simple noun phrase twice on "the cat eats":
   once with its operands consumed in the evaluator's left-to-right
   order, which parses, and once with the right-to-left order written
   out by explicit [let] sequencing, under which the noun phrase looks
   for the noun before the article is consumed and no parse exists. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let operand_order =
  {|
let show_pair pair =
  match pair with
  | (left, right) -> "(" ^ string_of_int left ^ " " ^ string_of_int right ^ ")"

let () = print_endline (show_pair (amb 1 2, amb 3 4))
|}
;;

let parser noun_phrase =
  {|
type tree =
  | Word of string * string
  | Node of string * tree list

let rec show tree =
  match tree with
  | Word (category, word) -> "(" ^ category ^ " " ^ word ^ ")"
  | Node (label, parts) -> "(" ^ label ^ show_parts parts ^ ")"

and show_parts parts =
  match parts with
  | [] -> ""
  | part :: rest -> " " ^ show part ^ show_parts rest

let nouns = [ "student"; "professor"; "cat"; "class" ]
let verbs = [ "studies"; "lectures"; "eats"; "sleeps" ]
let articles = [ "the"; "a" ]

let parse_word category words input =
  match !input with
  | [] -> require false; Word (category, "")
  | word :: rest ->
    let rec listed candidates =
      match candidates with
      | [] -> false
      | candidate :: more -> candidate = word || listed more
    in
    require (listed words);
    input := rest;
    Word (category, word)

let left_to_right_noun_phrase input =
  Node ("simple-noun-phrase", [ parse_word "article" articles input; parse_word "noun" nouns input ])

let right_to_left_noun_phrase input =
  let noun = parse_word "noun" nouns input in
  let article = parse_word "article" articles input in
  Node ("simple-noun-phrase", [ article; noun ])

let parse words =
  let input = ref words in
  let sentence =
    Node ("sentence", [ |}
  ^ noun_phrase
  ^ {| input; parse_word "verb" verbs input ])
  in
  require (match !input with [] -> true | _ :: _ -> false);
  sentence

let () = print_endline (show (parse [ "the"; "cat"; "eats" ]))
|}
;;

let ex_4_46 () =
  ("operand order" :: transcript operand_order)
  @ ("left to right" :: transcript (parser "left_to_right_noun_phrase"))
  @ ("right to left" :: transcript (parser "right_to_left_noun_phrase"))
;;
