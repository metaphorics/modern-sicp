(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.70: the [let] bindings in [add-assertion!] and
    [add-rule!]. The new THE-ASSERTIONS stream's tail is a delayed
    expression that names THE-ASSERTIONS itself; the [let] snapshots the
    old stream before the assignment, so the tail closes over the old
    value. With the broken [set!] version the tail is forced only after
    the assignment, when THE-ASSERTIONS already names the new stream:
    the data base contains itself, exactly the [(define ones
    (cons-stream 1 ones))] shape of 3.5.2 -- harmless where every head
    is identical, fatal for a data base, where every element after the
    first repeats the new assertion and the stored history is
    unreachable. The demonstration models THE-ASSERTIONS as the chapter
    3 memoized stream behind one [ref] cell and adds the assertion [c]
    to a data base holding [(a b)] both ways. This edition's data base
    is an eager list: [add_assertion] reads [!the_assertions] into
    [old_assertions], then assigns [old_assertions @ [assertion]] -- the
    right-hand side is evaluated before the cell is updated, the same
    old-value discipline with no laziness for the hazard to hide behind.
    The book's hazard would live in a memoized-stream data base whose
    tail thunk read THE-ASSERTIONS at force time. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Value = Sicp_common.Value

(** The book's THE-ASSERTIONS: a memoized stream behind one set!-able
    cell, here preloaded with the assertions [(a b)]. *)
let fresh_data_base () =
  ref
    (List.fold_right
       (fun name acc -> Streams.cons_stream (Value.symbol name) (fun () -> acc))
       [ "a"; "b" ]
       Streams.the_empty_stream)
;;

let take_show n db =
  String.concat " " (List.map Value.to_string (Streams.stream_take n !db))
;;

(** The book's broken listing: [(set! THE-ASSERTIONS (cons-stream
    assertion THE-ASSERTIONS))]. The tail thunk reads the cell, and the
    read happens at force time, after the assignment has rebound it. *)
let add_assertion_broken db assertion =
  db := Streams.cons_stream (Value.symbol assertion) (fun () -> !db)
;;

(** The book's repair: [(let ((old-assertions THE-ASSERTIONS)) (set!
    THE-ASSERTIONS (cons-stream assertion old-assertions)))]. *)
let add_assertion_with_let db assertion =
  let old_assertions = !db in
  db := Streams.cons_stream (Value.symbol assertion) (fun () -> old_assertions)
;;

(** The 3.5.2 [ones] built the broken way: the self-reference goes
    through the cell, so the tail is the stream itself. Uniform heads
    make the cycle invisible -- the intended 1 1 1 1 ... -- which is why
    the hint's example does not display the bug the data base suffers. *)
let ones_take4 () =
  let ones = ref Streams.the_empty_stream in
  ones := Streams.cons_stream (Value.int 1) (fun () -> !ones);
  take_show 4 ones
;;

let ex_4_70 () =
  let broken = fresh_data_base () in
  add_assertion_broken broken "c";
  let second_of_broken =
    Value.to_string (Streams.stream_car (Streams.stream_cdr !broken))
  in
  let repaired = fresh_data_base () in
  add_assertion_with_let repaired "c";
  [ "ones model (define ones (cons-stream 1 ones)): take(4) = "
    ^ ones_take4 ()
    ^ " -- uniform heads hide the cycle"
  ; "broken add-assertion! on (a b): add c => take(4) = " ^ take_show 4 broken
  ; "broken: element 2 = element 1 = "
    ^ second_of_broken
    ^ " -- the stream contains itself; the stored (a b) is unreachable"
  ; "let-bound add-assertion! on (a b): add c => take(4) = " ^ take_show 4 repaired
  ; "edition discipline: add_assertion binds old_assertions = !the_assertions, then \
     assigns old_assertions @ [assertion]; the right side is evaluated before the cell \
     is updated"
  ; "the book's hazard lives in a memoized-stream data base whose tail thunk reads \
     THE-ASSERTIONS at force time, after set! has rebound the name"
  ]
;;
