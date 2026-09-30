(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.48: extending the grammar with adjectives.
   [parse_modifiers] ambiguously answers no modifiers, or an adjective
   followed by more modifiers, and the simple noun phrase groups the
   modifiers with its noun, printed as one list inside the phrase:
   [((adj sleepy) (noun cat))]. On "the sleepy cat eats" the
   empty-modifier alternative fails at the noun and the adjective parse
   is the only answer; "the quick brown cat sleeps" parses with two
   adjectives. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program words =
  {|
type tree =
  | Word of string * string
  | Node of string * tree list
  | Group of tree list

let rec show tree =
  match tree with
  | Word (category, word) -> "(" ^ category ^ " " ^ word ^ ")"
  | Node (label, parts) -> "(" ^ label ^ show_parts parts ^ ")"
  | Group parts -> "(" ^ show_items parts ^ ")"

and show_parts parts =
  match parts with
  | [] -> ""
  | part :: rest -> " " ^ show part ^ show_parts rest

and show_items parts =
  match parts with
  | [] -> ""
  | part :: rest -> show part ^ show_parts rest

let nouns = [ "student"; "professor"; "cat"; "class" ]
let verbs = [ "studies"; "lectures"; "eats"; "sleeps" ]
let articles = [ "the"; "a" ]
let adjectives = [ "quick"; "brown"; "sleepy" ]
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

let rec parse_modifiers input =
  amb
    []
    (let modifier = parse_word "adj" adjectives input in
     modifier :: parse_modifiers input)

let parse_simple_noun_phrase input =
  let article = parse_word "article" articles input in
  let modifiers = parse_modifiers input in
  let noun = parse_word "noun" nouns input in
  Node ("simple-noun-phrase", [ article; Group (List.append modifiers [ noun ]) ])

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

let parse words =
  let input = ref words in
  let sentence = Node ("sentence", [ parse_noun_phrase input; parse_verb_phrase input ]) in
  require (match !input with [] -> true | _ :: _ -> false);
  sentence

let () = print_endline (show (parse |}
  ^ words
  ^ {|))
|}
;;

let ex_4_48 () =
  transcript (program {|[ "the"; "sleepy"; "cat"; "eats" ]|})
  @ transcript (program {|[ "the"; "quick"; "brown"; "cat"; "sleeps" ]|})
;;
