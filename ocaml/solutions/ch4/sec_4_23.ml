(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.23 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

type execution = Env.t -> (Value.t, Eval_error.t) result

type counts =
  { mutable analysis_steps : int
  ; mutable runtime_steps : int
  }

type style =
  | Book
  | Alyssa

let empty_sequence = Error (Eval_error.Invalid_form "empty sequence")

let rec flatten e =
  match Ast.view e with
  | Ast.Sequence (first, second) -> flatten first @ flatten second
  | _ -> [ e ]
;;

let counted counts (proc : execution) : execution =
  fun env ->
  counts.runtime_steps <- counts.runtime_steps + 1;
  proc env
;;

let book_sequence counts (procs : execution list) =
  let sequentially (first : execution) (second : execution) : execution =
    fun env ->
    let* _ = first env in
    second env
  in
  let rec loop first = function
    | [] -> first
    | second :: rest ->
      counts.analysis_steps <- counts.analysis_steps + 1;
      loop (sequentially first second) rest
  in
  match procs with
  | [] -> empty_sequence
  | first :: rest -> Ok (loop first rest)
;;

let alyssa_sequence counts (procs : execution list) =
  let rec execute procs env =
    counts.runtime_steps <- counts.runtime_steps + 1;
    match procs with
    | [] -> Error (Eval_error.Invalid_form "empty sequence")
    | [ last ] -> last env
    | proc :: rest ->
      let* _ = proc env in
      execute rest env
  in
  match procs with
  | [] -> empty_sequence
  | _ -> Ok (fun env -> execute procs env)
;;

let analyze_sequence style counts e =
  let procs =
    List.map (fun proc -> counted counts proc) (List.map S.analyze (flatten e))
  in
  match style with
  | Book -> book_sequence counts procs
  | Alyssa -> alyssa_sequence counts procs
;;

let measure style source ~runs =
  let* e = Sec_4_1.open_expression [] source in
  let counts = { analysis_steps = 0; runtime_steps = 0 } in
  let* execution = analyze_sequence style counts e in
  let out = Buffer.create 16 in
  let env = S.the_global_environment ~emit:(Buffer.add_string out) () in
  let rec go n =
    if n = 0
    then Ok ()
    else
      let* _ = execution env in
      go (n - 1)
  in
  let* () = go runs in
  Ok (Buffer.contents out, counts)
;;

let ex_4_23 () =
  let line name style label source =
    match measure style source ~runs:3 with
    | Ok (out, counts) ->
      Printf.sprintf
        "%s, %s: output %s, %d analysis steps, %d runtime steps"
        name
        label
        out
        counts.analysis_steps
        counts.runtime_steps
    | Error err -> name ^ ": error: " ^ Eval_error.to_string err
  in
  let one = "print_string \"a\"" in
  let two = "print_string \"a\"; print_string \"b\"" in
  [ line "book" Book "one expression" one
  ; line "Alyssa" Alyssa "one expression" one
  ; line "book" Book "two expressions" two
  ; line "Alyssa" Alyssa "two expressions" two
  ]
;;
