(* SPDX-License-Identifier: GPL-3.0-only *)

open Parsetree

type t =
  | Ctor of string * t list
  | Int of int
  | Float of float
  | String of string
  | List of t list
  | Tuple of t list
  | Record of (string * t) list

let ( let* ) = Result.bind

let at (loc : Location.t) detail =
  Error
    (Printf.sprintf
       "line %d, column %d: %s"
       loc.loc_start.pos_lnum
       (loc.loc_start.pos_cnum - loc.loc_start.pos_bol)
       detail)
;;

let all f items =
  List.fold_right
    (fun item acc ->
       let* acc = acc in
       let* v = f item in
       Ok (v :: acc))
    items
    (Ok [])
;;

let rec lower e =
  if e.pexp_attributes <> []
  then at e.pexp_loc "attributes are outside fixture data"
  else (
    match e.pexp_desc with
    | Pexp_constant { pconst_desc = Pconst_integer (s, None); _ } ->
      (match int_of_string_opt s with
       | Some n -> Ok (Int n)
       | None -> at e.pexp_loc "integer literal out of range")
    | Pexp_constant { pconst_desc = Pconst_float (s, None); _ } ->
      Ok (Float (float_of_string s))
    | Pexp_constant { pconst_desc = Pconst_string (s, _, None); _ } -> Ok (String s)
    | Pexp_constant _ ->
      at e.pexp_loc "only plain int, float, and string literals are data"
    | Pexp_construct ({ txt = Lident "[]"; _ }, None) -> Ok (List [])
    | Pexp_construct ({ txt = Lident "::"; _ }, Some arg) -> lower_list e.pexp_loc [] arg
    | Pexp_construct ({ txt = Lident name; _ }, None) -> Ok (Ctor (name, []))
    | Pexp_construct
        ( { txt = Lident name; _ }
        , Some { pexp_desc = Pexp_tuple parts; pexp_attributes = []; _ } ) ->
      let* fields = all lower_unlabelled parts in
      Ok (Ctor (name, fields))
    | Pexp_construct ({ txt = Lident name; _ }, Some arg) ->
      let* field = lower arg in
      Ok (Ctor (name, [ field ]))
    | Pexp_construct _ -> at e.pexp_loc "qualified constructors are outside fixture data"
    | Pexp_tuple parts ->
      let* parts = all lower_unlabelled parts in
      Ok (Tuple parts)
    | Pexp_record (fields, None) ->
      let* fields =
        all
          (fun ((label : Longident.t Location.loc), value) ->
             match label.txt with
             | Lident name ->
               let* value = lower value in
               Ok (name, value)
             | _ -> at label.loc "qualified record fields are outside fixture data")
          fields
      in
      Ok (Record fields)
    | Pexp_record (_, Some _) -> at e.pexp_loc "record update is outside fixture data"
    | _ ->
      at
        e.pexp_loc
        "only constructors, lists, tuples, records, and literals are fixture data")

and lower_unlabelled (label, e) =
  match label with
  | None -> lower e
  | Some _ -> at e.pexp_loc "labelled tuple components are outside fixture data"

and lower_list loc acc arg =
  match arg.pexp_desc with
  | Pexp_tuple [ (None, head); (None, tail) ] ->
    let* head = lower head in
    (match tail.pexp_desc with
     | Pexp_construct ({ txt = Lident "[]"; _ }, None) ->
       Ok (List (List.rev (head :: acc)))
     | Pexp_construct ({ txt = Lident "::"; _ }, Some next) ->
       lower_list loc (head :: acc) next
     | _ -> at loc "a list must end in []")
  | _ -> at loc "malformed list"
;;

let read ~filename text =
  let lexbuf = Lexing.from_string text in
  Lexing.set_filename lexbuf filename;
  match Parse.expression lexbuf with
  | e -> Result.map_error (fun detail -> filename ^ ": " ^ detail) (lower e)
  | exception exn ->
    (match Location.error_of_exn exn with
     | Some (`Ok report) ->
       Result.map_error
         (fun detail -> filename ^ ": " ^ detail)
         (at report.main.loc "the pinned OCaml parser rejects the fixture")
     | Some `Already_displayed | None ->
       Error (filename ^ ": the pinned OCaml parser rejects the fixture"))
;;

let describe = function
  | Ctor (name, _) -> "constructor " ^ name
  | Int _ -> "an integer"
  | Float _ -> "a float"
  | String _ -> "a string"
  | List _ -> "a list"
  | Tuple _ -> "a tuple"
  | Record _ -> "a record"
;;
