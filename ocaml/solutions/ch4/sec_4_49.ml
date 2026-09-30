(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.49: generating sentences. Alyssa's [parse_word] ignores
   the input and draws its word from [an_element_of] over the word list,
   so the grammar generates instead of parsing. The generation is
   infinite and the section's experiment walks every branch, so the
   demonstration caps each sentence at [phrase_limit] prepositional
   phrases and reports the first six sentences with the counts of the
   whole capped search. The six show the footnote's complaint: depth
   first search varies only its latest choice, so after "the student
   studies" and one phrase it keeps the same skeleton and walks the
   last noun phrase's words. Under the cap the verb first changes at
   sentence 42 and the subject at sentence 165 of 2592. *)

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
let phrase_limit = 1

let rec an_element_of items =
  match items with
  | [] -> require false; ""
  | x :: rest -> amb x (an_element_of rest)

let parse_word category words = Word (category, an_element_of words)

let parse_simple_noun_phrase phrases =
  Node ("simple-noun-phrase", [ parse_word "article" articles; parse_word "noun" nouns ])

let rec parse_noun_phrase phrases =
  let rec maybe_extend noun_phrase =
    amb
      noun_phrase
      (maybe_extend (Node ("noun-phrase", [ noun_phrase; parse_prepositional_phrase phrases ])))
  in
  maybe_extend (parse_simple_noun_phrase phrases)

and parse_prepositional_phrase phrases =
  phrases := !phrases + 1;
  require (!phrases <= phrase_limit);
  Node ("prep-phrase", [ parse_word "prep" prepositions; parse_noun_phrase phrases ])

let parse_verb_phrase phrases =
  let rec maybe_extend verb_phrase =
    amb
      verb_phrase
      (maybe_extend (Node ("verb-phrase", [ verb_phrase; parse_prepositional_phrase phrases ])))
  in
  maybe_extend (parse_word "verb" verbs)

let generate phrases =
  Node ("sentence", [ parse_noun_phrase phrases; parse_verb_phrase phrases ])

let () = print_endline (show (generate (ref 0)))
|}
;;

let ex_4_49 () =
  let lines = transcript program in
  let sentences = List.filter (fun line -> not (String.contains line ':')) lines in
  let counts = List.filter (fun line -> String.contains line ':') lines in
  List.filteri (fun index _ -> index < 6) sentences @ counts
;;
