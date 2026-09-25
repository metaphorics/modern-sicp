(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.47: Louis Reasoner's [parse-verb-phrase]. Louis's
    version works for parsing because the evaluator evaluates an
    [amb]'s alternatives lazily, one at a time only when they are
    tried: the recursive call inside the second alternative is entered
    only after the first alternative -- the bare verb -- has failed.
    Interchanging the two expressions in the [amb] would diverge, since
    the recursion would then be tried before any word is consumed. The
    demonstration runs Louis's version against the section's grammar
    and pins the same parses the original produces. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let program =
  {|
(define (require p) (if (not p) (amb)))
(define *unparsed* '())
(define nouns '(noun student professor cat class))
(define verbs '(verb studies lectures eats sleeps))
(define articles '(article the a))
(define prepositions '(prep for to in by with))
(define (parse-word word-list)
  (require (not (null? *unparsed*)))
  (require (member (car *unparsed*) (cdr word-list)))
  (let ((found-word (car *unparsed*)))
    (set! *unparsed* (cdr *unparsed*))
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
  (amb (parse-word verbs)
       (list 'verb-phrase
             (parse-verb-phrase)
             (parse-prepositional-phrase))))
(define (parse-sentence)
  (list 'sentence (parse-noun-phrase) (parse-verb-phrase)))
(define (parse input)
  (set! *unparsed* input)
  (let ((sent (parse-sentence))) (require (null? *unparsed*)) sent))|}
;;

let ex_4_47 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let cat_eats = Eval.run env "(parse '(the cat eats))" in
  let with_pp =
    Eval.run env "(parse '(the professor lectures to the student with the cat))"
  in
  let with_pp_again = Eval.try_again () in
  List.map show [ cat_eats; with_pp; with_pp_again ]
;;
