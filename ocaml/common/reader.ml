(* SPDX-License-Identifier: GPL-3.0-only *)

type position =
  { line : int
  ; column : int
  }

type t =
  | Unexpected_eof of position
  | Unexpected_char of char * position
  | Bad_number of string * position
  | Bad_literal of string * position
  | Bad_string of string * position
  | Bad_form of string * position

let pp ppf = function
  | Unexpected_eof p ->
    Format.fprintf ppf "%d:%d: unexpected end of input" p.line p.column
  | Unexpected_char (c, p) ->
    Format.fprintf ppf "%d:%d: unexpected character %C" p.line p.column c
  | Bad_number (token, p) ->
    Format.fprintf ppf "%d:%d: malformed number %s" p.line p.column token
  | Bad_literal (token, p) ->
    Format.fprintf ppf "%d:%d: malformed hash literal %s" p.line p.column token
  | Bad_string (detail, p) ->
    Format.fprintf ppf "%d:%d: malformed string: %s" p.line p.column detail
  | Bad_form (detail, p) ->
    Format.fprintf ppf "%d:%d: malformed form: %s" p.line p.column detail
;;

let to_string e = Format.asprintf "%a" pp e
let ( >>= ) = Result.bind

let all_results results =
  List.fold_right
    (fun result acc -> Result.bind result (fun x -> Result.map (fun xs -> x :: xs) acc))
    results
    (Ok [])
;;

type state =
  { src : string
  ; mutable i : int
  ; mutable pos : position
  }

let peek s = if s.i < String.length s.src then Some s.src.[s.i] else None
let peek_at s k = if s.i + k < String.length s.src then Some s.src.[s.i + k] else None

let bump s =
  if s.src.[s.i] = '\n'
  then s.pos <- { line = s.pos.line + 1; column = 0 }
  else s.pos <- { s.pos with column = s.pos.column + 1 };
  s.i <- s.i + 1
;;

let here s = s.pos

let is_space = function
  | ' ' | '\t' | '\r' | '\n' -> true
  | _ -> false
;;

let is_delimiter = function
  | ' ' | '\t' | '\r' | '\n' | '(' | ')' | '"' | ';' | '\'' -> true
  | _ -> false
;;

let at_delimiter s k =
  match peek_at s k with
  | None -> true
  | Some c -> is_delimiter c
;;

let is_symbol_constituent c =
  (c >= 'a' && c <= 'z')
  || (c >= 'A' && c <= 'Z')
  || (c >= '0' && c <= '9')
  ||
  match c with
  | '-' | '?' | '!' | '*' | '+' | '/' | '<' | '>' | '=' | '_' | '.' -> true
  | _ -> false
;;

let skip_atmosphere s =
  let rec line_comment () =
    match peek s with
    | Some '\n' -> bump s
    | Some _ ->
      bump s;
      line_comment ()
    | None -> ()
  in
  let rec go () =
    match peek s with
    | Some c when is_space c ->
      bump s;
      go ()
    | Some ';' ->
      bump s;
      line_comment ();
      go ()
    | _ -> ()
  in
  go ()
;;

let read_token s =
  let buf = Buffer.create 16 in
  let rec go () =
    match peek s with
    | Some c when not (is_delimiter c) ->
      Buffer.add_char buf c;
      bump s;
      go ()
    | _ -> ()
  in
  go ();
  Buffer.contents buf
;;

let is_digits body = body <> "" && String.for_all (fun c -> c >= '0' && c <= '9') body

let split_sign s =
  if String.length s = 0
  then "", s
  else (
    match s.[0] with
    | ('+' | '-') as sign -> String.make 1 sign, String.sub s 1 (String.length s - 1)
    | _ -> "", s)
;;

let split_exponent body =
  match String.index_opt body 'e', String.index_opt body 'E' with
  | Some k, _ | None, Some k ->
    Some (String.sub body 0 k, String.sub body (k + 1) (String.length body - k - 1))
  | None, None -> None
;;

let exponent_ok e =
  let sign, digits = split_sign e in
  is_digits digits && (sign = "" || sign = "+" || sign = "-")
;;

let classify_number token pos =
  let bad () = Error (Bad_number (token, pos)) in
  let _, body = split_sign token in
  let mantissa, exponent =
    match split_exponent body with
    | Some (m, e) -> m, Some e
    | None -> body, None
  in
  match String.index_opt mantissa '.' with
  | None ->
    if is_digits mantissa && exponent = None
    then (
      match int_of_string_opt token with
      | Some n -> Ok (Ast.DInt n)
      | None -> bad ())
    else bad ()
  | Some k ->
    let pre = String.sub mantissa 0 k in
    let post = String.sub mantissa (k + 1) (String.length mantissa - k - 1) in
    let exponent_ok =
      match exponent with
      | None -> true
      | Some e -> exponent_ok e
    in
    if is_digits pre && is_digits post && exponent_ok
    then (
      match float_of_string_opt token with
      | Some f -> Ok (Ast.DFloat f)
      | None -> bad ())
    else bad ()
;;

let is_number_like token =
  match token.[0] with
  | '0' .. '9' -> true
  | '+' | '-' ->
    String.length token > 1
    &&
      (match token.[1] with
      | '0' .. '9' -> true
      | _ -> false)
  | _ -> false
;;

let first_non_symbol_char token =
  let n = String.length token in
  let rec go k =
    if k = n
    then None
    else if is_symbol_constituent token.[k]
    then go (k + 1)
    else Some token.[k]
  in
  go 0
;;

let classify_atom token pos =
  if is_number_like token
  then classify_number token pos
  else (
    match first_non_symbol_char token with
    | None -> Ok (Ast.DSymbol token)
    | Some c -> Error (Unexpected_char (c, pos)))
;;

let quoted datum = Ast.DPair (Ast.DSymbol "quote", Ast.DPair (datum, Ast.DNil))

let read_string s =
  bump s;
  let buf = Buffer.create 16 in
  let rec go () =
    match peek s with
    | None -> Error (Bad_string ("unterminated string", here s))
    | Some '"' ->
      bump s;
      Ok (Ast.DString (Buffer.contents buf))
    | Some '\\' ->
      bump s;
      (match peek s with
       | Some '"' ->
         Buffer.add_char buf '"';
         bump s;
         go ()
       | Some '\\' ->
         Buffer.add_char buf '\\';
         bump s;
         go ()
       | Some c -> Error (Bad_string (Printf.sprintf "unknown escape \\%c" c, here s))
       | None -> Error (Bad_string ("unterminated string", here s)))
    | Some c ->
      Buffer.add_char buf c;
      bump s;
      go ()
  in
  go ()
;;

let read_hash s start =
  bump s;
  match peek s with
  | Some ('t' | 'f') when at_delimiter s 1 ->
    let value =
      match peek s with
      | Some 't' -> true
      | _ -> false
    in
    bump s;
    Ok (Ast.DBool value)
  | _ ->
    let token = "#" ^ read_token s in
    Error (Bad_literal (token, start))
;;

let rec read_datum s =
  skip_atmosphere s;
  let start = here s in
  match peek s with
  | None -> Error (Unexpected_eof start)
  | Some '(' ->
    bump s;
    read_list s start
  | Some ')' -> Error (Unexpected_char (')', start))
  | Some '"' -> read_string s
  | Some '\'' ->
    bump s;
    (match read_datum s with
     | Ok d -> Ok (quoted d)
     | Error e -> Error e)
  | Some '#' -> read_hash s start
  | Some _ ->
    let token = read_token s in
    if token = "."
    then Error (Bad_form ("`.` outside a list", start))
    else classify_atom token start

and read_list s start =
  let build elements tail =
    List.fold_left (fun tail d -> Ast.DPair (d, tail)) tail elements
  in
  let dotted elements =
    skip_atmosphere s;
    match peek s with
    | None -> Error (Unexpected_eof (here s))
    | Some ')' -> Error (Bad_form ("`.` with an empty tail", here s))
    | Some _ ->
      (match read_datum s with
       | Error e -> Error e
       | Ok tail ->
         skip_atmosphere s;
         (match peek s with
          | Some ')' ->
            bump s;
            Ok (build elements tail)
          | Some c -> Error (Unexpected_char (c, here s))
          | None -> Error (Unexpected_eof (here s))))
  in
  let rec loop elements =
    skip_atmosphere s;
    match peek s with
    | None -> Error (Unexpected_eof (here s))
    | Some ')' ->
      bump s;
      Ok (build elements Ast.DNil)
    | Some '.' when at_delimiter s 1 ->
      bump s;
      (match elements with
       | [] -> Error (Bad_form ("`.` with no preceding element", start))
       | _ -> dotted elements)
    | Some _ ->
      (match read_datum s with
       | Ok d -> loop (d :: elements)
       | Error e -> Error e)
  in
  loop []
;;

let datum_proper_list datum =
  match datum with
  | Ast.DNil -> Some []
  | Ast.DPair _ ->
    let rec go acc = function
      | Ast.DNil -> Some (List.rev acc)
      | Ast.DPair (d, more) -> go (d :: acc) more
      | _ -> None
    in
    go [] datum
  | _ -> None
;;

let datum_symbols d =
  match datum_proper_list d with
  | None -> None
  | Some items ->
    let symbols =
      List.filter_map
        (function
          | Ast.DSymbol x -> Some x
          | _ -> None)
        items
    in
    if List.length symbols = List.length items then Some symbols else None
;;

let map_ast_error name pos = function
  | Eval_error.Invalid_form detail -> Bad_form (name ^ ": " ^ detail, pos)
  | e -> Bad_form (name ^ ": " ^ Eval_error.to_string e, pos)
;;

let rec expr_of_datum s pos datum =
  let bad detail = Error (Bad_form (detail, pos)) in
  match datum with
  | Ast.DInt n -> Ok (Ast.int n)
  | Ast.DFloat f -> Ok (Ast.float f)
  | Ast.DBool b -> Ok (Ast.bool b)
  | Ast.DString x -> Ok (Ast.string x)
  | Ast.DSymbol x -> Ok (Ast.variable x)
  | Ast.DNil -> bad "empty application"
  | Ast.DPair (head, args) ->
    (match datum_proper_list args with
     | None -> bad "call form with a dotted argument list"
     | Some operands ->
       (match head with
        | Ast.DSymbol name -> special_form s pos name head operands
        | _ -> application s pos head operands))

and application s pos head operands =
  let open Result in
  expr_of_datum s pos head
  >>= fun operator ->
  all_results (List.map (expr_of_datum s pos) operands)
  >>= fun operands -> Ok (Ast.application operator operands)

and special_form s pos name head operands =
  let open Result in
  let bad detail = Error (Bad_form (name ^ ": " ^ detail, pos)) in
  let operands_exprs () = all_results (List.map (expr_of_datum s pos) operands) in
  match name, operands with
  | "quote", [ d ] -> Ok (Ast.quote d)
  | "quote", _ -> bad "expects one datum"
  | "define", _ -> define_form s pos operands
  | "set!", [ Ast.DSymbol n; value ] ->
    expr_of_datum s pos value |> Result.map (Ast.set n)
  | "set!", _ -> bad "expects (set! name expression)"
  | "if", [ c; t ] ->
    expr_of_datum s pos c
    >>= fun c -> expr_of_datum s pos t |> Result.map (fun t -> Ast.if_ c t None)
  | "if", [ c; t; a ] ->
    expr_of_datum s pos c
    >>= fun c ->
    expr_of_datum s pos t
    >>= fun t -> expr_of_datum s pos a >>= fun a -> Ok (Ast.if_ c t (Some a))
  | "if", _ -> bad "expects two or three operands"
  | "cond", _ -> cond_form s pos operands
  | "and", _ -> operands_exprs () |> Result.map Ast.and_
  | "or", _ -> operands_exprs () |> Result.map Ast.or_
  | "begin", [] -> bad "needs a nonempty body"
  | "begin", _ ->
    operands_exprs ()
    >>= fun body -> Ast.sequence body |> Result.map_error (map_ast_error name pos)
  | "let", _ -> let_form s pos operands
  | "lambda", _ -> lambda_form s pos operands
  | _ ->
    operands_exprs ()
    >>= fun operands ->
    expr_of_datum s pos head >>= fun operator -> Ok (Ast.application operator operands)

and define_form s pos operands =
  let open Result in
  let bad detail = Error (Bad_form ("define: " ^ detail, pos)) in
  match operands with
  | [ Ast.DSymbol n; value ] ->
    expr_of_datum s pos value
    |> Result.map (fun value -> Ast.definition (Ast.define_variable n value))
  | Ast.DPair (Ast.DSymbol n, params) :: body when body <> [] ->
    (match datum_symbols params with
     | None -> bad "expects a parameter list of symbols"
     | Some parameters ->
       all_results (List.map (expr_of_datum s pos) body)
       >>= fun body ->
       Ast.define_function n parameters body
       |> Result.map (fun d -> Ast.definition d)
       |> Result.map_error (map_ast_error "define" pos))
  | _ -> bad "expects (define name expression) or (define (name ...) body ...)"

and cond_form s pos operands =
  let open Result in
  let bad detail = Error (Bad_form ("cond: " ^ detail, pos)) in
  let body_list d =
    match datum_proper_list d with
    | None -> Error (Bad_form ("cond: clause body is a dotted list", pos))
    | Some items -> all_results (List.map (expr_of_datum s pos) items)
  in
  let rec clauses acc = function
    | [] -> if acc = [] then bad "needs clauses" else Ok (List.rev acc, None)
    | [ Ast.DPair (Ast.DSymbol "else", body) ] ->
      (match body_list body with
       | Ok [] -> bad "else clause needs a body"
       | Ok body -> Ok (List.rev acc, Some body)
       | Error e -> Error e)
    | Ast.DPair (test, body) :: more ->
      (match expr_of_datum s pos test with
       | Error e -> Error e
       | Ok test ->
         (match body_list body with
          | Error e -> Error e
          | Ok body -> clauses ((test, body) :: acc) more))
    | _ :: _ -> bad "clause is not a list"
  in
  clauses [] operands
  >>= fun (clauses, else_body) ->
  Ast.cond clauses else_body |> Result.map_error (map_ast_error "cond" pos)

and let_form s pos operands =
  let open Result in
  let bad detail = Error (Bad_form ("let: " ^ detail, pos)) in
  let binding = function
    | Ast.DPair (Ast.DSymbol n, Ast.DPair (v, Ast.DNil)) ->
      expr_of_datum s pos v |> Result.map (fun v -> n, v)
    | _ -> Error (Bad_form ("let: binding is not (name value)", pos))
  in
  match operands with
  | bindings_d :: body when body <> [] ->
    (match datum_proper_list bindings_d with
     | None -> bad "bindings are not a list"
     | Some items ->
       all_results (List.map binding items)
       >>= fun bindings ->
       all_results (List.map (expr_of_datum s pos) body)
       >>= fun body ->
       Ast.let_ bindings body |> Result.map_error (map_ast_error "let" pos))
  | _ -> bad "expects (let ((name value) ...) body ...)"

and lambda_form s pos operands =
  let open Result in
  let bad detail = Error (Bad_form ("lambda: " ^ detail, pos)) in
  match operands with
  | params :: body when body <> [] ->
    (match datum_symbols params with
     | None -> bad "expects a parameter list of symbols"
     | Some parameters ->
       all_results (List.map (expr_of_datum s pos) body)
       >>= fun body ->
       Ast.lambda parameters body |> Result.map_error (map_ast_error "lambda" pos))
  | _ -> bad "expects (lambda (x ...) body ...)"
;;

let parse_form s =
  skip_atmosphere s;
  let pos = here s in
  match peek s with
  | None -> Error (Unexpected_eof pos)
  | Some _ ->
    (match read_datum s with
     | Error e -> Error e
     | Ok datum -> expr_of_datum s pos datum)
;;

let read text = parse_form { src = text; i = 0; pos = { line = 1; column = 0 } }

let read_program text =
  let s = { src = text; i = 0; pos = { line = 1; column = 0 } } in
  let rec go acc =
    skip_atmosphere s;
    match peek s with
    | None -> Ok (List.rev acc)
    | Some _ ->
      (match parse_form s with
       | Ok e -> go (e :: acc)
       | Error e -> Error e)
  in
  go []
;;
