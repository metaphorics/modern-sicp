(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.47: Louis's [parse_verb_phrase]. Louis's version finds the
   same parses as the section's, because [amb] evaluates only the
   alternative the search tries. But once those parses are found the
   search keeps trying the second alternative, whose recursive call
   consumes no word before it chooses again: the search descends
   forever and never reports exhaustion. The section's experiment walks
   the whole search, so the demonstration runs Louis's procedure under
   a recursion ceiling [depth_limit]. With the ceiling the search ends,
   the parses are the section's, and the failed branches grow with the
   ceiling: without one the search does not end. Interchanging the two
   expressions of the [amb] makes the recursion the first alternative,
   so the search descends to the ceiling before any word is consumed;
   the parses then arrive in a different order, only after every deeper
   branch has failed, and without a ceiling no parse arrives at all. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program ~depth_limit ~interchanged ~words =
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
let depth_limit = |}
  ^ string_of_int depth_limit
  ^ {|
let interchanged = |}
  ^ string_of_bool interchanged
  ^ {|

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

let rec parse_verb_phrase input depth =
  require (depth <= depth_limit);
  if interchanged then
    amb
      (Node ("verb-phrase", [ parse_verb_phrase input (depth + 1); parse_prepositional_phrase input ]))
      (parse_word "verb" verbs input)
  else
    amb
      (parse_word "verb" verbs input)
      (Node ("verb-phrase", [ parse_verb_phrase input (depth + 1); parse_prepositional_phrase input ]))

let parse words =
  let input = ref words in
  let sentence = Node ("sentence", [ parse_noun_phrase input; parse_verb_phrase input 0 ]) in
  require (match !input with [] -> true | _ :: _ -> false);
  sentence

let () = print_endline (show (parse |}
  ^ words
  ^ {|))
|}
;;

let cat_eats = {|[ "the"; "cat"; "eats" ]|}

let professor =
  {|[ "the"; "professor"; "lectures"; "to"; "the"; "student"; "with"; "the"; "cat" ]|}
;;

let run ~depth_limit ~interchanged ~words label =
  (label ^ " depth_limit " ^ string_of_int depth_limit)
  :: transcript (program ~depth_limit ~interchanged ~words)
;;

let ex_4_47 () =
  run ~depth_limit:4 ~interchanged:false ~words:cat_eats "louis cat-eats"
  @ run ~depth_limit:8 ~interchanged:false ~words:cat_eats "louis cat-eats"
  @ run ~depth_limit:4 ~interchanged:false ~words:professor "louis professor"
  @ run ~depth_limit:8 ~interchanged:false ~words:professor "louis professor"
  @ run ~depth_limit:4 ~interchanged:true ~words:professor "interchanged professor"
  @ run ~depth_limit:8 ~interchanged:true ~words:professor "interchanged professor"
;;
