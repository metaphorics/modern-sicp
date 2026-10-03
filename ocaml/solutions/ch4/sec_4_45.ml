(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.45: the five parses of the professor's sentence. The
   section's grammar becomes a guest program over a closed parse-tree
   variant; the words not yet consumed live in a reference the parse
   threads through, and the search experiment rolls a failed branch's
   consumption back with the branch. A tree prints in the book's list
   notation. The complete search prints the five parses, which differ in
   where each prepositional phrase attaches, and finds no others. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program =
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
let prepositions = [ "for"; "to"; "in"; "by"; "with" ]

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

let parse_simple_noun_phrase input =
  Node ("simple-noun-phrase", [ parse_word "article" articles input; parse_word "noun" nouns input ])

let rec parse_noun_phrase input =
  let rec maybe_extend noun_phrase =
    amb
      noun_phrase
      (maybe_extend (Node ("noun-phrase", [ noun_phrase; parse_prepositional_phrase input ])))
  in
  maybe_extend (parse_simple_noun_phrase input)

and parse_prepositional_phrase input =
  Node ("prep-phrase", [ parse_word "prep" prepositions input; parse_noun_phrase input ])

let parse_verb_phrase input =
  let rec maybe_extend verb_phrase =
    amb
      verb_phrase
      (maybe_extend (Node ("verb-phrase", [ verb_phrase; parse_prepositional_phrase input ])))
  in
  maybe_extend (parse_word "verb" verbs input)

let parse_sentence input = Node ("sentence", [ parse_noun_phrase input; parse_verb_phrase input ])

let parse words =
  let input = ref words in
  let sentence = parse_sentence input in
  require (match !input with [] -> true | _ :: _ -> false);
  sentence

let () =
  print_endline
    (show
       (parse
          [ "the"; "professor"; "lectures"; "to"; "the"; "student"; "in"; "the"; "class"; "with"; "the"; "cat" ]))
|}
;;

let ex_4_45 () = transcript program
