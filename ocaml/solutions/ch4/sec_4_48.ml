(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.48: extending the grammar. Adjectives join the noun
    phrase: [parse-modifiers] ambiguously produces the empty modifier
    list or one adjective followed by more modifiers, and the simple
    noun phrase splices the modifiers before the noun with [append].
    The demonstration parses ``the sleepy cat eats'' -- the adjective
    parse is found first because the empty-modifier alternative cannot
    consume the input -- and then walks to the extension-free parse of
    the same sentence's reordering with [try_again]. *)

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
(define adjectives '(adj quick brown sleepy))
(define prepositions '(prep for to in by with))
(define (parse-word word-list)
  (require (not (null? *unparsed*)))
  (require (member (car *unparsed*) (cdr word-list)))
  (let ((found-word (car *unparsed*)))
    (set! *unparsed* (cdr *unparsed*))
    (list (car word-list) found-word)))
(define (parse-modifiers)
  (amb '()
       (let ((m (parse-word adjectives)))
         (cons m (parse-modifiers)))))
(define (parse-simple-noun-phrase)
  (list 'simple-noun-phrase
        (parse-word articles)
        (append (parse-modifiers) (list (parse-word nouns)))))
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

let ex_4_48 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let sleepy = Eval.run env "(parse '(the sleepy cat eats))" in
  let sleepy_pp = Eval.try_again () in
  let brown = Eval.run env "(parse '(the quick brown cat sleeps))" in
  List.map show [ sleepy; sleepy_pp; brown ]
;;
