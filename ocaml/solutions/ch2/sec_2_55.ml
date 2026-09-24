(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.55 *)

(** Exercise 2.55 (this edition's replacement): Eva Lu Ator's surprise
    becomes a reading exercise in the shared Scheme subset's own
    terms. The subset's @code{Reader} turns the abbreviation ['x] into
    a [Quote] node wrapping the datum [(quote x)]; reading
    [''abracadabra] therefore quotes the list [(quote abracadabra)].
    Unwrapping the outer quote and taking the [car] of what is left
    finds the symbol [quote] itself, which is exactly what the
    interpreter's REPL prints back. *)

let car = function
  | Sicp_common.Ast.DPair (a, _) -> a
  | _ -> invalid_arg "car: not a pair"
;;

let name_of = function
  | Sicp_common.Ast.DSymbol s -> s
  | _ -> invalid_arg "name_of: not a symbol"
;;

(** [quoted_datum text] is the datum a top-level [Quote] node wraps,
    after reading [text] with the shared subset's reader.
    [invalid_arg] when [text] fails to read or does not read as a
    quote. *)
let quoted_datum text =
  match Sicp_common.Reader.read text with
  | Error e -> invalid_arg ("quoted_datum: " ^ Sicp_common.Reader.to_string e)
  | Ok expr ->
    (match Sicp_common.Ast.view expr with
     | Quote datum -> datum
     | _ -> invalid_arg "quoted_datum: not a quote")
;;

(** [ex_2_55 ()] is the name of the symbol the interpreter prints back
    for [''abracadabra]. *)
let ex_2_55 () = name_of (car (quoted_datum "''abracadabra"))
