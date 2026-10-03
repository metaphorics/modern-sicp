(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.55: the three simple queries over the Microshaft data base
   -- everyone supervised by Ben Bitdiddle, the accounting division, and
   the Slumerville residents -- pinned in the data base's insertion
   order, plus the book's [same] observation [[supervisor, ?x, ?x]] as
   empty.

   The prescribed [[job, ?name, [accounting, ?title]]] matches only
   Cratchet: a proper-list pattern spans exactly two elements, so it
   misses the three-element [[accounting, chief, accountant]].  The
   dotted [[accounting | ?title]] retrieves the whole division, and the
   Slumerville query needs the same dotted tail because every
   Slumerville address carries a street and a number. *)

module Kit = struct
  module Q = Sicp_ch4.Sec_4_4
  module Streams = Q.Streams
  module Eval_error = Sicp_common.Eval_error

  let at name = Q.Atom name
  let v = Q.var
  let n k = Q.Num k
  let l = Q.list
  let atoms names = l (List.map at names)
  let person name = atoms (String.split_on_char ' ' name)
  let p items = Q.Pattern (l items)
  let job name title = l [ at "job"; person name; atoms title ]
  let salary name amount = l [ at "salary"; person name; n amount ]
  let supervisor name boss = l [ at "supervisor"; person name; person boss ]

  let address name town street number =
    l [ at "address"; person name; l ([ at town; atoms street ] @ List.map n number) ]
  ;;

  let can_do_job a b = l [ at "can-do-job"; atoms a; atoms b ]

  let microshaft =
    [ address "Bitdiddle Ben" "Slumerville" [ "Ridge"; "Road" ] [ 10 ]
    ; job "Bitdiddle Ben" [ "computer"; "wizard" ]
    ; salary "Bitdiddle Ben" 60000
    ; address "Hacker Alyssa P" "Cambridge" [ "Mass"; "Ave" ] [ 78 ]
    ; job "Hacker Alyssa P" [ "computer"; "programmer" ]
    ; salary "Hacker Alyssa P" 40000
    ; supervisor "Hacker Alyssa P" "Bitdiddle Ben"
    ; address "Fect Cy D" "Cambridge" [ "Ames"; "Street" ] [ 3 ]
    ; job "Fect Cy D" [ "computer"; "programmer" ]
    ; salary "Fect Cy D" 35000
    ; supervisor "Fect Cy D" "Bitdiddle Ben"
    ; address "Tweakit Lem E" "Boston" [ "Bay"; "State"; "Road" ] [ 22 ]
    ; job "Tweakit Lem E" [ "computer"; "technician" ]
    ; salary "Tweakit Lem E" 25000
    ; supervisor "Tweakit Lem E" "Bitdiddle Ben"
    ; address "Reasoner Louis" "Slumerville" [ "Pine"; "Tree"; "Road" ] [ 80 ]
    ; job "Reasoner Louis" [ "computer"; "programmer"; "trainee" ]
    ; salary "Reasoner Louis" 30000
    ; supervisor "Reasoner Louis" "Hacker Alyssa P"
    ; supervisor "Bitdiddle Ben" "Warbucks Oliver"
    ; address "Warbucks Oliver" "Swellesley" [ "Top"; "Heap"; "Road" ] []
    ; job "Warbucks Oliver" [ "administration"; "big"; "wheel" ]
    ; salary "Warbucks Oliver" 150000
    ; address "Scrooge Eben" "Weston" [ "Shady"; "Lane" ] [ 10 ]
    ; job "Scrooge Eben" [ "accounting"; "chief"; "accountant" ]
    ; salary "Scrooge Eben" 75000
    ; supervisor "Scrooge Eben" "Warbucks Oliver"
    ; address "Cratchet Robert" "Allston" [ "N"; "Harvard"; "Street" ] [ 16 ]
    ; job "Cratchet Robert" [ "accounting"; "scrivener" ]
    ; salary "Cratchet Robert" 18000
    ; supervisor "Cratchet Robert" "Scrooge Eben"
    ; address "Aull DeWitt" "Slumerville" [ "Onion"; "Square" ] [ 5 ]
    ; job "Aull DeWitt" [ "administration"; "secretary" ]
    ; salary "Aull DeWitt" 25000
    ; supervisor "Aull DeWitt" "Warbucks Oliver"
    ; can_do_job [ "computer"; "wizard" ] [ "computer"; "programmer" ]
    ; can_do_job [ "computer"; "wizard" ] [ "computer"; "technician" ]
    ; can_do_job [ "computer"; "programmer" ] [ "computer"; "programmer"; "trainee" ]
    ; can_do_job [ "administration"; "secretary" ] [ "administration"; "big"; "wheel" ]
    ]
  ;;

  let session ?(rules = []) assertions =
    let s = Q.new_session () in
    List.iter (Q.add_assertion s) assertions;
    List.iter (fun (conclusion, body) -> Q.add_rule s conclusion body) rules;
    s
  ;;

  let find_assertions s pattern frame =
    Q.stream_flatmap
      (fun datum ->
         match Q.pattern_match pattern datum frame with
         | Some extended -> Q.singleton_stream extended
         | None -> Streams.the_empty_stream)
      (Q.fetch_assertions s pattern)
  ;;

  let transcript s queries =
    let out = Buffer.create 256 in
    let result =
      Q.run ~emit:(Buffer.add_string out) s (List.map (fun q -> Q.Query q) queries)
    in
    let lines =
      List.filter
        (fun line -> not (String.equal line ""))
        (String.split_on_char '\n' (Buffer.contents out))
    in
    match result with
    | Ok () -> lines
    | Error e -> lines @ [ "error: " ^ Eval_error.to_string e ]
  ;;

  let rec take k s =
    if k <= 0
    then []
    else (
      match s with
      | Streams.Empty -> []
      | Streams.Cons (head, tail) ->
        if k = 1 then [ head ] else head :: take (k - 1) (Lazy.force tail))
  ;;

  (* Answers with the engine's display discipline: renamed rule variables
     are numbered by first occurrence within the answer, so the printed
     form does not depend on how many rule applications the search made
     before it.  Query variables ([id 0]) are left as they are. *)
  let canonical_query q =
    let seen = ref [] in
    let rec term = function
      | Q.Var v when v.Q.id <> 0 ->
        (match List.find_opt (fun (w, _) -> w = v) !seen with
         | Some (_, k) -> Q.Var { v with Q.id = k }
         | None ->
           let k = List.length !seen + 1 in
           seen := !seen @ [ v, k ];
           Q.Var { v with Q.id = k })
      | (Q.Var _ | Q.Atom _ | Q.Num _ | Q.Str _ | Q.Nil) as t -> t
      | Q.Pair (a, b) ->
        let a = term a in
        Q.Pair (a, term b)
    in
    let rec query = function
      | Q.Pattern t -> Q.Pattern (term t)
      | Q.And qs -> Q.And (List.map query qs)
      | Q.Or qs -> Q.Or (List.map query qs)
      | Q.Not q -> Q.Not (query q)
      | Q.Holds (name, ts) -> Q.Holds (name, List.map term ts)
      | Q.Always_true -> Q.Always_true
      | Q.Form (name, qs) -> Q.Form (name, List.map query qs)
    in
    query q
  ;;

  let render_prefix force s q =
    let produced = Dynarray.create () in
    let keep answer =
      Dynarray.add_last produced (Q.render_query (canonical_query answer))
    in
    match Q.answers s q with
    | Error e -> [ "error: " ^ Eval_error.to_string e ]
    | Ok stream ->
      (match force keep stream with
       | () -> Dynarray.to_list produced
       | exception Q.Query_error e ->
         Dynarray.to_list produced @ [ "error: " ^ Eval_error.to_string e ])
  ;;

  let answers_upto k s q =
    render_prefix (fun keep stream -> List.iter keep (take k stream)) s q
  ;;

  let answers_all s q = render_prefix (fun keep -> Streams.stream_for_each keep) s q
end

open Kit

let ex_4_55 () =
  transcript
    (session microshaft)
    [ p [ at "supervisor"; v "x"; person "Bitdiddle Ben" ]
    ; p [ at "job"; v "name"; l [ at "accounting"; v "title" ] ]
    ; p [ at "job"; v "name"; Q.dotted [ at "accounting" ] (v "title") ]
    ; p [ at "address"; v "name"; Q.dotted [ at "Slumerville" ] (v "where") ]
    ; p [ at "supervisor"; v "x"; v "x" ]
    ]
;;
