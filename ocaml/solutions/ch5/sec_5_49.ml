(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.49: a read-compile-execute-print loop.  The host drives
    the loop and calls [compile] as an operation between machine runs
    -- the book's own suggested arrangement.  Each form is compiled
    into a block under the shared compile state, the one machine is
    assembled with every block chained through a driver that prompts,
    runs the form, and prints its value, so definitions persist in the
    one global environment the way the book's loop needs. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** [chain_driver entries] is the driver fragment that prompts, runs,
    and prints each compiled form in order: before every entry the
    driver prompts for input and points [continue] at the printing
    block that follows the form's code; after the last value the chain
    jumps to [machine-end], the label that closes the assembled
    controller. *)
let chain_driver entries =
  let last = List.length entries - 1 in
  let one index entry =
    Printf.sprintf
      {|  (perform (op prompt-for-input))
  (assign continue (label print-%d))
  (goto (label %s))
print-%d
  (perform (op announce-output))
  (perform (op user-print) (reg val))%s|}
      index
      entry
      index
      (if index = last then "\n  (goto (label machine-end))" else "")
  in
  ";; the read-compile-execute-print loop: the host compiled each\n\
  \ form; the chain runs them in order and prints each value\n"
  ^ "  (assign env (op get-global-environment))\n"
  ^ String.concat "\n" (List.mapi one entries)
;;

(** [loop forms] is the read-compile-execute-print session over [forms]:
    one line group per form, the value the machine printed. *)
let loop forms =
  let state = C.new_state () in
  let compile_one form = C.compile_block state form in
  let rec compile_all acc = function
    | [] -> Ok (List.rev acc)
    | form :: rest -> compile_one form >>= fun block -> compile_all (block :: acc) rest
  in
  compile_all [] forms
  >>= fun blocks ->
  if blocks = []
  then Ok []
  else (
    let entries = List.map fst blocks in
    let blocks_text = String.concat "\n" (List.map snd blocks) in
    let controller =
      String.concat
        "\n"
        (List.map
           (fun (nm, text) -> if nm = "driver" then chain_driver entries else text)
           C.eceval_fragments)
      ^ "\n"
      ^ blocks_text
      ^ "\nmachine-end"
    in
    C.make_compiled_evaluator ~controller ~source:"" ~state ()
    >>= fun m ->
    (match C.start m with
     | Ok () -> Ok ()
     | Error e -> Error e)
    >>= fun () ->
    let lines = C.transcript m in
    let expected = 3 * List.length forms in
    if List.length lines <> expected
    then Error (C.Op_failed "the session printed an unexpected line count")
    else (
      let rec chunk acc = function
        | a :: b :: v :: rest -> chunk ([ a; b; v ] :: acc) rest
        | [] -> Ok (List.rev acc)
        | _ -> Error (C.Op_failed "the session lines do not group by form")
      in
      chunk [] lines))
;;

(** [ex_5_49 ()] runs the loop over a definition and two calls: the
    compiled definitions answer [ok] and the calls answer their
    values, all without an interpreter anywhere in the path. *)
let ex_5_49 () =
  loop
    [ "(define (square n) (* n n))"
    ; "(square 12)"
    ; "(define (twice n) (+ n n))"
    ; "(twice (square 21))"
    ]
  >>= fun sessions -> Ok (List.map (fun lines -> String.concat " " lines) sessions)
;;
