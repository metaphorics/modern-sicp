(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.50: the [ramb] special form. [ramb] is a choice point like
   [amb] whose two alternatives are tried in an order the edition's
   seeded generator draws, instead of left first. The clause is added to
   the search evaluator by open recursion; every other form, and the
   search driver, are unchanged.

   The search restarts every attempt from the beginning, so a choice
   point must see the same order in every attempt that reaches it. The
   k-th [ramb] of an attempt is the same choice point in every attempt
   that shares its decisions so far, so it takes the k-th bit of one
   seeded stream: the order is random across choice points and stable
   across attempts. Seeds are fixed, which makes the demonstration
   reproducible.

   Alyssa's generation of exercise 4.49 always starts from "the student
   studies", because [amb] tries its left alternative first. With
   [ramb] in [an_element_of] and in both [maybe_extend]s, each seed
   generates a different first sentence, differing in article, noun,
   verb, and whether a phrase is attached. That is the help: each fresh
   problem samples the grammar instead of starting in one corner.
   [ramb] reorders the search without changing it: under the same cap of
   one prepositional phrase it still finds the same 2592 sentences
   through the same numbers of choice points and failures. *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval = Sicp_ch4.Sec_4_3
module Random = Sicp_common.Random

(* [ramb_eval seed] is the search evaluator with the [ramb] clause,
   drawing its orders from a stream seeded with [seed]. The k-th order
   bit is drawn once and kept, so a restarted attempt reads the bits
   its predecessors drew. *)
let ramb_eval seed : Eval.eval_t =
  let generator =
    match Random.create seed with
    | Ok generator -> generator
    | Error _ -> invalid_arg "Sec_4_50.ramb_eval: the seed is zero"
  in
  let order_bits = Hashtbl.create 64 in
  let rec order_bit k =
    match Hashtbl.find_opt order_bits k with
    | Some bit -> bit
    | None ->
      Hashtbl.replace order_bits (Hashtbl.length order_bits) (Random.random generator 2);
      order_bit k
  in
  let current_attempt = ref None in
  let rambs_reached = ref 0 in
  let next_order_bit search =
    (match !current_attempt with
     | Some attempt when attempt == search -> ()
     | Some _ | None ->
       current_attempt := Some search;
       rambs_reached := 0);
    let k = !rambs_reached in
    incr rambs_reached;
    order_bit k
  in
  let rec eval search env e =
    match Ast.view e with
    | Ast.Apply (head, [ first; second ]) ->
      (match Ast.view head with
       | Ast.Var "ramb" ->
         let flipped = next_order_bit search in
         let* taken = Eval.choose search 2 in
         eval search env (if taken = flipped then first else second)
       | _ -> Eval.open_eval ~self:eval search env e)
    | _ -> Eval.open_eval ~self:eval search env e
  in
  eval
;;

let forms = [ "ramb", "let ramb x _y = x" ]

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
  | x :: rest -> ramb x (an_element_of rest)

let parse_word category words = Word (category, an_element_of words)

let parse_simple_noun_phrase phrases =
  Node ("simple-noun-phrase", [ parse_word "article" articles; parse_word "noun" nouns ])

let rec parse_noun_phrase phrases =
  let rec maybe_extend noun_phrase =
    ramb
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
    ramb
      verb_phrase
      (maybe_extend (Node ("verb-phrase", [ verb_phrase; parse_prepositional_phrase phrases ])))
  in
  maybe_extend (parse_word "verb" verbs)

let generate phrases =
  Node ("sentence", [ parse_noun_phrase phrases; parse_verb_phrase phrases ])

let () = print_endline (show (generate (ref 0)))
|}
;;

let first_sentence_and_counts seed =
  let lines =
    Eval.run_with ~eval:(ramb_eval seed) ~forms program
    |> String.split_on_char '\n'
    |> List.filter (fun line -> line <> "")
  in
  let counts = List.filter (fun line -> String.contains line ':') lines in
  match List.filter (fun line -> not (String.contains line ':')) lines with
  | [] -> ("seed " ^ Int64.to_string seed) :: counts
  | first :: _ -> ("seed " ^ Int64.to_string seed) :: first :: counts
;;

let ex_4_50 () =
  List.concat_map
    (fun offset -> first_sentence_and_counts (Int64.add 20260925L (Int64.of_int offset)))
    [ 0; 1; 2; 3 ]
;;
