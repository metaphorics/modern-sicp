(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the explicit-control evaluator runs the book's
   5.4.4 session, and every result the adapted prose displays is
   proved with [expect] -- the plain driver, then the monitored driver
   whose stack statistics the prose quotes. *)

module Eval = Sicp_ch5.Sec_5_4
module Replay = Sicp_ch1.Replay

let ( >>= ) = Result.bind

let factorial_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))
(factorial 5)|}
;;

(* The queue-dry end of the driver loop is the normal end. *)
let run ~controller source =
  Eval.make_evaluator ~controller ~source ()
  >>= fun m ->
  let ended =
    match Eval.start m with
    | Ok () -> Ok ()
    | Error e when Eval.error_to_string e = "operation failed: " ^ Eval.input_exhausted ->
      Ok ()
    | Error e -> Error e
  in
  ended >>= fun () -> Ok (Eval.transcript m)
;;

(* The book's 5.4.4 session on the plain driver: the definition, then
   the call, value 120. *)
let driver_session () =
  run ~controller:Eval.base_controller factorial_source
  >>= fun lines ->
  Replay.expect
    (String.concat "|" lines)
    ";;; EC-Eval input:|;;; EC-Eval value:|ok|;;; EC-Eval input:|;;; EC-Eval \
     value:|120|;;; EC-Eval input:";
  Ok ()
;;

(* The book's monitored driver of the performance subsection: the
   definition costs (total-pushes = 3 maximum-depth = 3) and the call
   answers (total-pushes = 144 maximum-depth = 28). *)
let monitored_session () =
  let controller =
    String.concat
      "\n"
      (List.map
         (fun (name, text) ->
            if name = "driver" then Sicp_ch5_solutions.Sec_5_26.monitored_driver else text)
         Eval.controller_fragments)
  in
  run ~controller factorial_source
  >>= fun lines ->
  Replay.expect
    (String.concat "|" lines)
    ";;; EC-Eval input:|(total-pushes = 3 maximum-depth = 3)|;;; EC-Eval value:|ok|;;; \
     EC-Eval input:|(total-pushes = 144 maximum-depth = 28)|;;; EC-Eval value:|120|;;; \
     EC-Eval input:";
  Ok ()
;;

(* The book's append session on the plain driver. *)
let append_session () =
  run
    ~controller:Eval.base_controller
    {|
(define (append x y)
  (if (null? x)
      y
      (cons (car x) (append (cdr x) y))))
(append '(a b c) '(d e f))|}
  >>= fun lines ->
  print_endline ("append: " ^ String.concat "|" lines);
  Ok ()
;;

let () =
  match
    driver_session () >>= fun () -> monitored_session () >>= fun () -> append_session ()
  with
  | Ok () -> print_endline "replay ok"
  | Error e -> print_endline ("replay failed: " ^ Eval.error_to_string e)
;;
