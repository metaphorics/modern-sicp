(* SPDX-License-Identifier: GPL-3.0-only *)

open Parsetree

type diagnostic_kind =
  | Syntax
  | Unsupported
  | Type_error
  | Contract

type diagnostic =
  { kind : diagnostic_kind
  ; at : Ast.span
  ; detail : string
  }

type program =
  { items : Ast.item list
  ; at : Ast.span
  }

(* The unqualified prelude of grammar section 7. *)

let unqualified_prelude =
  [ "print_string"
  ; "print_endline"
  ; "print_int"
  ; "print_newline"
  ; "string_of_int"
  ; "string_of_float"
  ; "float_of_int"
  ; "sqrt"
  ]
;;

(* The qualified prelude applications of grammar sections 3 and 6. *)

let qualified_prelude =
  [ "Array.make"
  ; "Array.get"
  ; "Array.set"
  ; "Array.length"
  ; "List.map"
  ; "List.filter"
  ; "List.fold_left"
  ; "List.fold_right"
  ; "List.length"
  ; "List.rev"
  ; "List.append"
  ; "List.sort"
  ; "Hashtbl.create"
  ; "Hashtbl.find_opt"
  ; "Hashtbl.replace"
  ; "Hashtbl.remove"
  ; "Hashtbl.length"
  ]
;;

(* The constructors the prelude declares, with their payload arity. *)

let built_in_ctors = [ "[]", 0; "::", 2; "Some", 1; "None", 0; "Ok", 1; "Error", 1 ]

let built_in_types =
  [ "unit"; "int"; "float"; "bool"; "string"; "list"; "option"; "result"; "ref"; "array" ]
;;

let kind_to_string = function
  | Syntax -> "syntax-invalid"
  | Unsupported -> "host-valid, subset-unsupported"
  | Type_error -> "type-invalid"
  | Contract -> "contract-invalid"
;;

let diagnostic_to_string d =
  Printf.sprintf
    "%s at line %d, column %d: %s"
    (kind_to_string d.kind)
    d.at.Ast.start.Ast.line
    d.at.Ast.start.Ast.column
    d.detail
;;

let items p = p.items
let program_span p = p.at

let position_of_lexing (p : Lexing.position) : Ast.position =
  { line = p.pos_lnum; column = p.pos_cnum - p.pos_bol }
;;

let span_of_location (l : Location.t) : Ast.span =
  { start = position_of_lexing l.loc_start; stop = position_of_lexing l.loc_end }
;;

let dummy_span : Ast.span =
  { start = { line = 0; column = 0 }; stop = { line = 0; column = 0 } }
;;

let syntax_at at detail = Error { kind = Syntax; at; detail }
let unsupported at detail = Error { kind = Unsupported; at; detail }
let type_error at detail = Error { kind = Type_error; at; detail }
let contract at detail = Error { kind = Contract; at; detail }

module String_set = Set.Make (String)

let all_unit f xs =
  let rec go = function
    | [] -> Ok ()
    | x :: rest ->
      (match f x with
       | Ok () -> go rest
       | Error _ as e -> e)
  in
  go xs
;;

(* Stage 1: the subset lexer over raw source text.  The alphabet is
   ASCII, comments nest, and strings admit only the five escapes of the
   grammar. *)

type lex_state =
  | Normal
  | In_string
  | In_comment of int

let scan_lexical source =
  let n = String.length source in
  let state = ref Normal in
  let i = ref 0 in
  let line = ref 1 in
  let bol = ref 0 in
  let err = ref None in
  let here () : Ast.span =
    { start = { line = !line; column = !i - !bol }
    ; stop = { line = !line; column = !i - !bol }
    }
  in
  let fail_kind kind detail = if !err = None then err := Some (kind, here (), detail) in
  let fail detail = fail_kind Syntax detail in
  while !i < n && !err = None do
    let c = source.[!i] in
    if c = '\n'
    then (
      incr line;
      incr i;
      bol := !i)
    else (
      match !state with
      | Normal ->
        if Char.code c > 127
        then fail "non-ASCII byte outside the subset alphabet"
        else if c = '#'
        then (
          let only_space = ref true in
          let j = ref !bol in
          while !j < !i do
            (match source.[!j] with
             | ' ' | '\t' -> ()
             | _ -> only_space := false);
            incr j
          done;
          let k = ref (!i + 1) in
          while !k < n && (source.[!k] = ' ' || source.[!k] = '\t') do
            incr k
          done;
          if !only_space && !k < n && source.[!k] >= '0' && source.[!k] <= '9'
          then fail_kind Unsupported "line directives are outside the grammar"
          else incr i)
        else if c = '('
        then
          if !i + 1 < n && source.[!i + 1] = '*'
          then (
            state := In_comment 1;
            i := !i + 2)
          else incr i
        else if c = '"'
        then (
          state := In_string;
          incr i)
        else if c = '{'
        then
          if !i + 1 < n && source.[!i + 1] = '|'
          then fail "quoted string delimiters are outside the subset alphabet"
          else incr i
        else if c = '`'
        then fail "quotation is outside the subset alphabet"
        else incr i
      | In_string ->
        if Char.code c > 127
        then fail "non-ASCII byte outside the subset alphabet"
        else if c = '"'
        then (
          state := Normal;
          incr i)
        else if c = '\\'
        then
          if !i + 1 >= n
          then fail "string ends after a backslash"
          else (
            let e = source.[!i + 1] in
            match e with
            | '\\' | '"' | 'n' | 'r' | 't' -> i := !i + 2
            | _ -> fail "string escape outside the subset alphabet")
        else incr i
      | In_comment depth ->
        if c = '(' && !i + 1 < n && source.[!i + 1] = '*'
        then (
          state := In_comment (depth + 1);
          i := !i + 2)
        else if c = '*' && !i + 1 < n && source.[!i + 1] = ')'
        then (
          if depth = 1 then state := Normal else state := In_comment (depth - 1);
          i := !i + 2)
        else incr i)
  done;
  match !err, !state with
  | Some (kind, at, detail), _ -> Error { kind; at; detail }
  | None, In_comment _ -> syntax_at (here ()) "comment reaches end of input"
  | None, In_string -> syntax_at (here ()) "string reaches end of input"
  | None, Normal -> Ok ()
;;

(* Stage 2: the pinned parser.  Its syntax errors are subset syntax
   errors.  Unexpected exceptions are programming errors and propagate. *)

let parse filename source =
  let lexbuf = Lexing.from_string source in
  lexbuf.lex_curr_p <- { lexbuf.lex_curr_p with pos_fname = filename };
  match Parse.implementation lexbuf with
  | structure -> Ok structure
  | exception exn ->
    (match Location.error_of_exn exn with
     | Some (`Ok report) ->
       syntax_at
         (span_of_location report.main.loc)
         "the pinned OCaml parser rejects the source"
     | Some `Already_displayed ->
       syntax_at dummy_span "the pinned OCaml parser rejects the source"
     | None -> raise exn)
;;

type context =
  { ctors : (string * int) list ref
  ; types : String_set.t ref
  ; contracts : (Ast.span * string) list ref
  ; frees : (Ast.span * string * string) list ref
  ; names : String_set.t
  }

let note_contract ctx at detail = ctx.contracts := (at, detail) :: !(ctx.contracts)

let record_arity ctx at name arity =
  if List.mem_assoc name built_in_ctors
  then
    note_contract ctx at ("constructor " ^ name ^ " collides with a built-in constructor")
  else if List.mem_assoc name !(ctx.ctors)
  then note_contract ctx at ("constructor " ^ name ^ " is declared twice in the unit")
  else ctx.ctors := (name, arity) :: !(ctx.ctors)
;;

let constructor_arity ctx name =
  match List.assoc_opt name !(ctx.ctors) with
  | Some _ as found -> found
  | None -> List.assoc_opt name built_in_ctors
;;

let note_free ctx at kind name = ctx.frees := (at, kind, name) :: !(ctx.frees)

let is_value_name s =
  String.length s > 0
  && (match s.[0] with
      | 'a' .. 'z' | '_' -> true
      | _ -> false)
  && String.for_all
       (function
         | 'a' .. 'z' | 'A' .. 'Z' | '0' .. '9' | '_' | '\'' -> true
         | _ -> false)
       s
;;

let is_type_name s =
  String.length s > 0
  && (match s.[0] with
      | 'a' .. 'z' | '_' -> true
      | _ -> false)
  && String.for_all
       (function
         | 'a' .. 'z' | 'A' .. 'Z' | '0' .. '9' | '_' | '\'' -> true
         | _ -> false)
       s
;;

let is_constructor_name s =
  String.length s > 0
  && (match s.[0] with
      | 'A' .. 'Z' -> true
      | _ -> false)
  && String.for_all
       (function
         | 'a' .. 'z' | 'A' .. 'Z' | '0' .. '9' | '_' | '\'' -> true
         | _ -> false)
       s
;;

(* Type expressions are validated and discarded; the pinned type checker
   assigns them. *)

let rec validate_type ctx (ct : Parsetree.core_type) =
  match ct.ptyp_desc with
  | Ptyp_var name ->
    if is_type_name name
    then Ok ()
    else unsupported (span_of_location ct.ptyp_loc) "malformed type variable"
  | Ptyp_constr ({ txt = Lident name; _ }, args) ->
    if not (is_type_name name)
    then unsupported (span_of_location ct.ptyp_loc) "malformed type constructor name"
    else if List.mem name built_in_types || String_set.mem name !(ctx.types)
    then all_unit (validate_type ctx) args
    else (
      (* Either unbound, which the pinned type checker rejects, or a
         standard-library name outside the fixed surface.  Only typing
         can tell which, so record the occurrence and continue. *)
      note_free ctx (span_of_location ct.ptyp_loc) "type name" name;
      all_unit (validate_type ctx) args)
  | Ptyp_constr ({ txt = Ldot ({ txt = Lident "Hashtbl"; _ }, { txt = "t"; _ }); _ }, args)
    ->
    if List.length args = 2
    then all_unit (validate_type ctx) args
    else unsupported (span_of_location ct.ptyp_loc) "Hashtbl.t takes two arguments"
  | Ptyp_constr _ ->
    unsupported
      (span_of_location ct.ptyp_loc)
      "qualified type paths are outside the grammar"
  | Ptyp_arrow (Asttypes.Nolabel, a, b) ->
    (match validate_type ctx a with
     | Error e -> Error e
     | Ok () -> validate_type ctx b)
  | Ptyp_arrow _ ->
    unsupported
      (span_of_location ct.ptyp_loc)
      "labelled and optional arrows are outside the grammar"
  | Ptyp_tuple parts ->
    if List.exists (fun (label, _) -> label <> None) parts
    then
      unsupported
        (span_of_location ct.ptyp_loc)
        "labelled tuple fields are outside the grammar"
    else all_unit (validate_type ctx) (List.map snd parts)
  | _ ->
    unsupported
      (span_of_location ct.ptyp_loc)
      "type form outside the grammar's type_expr production"
;;

(* Literal lowering.  The pinned parser folds unary minus on a literal
   into the constant, so a leading minus is accepted here.  The subset
   literal productions admit decimal integers, decimal floats, and
   double-quoted strings only. *)

let is_digits s =
  String.length s > 0
  && String.for_all
       (function
         | '0' .. '9' -> true
         | _ -> false)
       s
;;

let strip_minus s =
  if String.length s > 0 && s.[0] = '-' then String.sub s 1 (String.length s - 1) else s
;;

let valid_int_literal s =
  let body = strip_minus s in
  String.length body > 0
  && String.for_all
       (function
         | '0' .. '9' -> true
         | _ -> false)
       body
;;

let split_float body =
  match String.index_opt body 'e' with
  | Some i ->
    Some (String.sub body 0 i, String.sub body (i + 1) (String.length body - i - 1))
  | None ->
    (match String.index_opt body 'E' with
     | Some i ->
       Some (String.sub body 0 i, String.sub body (i + 1) (String.length body - i - 1))
     | None -> None)
;;

let valid_exponent s = s <> "" && is_digits (strip_minus s)

let valid_float_literal s =
  let body = strip_minus s in
  if String.contains body '_' || body = ""
  then false
  else (
    match split_float body with
    | None ->
      (match String.index_opt body '.' with
       | None -> false
       | Some i ->
         let before = String.sub body 0 i in
         let after = String.sub body (i + 1) (String.length body - i - 1) in
         is_digits before && (after = "" || is_digits after))
    | Some (mantissa, exponent) ->
      valid_exponent exponent
      &&
        (match String.index_opt mantissa '.' with
        | None -> is_digits mantissa
        | Some i ->
          let before = String.sub mantissa 0 i in
          let after = String.sub mantissa (i + 1) (String.length mantissa - i - 1) in
          is_digits before && (after = "" || is_digits after)))
;;

let lower_scalar (c : Parsetree.constant) at : (Ast.scalar, diagnostic) result =
  match c.pconst_desc with
  | Pconst_integer (s, None) ->
    if valid_int_literal s
    then Ok (Ast.Int (int_of_string s))
    else syntax_at at "integer literal is outside the subset alphabet"
  | Pconst_integer (_, Some _) ->
    syntax_at at "suffixed integer literal is outside the subset alphabet"
  | Pconst_float (s, None) ->
    if valid_float_literal s
    then Ok (Ast.Float (float_of_string s))
    else syntax_at at "float literal is outside the subset alphabet"
  | Pconst_float (_, Some _) ->
    syntax_at at "suffixed float literal is outside the subset alphabet"
  | Pconst_string (s, _, None) -> Ok (Ast.String s)
  | Pconst_string (_, _, Some _) ->
    syntax_at at "quoted string delimiters are outside the subset alphabet"
  | Pconst_char _ ->
    unsupported at "character literals are outside the grammar's scalar forms"
;;

(* Pattern lowering.  The result carries the names the pattern binds. *)

let rec lower_pattern ctx (p : Parsetree.pattern) =
  let at = span_of_location p.ppat_loc in
  match p.ppat_desc with
  | Ppat_any -> Ok (Ast.wildcard ~at (), String_set.empty)
  | Ppat_var { txt = name; _ } ->
    if is_value_name name
    then Ok (Ast.pvar ~at name, String_set.singleton name)
    else unsupported at "binding name is not a value identifier"
  | Ppat_constant c ->
    (match lower_scalar c at with
     | Error e -> Error e
     | Ok s -> Ok (Ast.pscalar ~at s, String_set.empty))
  | Ppat_tuple (fields, Asttypes.Closed) when List.for_all (fun (l, _) -> l = None) fields
    ->
    (match lower_patterns ctx (List.map snd fields) with
     | Error e -> Error e
     | Ok (ps, names) -> Ok (Ast.ptuple ~at ps, names))
  | Ppat_tuple _ ->
    unsupported at "labelled and open tuple patterns are outside the grammar"
  | Ppat_construct ({ txt = Lident "[]"; _ }, None) ->
    Ok (Ast.pnil ~at (), String_set.empty)
  | Ppat_construct ({ txt = Lident "[]"; _ }, Some _) ->
    syntax_at at "the empty-list constructor takes no payload"
  | Ppat_construct ({ txt = Lident "::"; _ }, Some ([], arg)) ->
    (match arg.ppat_desc with
     | Ppat_tuple ([ (_, h); (_, t) ], Asttypes.Closed) ->
       (match lower_pattern ctx h with
        | Error e -> Error e
        | Ok (h, hn) ->
          (match lower_pattern ctx t with
           | Error e -> Error e
           | Ok (t, tn) -> Ok (Ast.pcons ~at h t, String_set.union hn tn)))
     | _ -> unsupported at "the cons pattern takes a pair payload")
  | Ppat_construct ({ txt = Lident "::"; _ }, _) ->
    syntax_at at "the cons pattern takes one pair payload"
  | Ppat_construct ({ txt = Lident "()"; _ }, None) ->
    Ok (Ast.pscalar ~at Ast.Unit, String_set.empty)
  | Ppat_construct ({ txt = Lident "true"; _ }, None) ->
    Ok (Ast.pscalar ~at (Ast.Bool true), String_set.empty)
  | Ppat_construct ({ txt = Lident "false"; _ }, None) ->
    Ok (Ast.pscalar ~at (Ast.Bool false), String_set.empty)
  | Ppat_construct ({ txt = Lident name; _ }, payload) when is_constructor_name name ->
    (match constructor_arity ctx name with
     | None -> note_free ctx at "constructor" name
     | Some _ -> ());
    (match payload with
     | None -> Ok (Ast.pconstruct ~at name [], String_set.empty)
     | Some ([], arg) ->
       (match arg.ppat_desc with
        | Ppat_tuple (fields, Asttypes.Closed)
          when List.length fields >= 2 && List.for_all (fun (l, _) -> l = None) fields ->
          (match lower_patterns ctx (List.map snd fields) with
           | Error e -> Error e
           | Ok (ps, names) -> Ok (Ast.pconstruct ~at name ps, names))
        | _ ->
          (match lower_pattern ctx arg with
           | Error e -> Error e
           | Ok (p, names) -> Ok (Ast.pconstruct ~at name [ p ], names)))
     | Some (_ :: _, _) ->
       unsupported at "existential constructor patterns are outside the grammar")
  | Ppat_construct _ ->
    unsupported at "qualified constructor names are outside the grammar"
  | _ -> unsupported at "pattern outside the grammar's pattern production"

and lower_patterns ctx ps =
  let rec go acc names = function
    | [] -> Ok (List.rev acc, names)
    | p :: rest ->
      (match lower_pattern ctx p with
       | Error e -> Error e
       | Ok (p, names') -> go (p :: acc) (String_set.union names names') rest)
  in
  go [] String_set.empty ps
;;

(* Expression lowering.  [scope] holds the names the program binds at
   one point; a program binding shadows the prelude.  A name that is
   neither program-bound nor prelude is recorded as a free occurrence:
   only the pinned type checker can later distinguish a standard-library
   hit (host-valid, subset-unsupported) from an unbound name
   (type-invalid), so those rejections run after typing succeeds. *)

let ( let* ) = Result.bind
let fresh_function_parameter = "%function-argument"

let constructor_eta_parameters n =
  List.init n (fun i -> "%constructor-argument-" ^ string_of_int (i + 1))
;;

type operator_form =
  | Form_arith of Ast.arith
  | Form_compare of Ast.comparison
  | Form_concat
  | Form_not
  | Form_neg
  | Form_deref
  | Form_assign
  | Form_make_ref
  | Form_and
  | Form_or

let operator_form name arity =
  match name, arity with
  | "+", 2 -> Some (Form_arith Ast.Add)
  | "-", 2 -> Some (Form_arith Ast.Sub)
  | "~-", 1 -> Some Form_neg
  | "*", 2 -> Some (Form_arith Ast.Mul)
  | "/", 2 -> Some (Form_arith Ast.Div)
  | "mod", 2 -> Some (Form_arith Ast.Rem)
  | "+.", 2 -> Some (Form_arith Ast.Addf)
  | "-.", 2 -> Some (Form_arith Ast.Subf)
  | "~-.", 1 -> Some Form_neg
  | "*.", 2 -> Some (Form_arith Ast.Mulf)
  | "/.", 2 -> Some (Form_arith Ast.Divf)
  | "=", 2 -> Some (Form_compare Ast.Eq)
  | "<>", 2 -> Some (Form_compare Ast.Ne)
  | "<", 2 -> Some (Form_compare Ast.Lt)
  | "<=", 2 -> Some (Form_compare Ast.Le)
  | ">", 2 -> Some (Form_compare Ast.Gt)
  | ">=", 2 -> Some (Form_compare Ast.Ge)
  | "^", 2 -> Some Form_concat
  | "not", 1 -> Some Form_not
  | "!", 1 -> Some Form_deref
  | ":=", 2 -> Some Form_assign
  | "ref", 1 -> Some Form_make_ref
  | "&&", 2 -> Some Form_and
  | "||", 2 -> Some Form_or
  | _ -> None
;;

let builtin_form at form args =
  match form, args with
  | Form_arith op, [ a; b ] -> Ok (Ast.arith ~at op a b)
  | Form_compare op, [ a; b ] -> Ok (Ast.compare_ ~at op a b)
  | Form_concat, [ a; b ] -> Ok (Ast.concat ~at a b)
  | Form_and, [ a; b ] -> Ok (Ast.and_ ~at a b)
  | Form_or, [ a; b ] -> Ok (Ast.or_ ~at a b)
  | Form_assign, [ a; b ] -> Ok (Ast.assign ~at a b)
  | Form_not, [ a ] -> Ok (Ast.not_ ~at a)
  | Form_neg, [ a ] -> Ok (Ast.neg ~at a)
  | Form_deref, [ a ] -> Ok (Ast.deref ~at a)
  | Form_make_ref, [ a ] -> Ok (Ast.make_ref ~at a)
  | _ -> unsupported at "an operator is applied with the wrong arity"
;;

let lower_binding_pattern (p : Parsetree.pattern) =
  match p.ppat_desc with
  | Ppat_var { txt = name; _ } when is_value_name name ->
    Ok (Some name, String_set.singleton name)
  | Ppat_var _ ->
    unsupported (span_of_location p.ppat_loc) "a binding name is not a value identifier"
  | Ppat_any -> Ok (None, String_set.empty)
  | Ppat_construct ({ txt = Lident "()"; _ }, None) -> Ok (None, String_set.empty)
  | _ ->
    unsupported
      (span_of_location p.ppat_loc)
      "let patterns beyond a name, _, or () are outside the grammar"
;;

let rec lower_expr ctx scope (e : Parsetree.expression) =
  let at = span_of_location e.pexp_loc in
  match e.pexp_desc with
  | Pexp_constant c ->
    let* s = lower_scalar c at in
    Ok (Ast.scalar ~at s)
  | Pexp_ident { txt = Lident name; _ } ->
    if String_set.mem name scope || String_set.mem name ctx.names
    then Ok (Ast.var ~at name)
    else if is_value_name name
    then (
      note_free ctx at "value" name;
      Ok (Ast.var ~at name))
    else unsupported at "an operator used as a value is outside the grammar"
  | Pexp_ident { txt = Ldot ({ txt = Lident modname; _ }, { txt = member; _ }); _ } ->
    let full = modname ^ "." ^ member in
    if List.mem full qualified_prelude
    then Ok (Ast.var ~at full)
    else
      unsupported
        at
        "a qualified name outside the fixed standard-library surface is outside the \
         grammar"
  | Pexp_ident _ -> unsupported at "arbitrary qualified paths are outside the grammar"
  | Pexp_let (rec_flag, vbs, body) ->
    let* bindings, names = lower_value_bindings ctx scope rec_flag vbs in
    let* body = lower_expr ctx (String_set.union scope names) body in
    let is_rec =
      match rec_flag with
      | Asttypes.Recursive -> true
      | Nonrecursive -> false
    in
    Ok (Ast.let_ ~at is_rec bindings body)
  | Pexp_function (params, constraint_, body) ->
    lower_function ctx scope at params constraint_ body
  | Pexp_apply (fn, args) -> lower_apply ctx scope at fn args
  | Pexp_match (scrutinee, cases) ->
    let* scrutinee = lower_expr ctx scope scrutinee in
    let* cases = lower_cases ctx scope cases in
    Ok (Ast.match_ ~at scrutinee cases)
  | Pexp_ifthenelse (condition, yes, Some no) ->
    let* condition = lower_expr ctx scope condition in
    let* yes = lower_expr ctx scope yes in
    let* no = lower_expr ctx scope no in
    Ok (Ast.if_ ~at condition yes no)
  | Pexp_ifthenelse (_, _, None) ->
    unsupported at "an if without else is outside the grammar's if production"
  | Pexp_sequence (first, second) ->
    let* first = lower_expr ctx scope first in
    let* second = lower_expr ctx scope second in
    Ok (Ast.sequence ~at first second)
  | Pexp_tuple fields ->
    if List.exists (fun (label, _) -> label <> None) fields
    then unsupported at "labelled tuple fields are outside the grammar"
    else
      let* parts = lower_exprs ctx scope (List.map snd fields) in
      Ok (Ast.tuple ~at parts)
  | Pexp_construct (name_loc, payload) -> lower_construct ctx scope at name_loc payload
  | Pexp_record (fields, None) -> lower_record ctx scope at fields
  | Pexp_record (_, Some _) -> unsupported at "record update syntax is excluded"
  | Pexp_field (record_expr, { txt = Lident name; _ }) ->
    let* record_expr = lower_expr ctx scope record_expr in
    Ok (Ast.field ~at record_expr name)
  | Pexp_field (_, _) -> unsupported at "qualified field names are outside the grammar"
  | Pexp_setfield _ -> unsupported at "mutable record fields and assignment are excluded"
  | _ -> unsupported at "an expression outside the grammar's expr production"

and lower_exprs ctx scope exprs =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | e :: rest ->
      let* e = lower_expr ctx scope e in
      go (e :: acc) rest
  in
  go [] exprs

and lower_cases ctx scope cases =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | (c : Parsetree.case) :: rest ->
      (match c.pc_guard with
       | Some g ->
         unsupported (span_of_location g.pexp_loc) "match guards are outside the grammar"
       | None ->
         let* pattern, binders = lower_pattern ctx c.pc_lhs in
         let* body = lower_expr ctx (String_set.union scope binders) c.pc_rhs in
         go ((pattern, body) :: acc) rest)
  in
  go [] cases

and lower_function ctx scope at params constraint_ body =
  match constraint_ with
  | Some _ -> unsupported at "type constraints on functions are outside the grammar"
  | None ->
    let* names = lower_params params in
    let inner = String_set.union scope (String_set.of_list names) in
    (match body with
     | Pfunction_body e ->
       let* e = lower_expr ctx inner e in
       Ok (Ast.fun_ ~at names e)
     | Pfunction_cases (cases, _, _) ->
       let arg = fresh_function_parameter in
       let* cases = lower_cases ctx inner cases in
       Ok (Ast.fun_ ~at (names @ [ arg ]) (Ast.match_ ~at (Ast.var ~at arg) cases)))

and lower_params params =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | (p : Parsetree.function_param) :: rest ->
      (match p.pparam_desc with
       | Pparam_val (Asttypes.Nolabel, None, pat) ->
         (match pat.ppat_desc with
          | Ppat_var { txt = name; _ } when is_value_name name -> go (name :: acc) rest
          | Ppat_var _ ->
            unsupported
              (span_of_location pat.ppat_loc)
              "a parameter name is not a value identifier"
          | Ppat_any -> go ("_" :: acc) rest
          | _ ->
            unsupported
              (span_of_location pat.ppat_loc)
              "destructuring parameters are outside the grammar; destructure with match")
       | Pparam_val (_, Some _, _) ->
         unsupported
           (span_of_location p.pparam_loc)
           "optional parameters are outside the grammar"
       | Pparam_val (_, None, _) ->
         unsupported
           (span_of_location p.pparam_loc)
           "labelled parameters are outside the grammar"
       | Pparam_newtype _ ->
         unsupported
           (span_of_location p.pparam_loc)
           "locally abstract type parameters are outside the grammar")
  in
  go [] params

and lower_apply ctx scope at fn args =
  if List.exists (fun (label, _) -> label <> Asttypes.Nolabel) args
  then unsupported at "labelled and optional arguments are outside the grammar"
  else (
    let fn_at = span_of_location fn.pexp_loc in
    let* lowered = lower_exprs ctx scope (List.map snd args) in
    match fn.pexp_desc with
    | Pexp_ident { txt = Lident name; _ } ->
      let call target = Ok (Ast.apply ~at (Ast.var ~at:fn_at target) lowered) in
      if String_set.mem name scope
      then call name
      else (
        match operator_form name (List.length lowered) with
        | Some form -> builtin_form at form lowered
        | None ->
          if not (is_value_name name)
          then
            unsupported
              at
              ("the operator " ^ name ^ " is outside the admitted operator forms")
          else (
            if not (String_set.mem name ctx.names) then note_free ctx at "value" name;
            call name))
    | Pexp_ident { txt = Ldot ({ txt = Lident modname; _ }, { txt = member; _ }); _ } ->
      let full = modname ^ "." ^ member in
      if List.mem full qualified_prelude
      then Ok (Ast.apply ~at (Ast.var ~at:fn_at full) lowered)
      else
        unsupported
          at
          "a qualified name outside the fixed standard-library surface is outside the \
           grammar"
    | Pexp_ident _ -> unsupported at "arbitrary qualified paths are outside the grammar"
    | _ ->
      let* fn = lower_expr ctx scope fn in
      Ok (Ast.apply ~at fn lowered))

and lower_construct ctx scope at (name_loc : Longident.t Location.loc) payload =
  match name_loc.txt with
  | Lident name ->
    if is_constructor_name name
    then lower_user_construct ctx scope at name payload
    else (
      match name, payload with
      | "[]", None -> Ok (Ast.nil ~at ())
      | "[]", Some _ -> syntax_at at "the empty-list constructor takes no payload"
      | "::", Some arg ->
        (match arg.pexp_desc with
         | Pexp_tuple [ (_, h); (_, t) ] ->
           let* h = lower_expr ctx scope h in
           let* t = lower_expr ctx scope t in
           Ok (Ast.cons ~at h t)
         | _ -> unsupported at "the cons form takes a pair payload")
      | "::", None ->
        unsupported at "the cons operator used as a value is outside the grammar"
      | "()", None -> Ok (Ast.scalar ~at Ast.Unit)
      | "true", None -> Ok (Ast.scalar ~at (Ast.Bool true))
      | "false", None -> Ok (Ast.scalar ~at (Ast.Bool false))
      | _ ->
        unsupported at "a constructor form outside the grammar's simple_expr production")
  | _ -> unsupported at "qualified constructor names are outside the grammar"

and lower_user_construct ctx scope at name payload =
  let note () =
    match constructor_arity ctx name with
    | None -> note_free ctx at "constructor" name
    | Some _ -> ()
  in
  match payload with
  | None ->
    (match constructor_arity ctx name with
     | Some n when n >= 1 ->
       let params = constructor_eta_parameters n in
       let body = Ast.construct ~at name (List.map (fun p -> Ast.var ~at p) params) in
       Ok (Ast.fun_ ~at params body)
     | _ ->
       note ();
       Ok (Ast.construct ~at name []))
  | Some arg ->
    note ();
    (match arg.pexp_desc with
     | Pexp_tuple fields
       when List.length fields >= 2
            && List.for_all (fun (label, _) -> label = None) fields ->
       let* parts = lower_exprs ctx scope (List.map snd fields) in
       Ok (Ast.construct ~at name parts)
     | _ ->
       let* part = lower_expr ctx scope arg in
       Ok (Ast.construct ~at name [ part ]))

and lower_record ctx scope at fields =
  let rec go seen acc = function
    | [] -> Ok (Ast.record ~at (List.rev acc))
    | ((label_loc : Longident.t Location.loc), value) :: rest ->
      (match label_loc.txt with
       | Lident name when not (List.mem name seen) ->
         let* value = lower_expr ctx scope value in
         go (name :: seen) ((name, value) :: acc) rest
       | Lident name ->
         contract
           at
           ("record field " ^ name ^ " is supplied twice in one record expression")
       | _ -> unsupported at "qualified field names are outside the grammar")
  in
  go [] [] fields

and lower_value_bindings ctx scope rec_flag vbs =
  let is_rec =
    match rec_flag with
    | Asttypes.Recursive -> true
    | Nonrecursive -> false
  in
  let rec gather acc = function
    | [] -> Ok (List.rev acc)
    | (vb : Parsetree.value_binding) :: rest ->
      (match vb.pvb_constraint with
       | Some _ ->
         unsupported
           (span_of_location vb.pvb_loc)
           "type constraints on bindings are outside the grammar"
       | None ->
         let* name, binders = lower_binding_pattern vb.pvb_pat in
         gather ((name, binders, vb) :: acc) rest)
  in
  let* gathered = gather [] vbs in
  let names =
    List.fold_left
      (fun set (_, binders, _) -> String_set.union set binders)
      String_set.empty
      gathered
  in
  let rec build acc = function
    | [] -> Ok (List.rev acc, names)
    | (name, _, (vb : Parsetree.value_binding)) :: rest ->
      let rhs_scope = if is_rec then String_set.union scope names else scope in
      let* rhs = lower_expr ctx rhs_scope vb.pvb_expr in
      build ({ Ast.name; rhs } :: acc) rest
  in
  build [] gathered
;;

(* Type declarations.  A group registers every name before any body is
   validated, so mutually recursive types resolve inside the group. *)

let lower_type_params (td : Parsetree.type_declaration) =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | (ct, (variance, injectivity)) :: rest ->
      (match ct.ptyp_desc, variance, injectivity with
       | Ptyp_var name, Asttypes.NoVariance, Asttypes.NoInjectivity when is_type_name name
         -> go (name :: acc) rest
       | _ ->
         unsupported
           (span_of_location ct.ptyp_loc)
           "type parameters are plain Hindley-Milner variables without annotations")
  in
  go [] td.ptype_params
;;

let lower_constructors ctx constructors =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | (cd : Parsetree.constructor_declaration) :: rest ->
      let at = span_of_location cd.pcd_loc in
      if not (is_constructor_name cd.pcd_name.txt)
      then unsupported at "a constructor name is not a constructor identifier"
      else if cd.pcd_vars <> []
      then unsupported at "existential constructor variables are outside the grammar"
      else (
        match cd.pcd_res with
        | Some _ -> unsupported at "GADT result types are outside the grammar"
        | None ->
          (match cd.pcd_args with
           | Pcstr_record _ -> unsupported at "inline records are outside the grammar"
           | Pcstr_tuple args ->
             let* () = all_unit (validate_type ctx) args in
             let name = cd.pcd_name.txt in
             let arity = List.length args in
             record_arity ctx at name arity;
             go ((name, arity) :: acc) rest))
  in
  go [] constructors
;;

let lower_labels ctx labels =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | (ld : Parsetree.label_declaration) :: rest ->
      (match ld.pld_mutable with
       | Asttypes.Mutable ->
         unsupported (span_of_location ld.pld_loc) "mutable record fields are excluded"
       | Asttypes.Immutable ->
         if not (is_value_name ld.pld_name.txt)
         then
           unsupported
             (span_of_location ld.pld_loc)
             "a field name is not a value identifier"
         else (
           match validate_type ctx ld.pld_type with
           | Error e -> Error e
           | Ok () -> go (ld.pld_name.txt :: acc) rest))
  in
  go [] labels
;;

let lower_type_declaration ctx (td : Parsetree.type_declaration) =
  let at = span_of_location td.ptype_loc in
  let tname = td.ptype_name.txt in
  if not (is_type_name tname)
  then unsupported at "a type name is not a type identifier"
  else if td.ptype_constraints <> []
  then unsupported at "type constraints on declarations are outside the grammar"
  else if td.ptype_private = Asttypes.Private
  then unsupported at "private types are outside the grammar"
  else
    let* type_params = lower_type_params td in
    match td.ptype_kind, td.ptype_manifest with
    | Ptype_abstract, Some manifest ->
      let* () = validate_type ctx manifest in
      Ok { Ast.type_name = tname; type_params; constructors = []; fields = [] }
    | Ptype_abstract, None ->
      unsupported at "a bare abstract type declaration is outside the grammar's type_body"
    | Ptype_variant constructors, None ->
      let* constructors = lower_constructors ctx constructors in
      Ok { Ast.type_name = tname; type_params; constructors; fields = [] }
    | Ptype_record labels, None ->
      let* fields = lower_labels ctx labels in
      Ok { Ast.type_name = tname; type_params; constructors = []; fields }
    | Ptype_variant _, Some _ | Ptype_record _, Some _ ->
      unsupported
        at
        "a manifest on a variant or record declaration is outside the grammar's type_body"
    | _ -> unsupported at "extensible and external types are outside the grammar"
;;

let lower_type_group ctx decls =
  let rec register = function
    | [] -> Ok ()
    | (td : Parsetree.type_declaration) :: rest ->
      let at = span_of_location td.ptype_loc in
      let tname = td.ptype_name.txt in
      if not (is_type_name tname)
      then unsupported at "a type name is not a type identifier"
      else (
        if List.mem tname built_in_types
        then
          note_contract
            ctx
            at
            ("type name " ^ tname ^ " collides with a built-in type name")
        else if String_set.mem tname !(ctx.types)
        then note_contract ctx at ("type name " ^ tname ^ " is declared twice in the unit")
        else ctx.types := String_set.add tname !(ctx.types);
        register rest)
  in
  let* () = register decls in
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | td :: rest ->
      let* shape = lower_type_declaration ctx td in
      go (shape :: acc) rest
  in
  go [] decls
;;

(* Structure items.  Floating doc comments become ocaml.text attribute
   items; they are comments in the subset and are skipped. *)

let is_comment_attribute (attr : Parsetree.attribute) =
  match attr.attr_name.txt with
  | "ocaml.text" | "ocaml.doc" -> true
  | _ -> false
;;

let lower_items ctx structure =
  let rec go scope acc first last = function
    | [] ->
      let at =
        match first, last with
        | Some (start : Ast.span), Some (stop : Ast.span) ->
          { Ast.start = start.Ast.start; stop = stop.Ast.stop }
        | _ -> dummy_span
      in
      Ok (List.rev acc, at)
    | (item : Parsetree.structure_item) :: rest ->
      let at = span_of_location item.pstr_loc in
      let first =
        match first with
        | None -> Some at
        | some -> some
      in
      let last = Some at in
      (match item.pstr_desc with
       | Pstr_value (rec_flag, vbs) ->
         let is_rec =
           match rec_flag with
           | Asttypes.Recursive -> true
           | Nonrecursive -> false
         in
         let* bindings, names = lower_value_bindings ctx scope rec_flag vbs in
         go
           (String_set.union scope names)
           (Ast.Value_item (is_rec, bindings) :: acc)
           first
           last
           rest
       | Pstr_type (_flag, decls) ->
         let* shapes = lower_type_group ctx decls in
         go
           scope
           (List.rev_append (List.map (fun shape -> Ast.Type_item shape) shapes) acc)
           first
           last
           rest
       | Pstr_attribute attr when is_comment_attribute attr ->
         go scope acc first last rest
       | Pstr_eval _ ->
         unsupported
           at
           "a top-level expression is outside the grammar's top_item production"
       | _ -> unsupported at "a declaration outside the grammar's top_item production")
  in
  go String_set.empty [] None None structure
;;

let check_attributes structure =
  let seen = ref None in
  let iterator =
    { Ast_iterator.default_iterator with
      attribute =
        (fun sub attr ->
          (match attr.attr_name.txt, !seen with
           | ("ocaml.text" | "ocaml.doc"), _ -> ()
           | name, None ->
             seen
             := Some
                  ( span_of_location attr.attr_loc
                  , "the attribute " ^ name ^ " is outside the grammar" )
           | _, Some _ -> ());
          Ast_iterator.default_iterator.attribute sub attr)
    }
  in
  iterator.structure iterator structure;
  match !seen with
  | None -> Ok ()
  | Some (at, detail) -> unsupported at detail
;;

(* Stage 3: the pinned type checker, run in process through
   compiler-libs.  Warning state and Location.input_name are process
   global and are saved and restored around the call.  Warning 8
   (Partial_match) is captured instead of promoted: a captured warning 8
   rejects the unit exactly like the oracle's -warn-error +8 while
   keeping the grammar section 8 category Contract. *)

let typecheck_unit structure =
  Compmisc.init_path ();
  let env = Compmisc.initial_env () in
  let captured = ref [] in
  let previous_reporter = !Location.warning_reporter in
  let previous_input_name = !Location.input_name in
  (Location.warning_reporter
   := fun loc w ->
        captured := (loc, w) :: !captured;
        None);
  let restore () =
    Location.warning_reporter := previous_reporter;
    Location.input_name := previous_input_name
  in
  match Typemod.type_structure env structure with
  | typed, _, _, _, _ ->
    restore ();
    Ok (typed, List.rev !captured)
  | exception exn ->
    restore ();
    (match Location.error_of_exn exn with
     | Some (`Ok report) ->
       type_error
         (span_of_location report.main.Location.loc)
         "the pinned type checker rejects the source"
     | Some `Already_displayed ->
       type_error dummy_span "the pinned type checker rejects the source"
     | None -> raise exn)
;;

let partial_match_span warnings =
  List.find_map
    (fun (loc, w) ->
       match w with
       | Warnings.Partial_match _ -> Some (span_of_location loc)
       | _ -> None)
    warnings
;;

(* Grammar section 119: = and <> hold only on unit, int, float, bool,
   and string; the ordered comparisons hold only on int, float, and
   string.  The walk sees the typed tree, so a polymorphic comparison
   that ocamlc admits is still rejected. *)

let comparison_violation typed =
  let open Typedtree in
  let violation = ref None in
  let note at detail =
    match !violation with
    | None -> violation := Some (at, detail)
    | Some _ -> ()
  in
  let scalar_operand env ty allowed =
    match Types.get_desc (Ctype.expand_head env ty) with
    | Types.Tconstr (path, _, _) -> List.mem (Path.name path) allowed
    | _ -> false
  in
  let iterator =
    { Tast_iterator.default_iterator with
      expr =
        (fun sub e ->
          (match e.exp_desc with
           | Texp_apply (fn, args) ->
             (match fn.exp_desc with
              | Texp_ident (_, id_loc, _) ->
                let allowed =
                  match id_loc.Location.txt with
                  | Lident "=" -> Some [ "unit"; "int"; "float"; "bool"; "string" ]
                  | Lident "<>" -> Some [ "unit"; "int"; "float"; "bool"; "string" ]
                  | Lident "<" -> Some [ "int"; "float"; "string" ]
                  | Lident "<=" -> Some [ "int"; "float"; "string" ]
                  | Lident ">" -> Some [ "int"; "float"; "string" ]
                  | Lident ">=" -> Some [ "int"; "float"; "string" ]
                  | _ -> None
                in
                (match allowed with
                 | None -> ()
                 | Some allowed ->
                   List.iter
                     (fun (_, arg) ->
                        match arg with
                        | Arg a ->
                          if not (scalar_operand a.exp_env a.exp_type allowed)
                          then
                            note
                              (span_of_location e.exp_loc)
                              "a comparison operand type is outside the admitted \
                               comparison types"
                        | Omitted () -> ())
                     args)
              | _ -> ())
           | _ -> ());
          Tast_iterator.default_iterator.expr sub e)
    }
  in
  iterator.structure iterator typed;
  !violation
;;

let report_frees ctx =
  match List.rev !(ctx.frees) with
  | [] -> Ok ()
  | (at, kind, name) :: _ ->
    unsupported
      at
      (kind
       ^ " "
       ^ name
       ^ " resolves only through the implicit standard-library open and is outside the \
          fixed surface")
;;

let report_contracts ctx partial comparison =
  match partial, comparison, List.rev !(ctx.contracts) with
  | Some at, _, _ -> contract at "a case set is non-exhaustive"
  | None, Some (at, detail), _ -> contract at detail
  | None, None, (at, detail) :: _ -> contract at detail
  | None, None, [] -> Ok ()
;;

type experiment =
  | Core
  | Lazy
  | Search

let experiment_names = function
  | Core -> String_set.empty
  | Lazy -> String_set.of_list [ "delay"; "force" ]
  | Search -> String_set.of_list [ "amb"; "require" ]
;;

let experiment_preamble = function
  | Core -> Ok []
  | Lazy -> parse "<lazy-experiment>" "let delay x = x\nlet force x = x\n"
  | Search -> parse "<search-experiment>" "let amb x _y = x\nlet require _b = ()\n"
;;

let admit ~experiment ~forms structure =
  let names =
    String_set.union
      (String_set.of_list unqualified_prelude)
      (String_set.union
         (experiment_names experiment)
         (String_set.of_list (List.map fst forms)))
  in
  let ctx =
    { ctors = ref []
    ; types = ref String_set.empty
    ; contracts = ref []
    ; frees = ref []
    ; names
    }
  in
  let* preamble = experiment_preamble experiment in
  let* stubs =
    parse
      "<experiment-forms>"
      (String.concat "" (List.map (fun (_, stub) -> stub ^ "\n") forms))
  in
  let preamble = preamble @ stubs in
  let* () = check_attributes structure in
  let* items, at = lower_items ctx structure in
  let* typed, warnings = typecheck_unit (preamble @ structure) in
  let* () = report_frees ctx in
  let* () =
    report_contracts ctx (partial_match_span warnings) (comparison_violation typed)
  in
  Ok { items; at }
;;

let check_experiment_with ~experiment ~forms ~filename source =
  let* () = scan_lexical source in
  let* structure = parse filename source in
  admit ~experiment ~forms structure
;;

let check_experiment ~experiment ~filename source =
  check_experiment_with ~experiment ~forms:[] ~filename source
;;

let check ~filename source = check_experiment ~experiment:Core ~filename source
