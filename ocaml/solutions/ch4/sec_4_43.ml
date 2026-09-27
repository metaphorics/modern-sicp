(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.43: the yacht puzzle. The daughters are chosen for the
    five fathers, Sir Barnacle's daughter is fixed to Melissa, the four
    given yachts keep their names and Parker's yacht takes the one
    remaining daughter's name, no father's yacht carries his own
    daughter's name, and Gabrielle's father owns the yacht named after
    Dr. Parker's daughter. The demonstration answers ``who is Lorna's
    father'' for the told puzzle, shows the told puzzle has one
    solution, then drops the premise that Mary Ann is Moore's daughter
    and enumerates: the open puzzle has two solutions and Lorna's
    father differs between them. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [yacht_program told] is the puzzle with the Mary Ann premise when
    [told], without it otherwise. Gabrielle's father's yacht answers
    the [cond] over the five fathers, and it must equal Parker's
    daughter. *)
let yacht_program told =
  let mary_ann_clause =
    if told then "(require (eq? moore 'mary-ann))" else "(require #t)"
  in
  {|(define (require p) (if (not p) (amb)))
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define daughters '(mary-ann gabrielle melissa rosalind lorna))
(define (yacht-puzzle)
  (let ((moore (an-element-of daughters))
        (downing (an-element-of daughters))
        (hall (an-element-of daughters))
        (barnacle (an-element-of daughters))
        (parker (an-element-of daughters))
        (yacht-parker (an-element-of daughters)))
    (require (distinct? (list moore downing hall barnacle parker)))
    |}
  ^ "    "
  ^ mary_ann_clause
  ^ "\n"
  ^ {|    (require (eq? barnacle 'melissa))
    (require (distinct? (list 'lorna 'rosalind 'melissa 'gabrielle yacht-parker)))
    (require (not (eq? yacht-parker parker)))
    (require (not (eq? 'lorna moore)))
    (require (not (eq? 'rosalind hall)))
    (require (not (eq? 'melissa downing)))
    (require (not (eq? 'gabrielle barnacle)))
    (require (eq? (cond ((eq? moore 'gabrielle) 'lorna)
                        ((eq? hall 'gabrielle) 'rosalind)
                        ((eq? downing 'gabrielle) 'melissa)
                        ((eq? barnacle 'gabrielle) 'gabrielle)
                        (else yacht-parker))
                  parker))
    (list 'lornas-father
          (cond ((eq? moore 'lorna) 'moore)
                ((eq? downing 'lorna) 'downing)
                ((eq? hall 'lorna) 'hall)
                ((eq? barnacle 'lorna) 'barnacle)
                ((eq? parker 'lorna) 'parker)
                (else 'unknown)))))|}
;;

let ex_4_43 () =
  let told_env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) =
    Eval.run_program told_env (yacht_program true)
  in
  let told_answer = Eval.run told_env "(yacht-puzzle)" in
  let told_exhausted = Eval.try_again () in
  let open_env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) =
    Eval.run_program open_env (yacht_program false)
  in
  let open_first = Eval.run open_env "(yacht-puzzle)" in
  let open_second = Eval.try_again () in
  let open_third = Eval.try_again () in
  [ told_answer; told_exhausted; open_first; open_second; open_third ] |> List.map show
;;
