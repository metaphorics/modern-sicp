(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.39: the order of the restrictions. The order cannot
    change the set of answers -- every restriction is imposed on every
    complete assignment -- but it changes the time: a restriction
    placed before the choices it does not involve is checked once per
    partial assignment instead of once per complete one. The
    demonstration runs the book's order and a reordered procedure that
    picks Fletcher first under its own restrictions, and the 4.44a
    counter measures both searches on the way to the same answer. *)

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
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5))
        (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5))
        (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))
    (require (not (= (abs (- smith fletcher)) 1)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(define (multiple-dwelling-reordered)
  (let ((fletcher (amb 1 2 3 4)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (let ((cooper (amb 1 2 3 4 5)))
      (require (not (= cooper 1)))
      (require (not (= (abs (- fletcher cooper)) 1)))
      (let ((baker (amb 1 2 3 4)))
        (require (not (= baker 5)))
        (let ((miller (amb 1 2 3 4 5)))
          (require (> miller cooper))
          (let ((smith (amb 1 2 3 4 5)))
            (require (not (= (abs (- smith fletcher)) 1)))
            (require (distinct? (list baker cooper fletcher miller smith)))
            (list (list 'baker baker) (list 'cooper cooper)
                  (list 'fletcher fletcher) (list 'miller miller)
                  (list 'smith smith))))))))|}
;;

let ex_4_39 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  Eval.reset_backtrack_count ();
  let book_order = show (Eval.run env "(multiple-dwelling)") in
  let book_count = "backtracks=" ^ string_of_int (Eval.backtrack_count ()) in
  Eval.reset_backtrack_count ();
  let reordered = show (Eval.run env "(multiple-dwelling-reordered)") in
  let reordered_count = "backtracks=" ^ string_of_int (Eval.backtrack_count ()) in
  [ book_order; book_count; reordered; reordered_count ]
;;
