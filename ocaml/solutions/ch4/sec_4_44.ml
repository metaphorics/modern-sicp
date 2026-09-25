(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.44 and the tailored addition 4.44a: the n-queens puzzle
    under [amb]. The board is a list of rows built column by column;
    each new column draws a row with [an-integer-between] and requires
    the placement safe against every earlier column before the next
    column is placed. Exercise 4.44 runs the puzzle for board size 8
    and pins the first solution. Exercise 4.44a turns the edition's
    backtrack counter -- incremented once for every delivery of
    [Fail] to a choice-point handler -- onto board sizes 4 to 6 and
    asserts the counts the independent simulation produces. *)

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
(define (an-integer-between low high)
  (require (not (> low high)))
  (amb low (an-integer-between (+ low 1) high)))
(define (attacks? row1 col1 row2 col2)
  (if (= row1 row2)
      #t
      (if (= (abs (- row1 row2)) (abs (- col1 col2))) #t #f)))
(define (safe-up-to? new-row new-col rows col)
  (if (null? rows)
      #t
      (if (attacks? new-row new-col (car rows) col)
          #f
          (safe-up-to? new-row new-col (cdr rows) (- col 1)))))
(define (queens board-size)
  (define (place col rows)
    (if (> col board-size)
        rows
        (let ((row (an-integer-between 1 board-size)))
          (require (safe-up-to? row col rows (- col 1)))
          (place (+ col 1) (cons row rows)))))
  (place 1 '()))|}
;;

let ex_4_44 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let eight = Eval.run env "(queens 8)" in
  let four = Eval.run env "(queens 4)" in
  let six = Eval.run env "(queens 6)" in
  List.map show [ eight; four; six ]
;;

(** [ex_4_44a ()] asserts the backtrack counts of the first search for
    board sizes 4 to 6: a backtrack is one delivery of [Fail] to a
    choice-point handler, the search re-entering a made choice to try
    its next alternative. When the choice has no alternative left the
    delivery still happened and counts, and the [Fail] propagates to
    the previous choice point. *)
let ex_4_44a () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let run_board n =
    Eval.reset_backtrack_count ();
    let answer = Eval.run env ("(queens " ^ string_of_int n ^ ")") in
    answer, Eval.backtrack_count ()
  in
  let answer4, count4 = run_board 4 in
  let answer5, count5 = run_board 5 in
  let answer6, count6 = run_board 6 in
  List.map show [ answer4; answer5; answer6 ]
  @ [ "backtracks(4)=" ^ string_of_int count4
    ; "backtracks(5)=" ^ string_of_int count5
    ; "backtracks(6)=" ^ string_of_int count6
    ]
;;
