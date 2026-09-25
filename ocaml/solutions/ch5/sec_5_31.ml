(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.31: which of the evaluator's saves are superfluous for
    four combinations.  The compiler answers by construction: each
    compilation's [preserving] emits exactly the saves the register
    analysis demands, so the answer is the compiler's own output. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** The four combinations of the exercise, in order. *)
let combinations = [ "(f 'x 'y)"; "((f) 'x 'y)"; "(f (g 'x) y)"; "(f (g 'x) 'y)" ]

(** [saves_of src] is the [save]/[restore] pairs of the combination's
    compilation, as the machine reads them, one instruction per line. *)
let saves_of state src =
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile C.default_config state [] exp "val" C.Next
    >>= fun seq ->
    Ok
      (List.filter
         (fun s ->
            String.length s > 5
            && (String.sub s 0 5 = "(save" || String.sub s 0 8 = "(restore"))
         seq.stmts)
;;

let render lines = String.concat "; " lines

(** [ex_5_31 ()] answers, for each combination, which of the
    evaluator's saves the compiler keeps: (a) [f], [x] and [y] are all
    symbols, so no evaluation can change [env], [argl], [proc], or
    [continue] and every save is superfluous; (b) the operator
    [(f)] is itself a call, so [env] and [continue] must survive it;
    (c) the operand [(g 'x)] is a call, so [env] and [argl] (and
    [continue]) are preserved around it, while the last operand [y]
    needs nothing; (d) like (c) with a constant last operand. *)
let ex_5_31 () =
  let state = C.new_state () in
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | src :: rest ->
      saves_of state src
      >>= fun saves -> go (Printf.sprintf "%s: %s" src (render saves) :: acc) rest
  in
  go [] combinations >>= fun lines -> Ok lines
;;
