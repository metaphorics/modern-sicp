(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.70: the [let] bindings in [add-assertion!] and
   [add-rule!].  The new THE-ASSERTIONS stream's tail is a delayed
   expression that names THE-ASSERTIONS itself; the [let] snapshots the
   old stream before the assignment, so the tail closes over the old
   value.  With the broken [set!] version the tail is forced only after
   the assignment, when THE-ASSERTIONS already names the new stream: the
   data base contains itself, exactly the [ones] shape of 3.5.2 --
   harmless where every head is identical, fatal for a data base, where
   every element after the first repeats the new assertion and the
   stored history is unreachable.  The demonstration models
   THE-ASSERTIONS as the chapter 3 memoized stream of assertion terms
   behind one [ref] cell and adds the assertion [c] to a data base
   holding [a] and [b] both ways.

   This edition's session keeps its assertions in an immutable list
   field: [add_assertion] assigns [session.assertions @ [t]], and the
   right-hand side is evaluated before the field is updated -- the same
   old-value discipline, with no laziness for the hazard to hide
   behind. *)

open Sec_4_55.Kit

let fresh_data_base () =
  ref
    (List.fold_right
       (fun name acc -> Streams.cons_stream (at name) (fun () -> acc))
       [ "a"; "b" ]
       Streams.the_empty_stream)
;;

let take_show k db = String.concat " " (List.map Q.render_term (take k !db))

(* The book's broken listing: the tail thunk reads the cell at force
   time, after the assignment has rebound it. *)
let add_assertion_broken db name = db := Streams.cons_stream (at name) (fun () -> !db)

(* The book's repair: the old stream is bound before the assignment. *)
let add_assertion_with_let db name =
  let old_assertions = !db in
  db := Streams.cons_stream (at name) (fun () -> old_assertions)
;;

(* The 3.5.2 [ones] built the broken way: uniform heads make the cycle
   invisible, which is why the hint's example does not display the bug
   the data base suffers. *)
let ones_take4 () =
  let ones = ref Streams.the_empty_stream in
  ones := Streams.cons_stream (n 1) (fun () -> !ones);
  take_show 4 ones
;;

let ex_4_70 () =
  let broken = fresh_data_base () in
  add_assertion_broken broken "c";
  let second_of_broken =
    Q.render_term (Streams.stream_car (Streams.stream_cdr !broken))
  in
  let repaired = fresh_data_base () in
  add_assertion_with_let repaired "c";
  [ "ones model (define ones (cons-stream 1 ones)): take(4) = "
    ^ ones_take4 ()
    ^ " -- uniform heads hide the cycle"
  ; "broken add-assertion! on a b: add c => take(4) = " ^ take_show 4 broken
  ; "broken: element 2 = element 1 = "
    ^ second_of_broken
    ^ " -- the stream contains itself; the stored a b is unreachable"
  ; "let-bound add-assertion! on a b: add c => take(4) = " ^ take_show 4 repaired
  ; "edition discipline: add_assertion assigns session.assertions @ [t]; the right side \
     is evaluated before the field is updated"
  ; "the book's hazard lives in a memoized-stream data base whose tail thunk reads \
     THE-ASSERTIONS at force time, after set! has rebound the name"
  ]
;;
