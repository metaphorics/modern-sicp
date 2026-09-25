(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.50: the [ramb] special form. [ramb] searches its
    alternatives in a random order: the variant dispatch shuffles the
    analyzed alternatives with the edition's seeded generator and hands
    them to the same [choice] handler, so every other property of the
    search -- the undo trail, the try-again protocol -- is unchanged.
    The generator's seed is fixed, which makes the demonstration
    reproducible. Applied to Alyssa's generation of exercise 4.49,
    [ramb] samples the word choices instead of always descending the
    first alternative, so the generated sentences vary instead of
    walking one recursion; the demonstration bounds the phrase depth at
    two so the grammar's recursion cannot outrun the host stack. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Random = Sicp_common.Random
module Ast = Sicp_common.Ast
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** The seed of the demonstration's generator: fixed, so the shuffled
    searches are reproducible under test. *)
let seed = Int64.of_int 20260925

(** [shuffled items] is [items] in Fisher-Yates order under the seeded
    generator; a rejected seed falls back to the unshuffled list, which
    cannot occur for this fixed nonzero seed. *)
let shuffled items =
  match Random.create seed with
  | Error _ -> items
  | Ok gen ->
    let a = Array.of_list items in
    for i = Array.length a - 1 downto 1 do
      let j = Random.random gen (i + 1) in
      let t = a.(i) in
      a.(i) <- a.(j);
      a.(j) <- t
    done;
    Array.to_list a
;;

module rec Ramb_eval : sig
  val eval_k : Eval.eval_k
end = struct
  module C = Eval.Core (Ramb_eval)

  let eval_k exp env succeed =
    match Ast.view exp with
    | Ast.Application (operator, operands) ->
      (match Ast.view operator with
       | Ast.Variable "ramb" ->
         let alternatives =
           List.map (fun e env2 succ2 -> Ramb_eval.eval_k e env2 succ2) operands
         in
         Eval.choice (shuffled alternatives) env succeed
       | _ -> C.eval_k exp env succeed)
    | _ -> C.eval_k exp env succeed
  ;;
end

let eval exp env = Eval.drive (fun () -> Ramb_eval.eval_k exp env Eval.report)

let run env text =
  match Reader.read text with
  | Ok exp -> eval exp env
  | Error e -> Error (Eval_error.Invalid_form (Reader.to_string e))
;;

let run_program env text =
  match Reader.read_program text with
  | Error e -> Error (Eval_error.Invalid_form (Reader.to_string e))
  | Ok exps ->
    let rec go = function
      | [] -> Ok (Value.symbol "ok")
      | [ exp ] -> eval exp env
      | exp :: rest -> eval exp env >>= fun _ -> go rest
    in
    go exps
;;

let program =
  {|
(define (require p) (if (not p) (amb)))
(define (ramb-element-of items)
  (require (not (null? items)))
  (ramb (car items) (ramb-element-of (cdr items))))
(define *unparsed* '())
(define nouns '(noun student professor cat class))
(define verbs '(verb studies lectures eats sleeps))
(define articles '(article the a))
(define prepositions '(prep for to in by with))
(define (parse-word word-list)
  (let ((found-word (ramb-element-of (cdr word-list))))
    (list (car word-list) found-word)))
(define (parse-simple-noun-phrase)
  (list 'simple-noun-phrase (parse-word articles) (parse-word nouns)))
(define (parse-noun-phrase)
  (define (maybe-extend noun-phrase)
    (amb noun-phrase
         (maybe-extend
          (list 'noun-phrase noun-phrase (parse-prepositional-phrase)))))
  (maybe-extend (parse-simple-noun-phrase)))
(define (parse-prepositional-phrase)
  (list 'prep-phrase (parse-word prepositions) (parse-noun-phrase)))
(define (parse-verb-phrase)
  (define (maybe-extend verb-phrase)
    (amb verb-phrase
         (maybe-extend
          (list 'verb-phrase verb-phrase (parse-prepositional-phrase)))))
  (maybe-extend (parse-word verbs)))
(define (parse-sentence)
  (list 'sentence (parse-noun-phrase) (parse-verb-phrase)))
(define (parse input)
  (set! *unparsed* input)
  (let ((sent (parse-sentence))) (require (null? *unparsed*)) sent))|}
;;

let ex_4_50 () =
  let env = Eval.setup_environment () in
  let (_ : (Value.t, Eval_error.t) result) = run_program env program in
  let first = run env "(parse '())" in
  let second = Eval.try_again () in
  let third = Eval.try_again () in
  let fourth = Eval.try_again () in
  let fifth = Eval.try_again () in
  let sixth = Eval.try_again () in
  List.map show [ first; second; third; fourth; fifth; sixth ]
;;
