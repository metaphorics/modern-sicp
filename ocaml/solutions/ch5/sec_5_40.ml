(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.40: the compiler maintains the compile-time environment.
    The edition's [compile] carries it from the start -- an argument of
    every code generator, extended by [compile-lambda-body] with the
    frame of the procedure's parameters -- so 5.40's modification is
    intrinsic.  The dump the exercise asks for is the compiler's
    [trace]: every variable reference reports the compile-time
    environment it was compiled against.  (5.41's [find-variable] and
    5.42's addressing land in the next two exercises; this module only
    watches the environment threading.) *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** The nested-lambda example at the start of 5.5.6. *)
let nested_example = "(define (f x y) (lambda (a b c d e) (lambda (y z) (+ x y z))))"

(** [render_cenv] is the compile-time environment as the book prints
    frames: newest first. *)
let render_cenv (frames : string list list) =
  String.concat " " (List.map (fun f -> "(" ^ String.concat " " f ^ ")") frames)
;;

(** [dump src] compiles [src] with the trace on and answers one line
    per variable reference: the name and its compile-time environment. *)
let dump src =
  let lines = ref [] in
  let cfg =
    { C.default_config with
      trace = Some (fun frames name -> lines := (render_cenv frames, name) :: !lines)
    }
  in
  let state = C.new_state () in
  match
    match Sicp_common.Reader.read src with
    | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
    | Ok exp -> C.compile cfg state [] exp "val" C.Next
  with
  | Error e -> Error e
  | Ok _ -> Ok (List.rev_map (fun (env, name) -> name ^ " in " ^ env) !lines |> List.rev)
;;

(** [ex_5_40 ()] dumps the compile-time environments of the example:
    [x] compiles in [(x y)], [z] in [(y z)] over [(a b c d e)] over
    [(x y)] -- the environments the book's text names. *)
let ex_5_40 () = dump nested_example >>= fun lines -> Ok lines
