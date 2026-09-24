(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.13 *)

(** Exercise 3.13 (and the edition's addition 3.13a): [make_cycle]
    closes the last pair onto the first, and no naive walk or naive
    printer survives the result. [ex_3_13] shows a bounded walk never
    reaches the empty list, [last_pair_bounded] shows the book's
    [last-pair] gets no answer, and [show_cycle] prints a cyclic
    structure in a finite string keyed by physical identity. *)

open Sicp_ch3.Sec_3_3.Mpairs

let make_cycle x =
  set_cdr (last_pair x) x;
  x
;;

(* [last_pair] itself diverges on a cyclic structure; the bounded
   variant answers [None] when the walk has not ended after [n] cdrs. *)
let last_pair_bounded n x =
  let rec go fuel o =
    if fuel = 0 then None else if is_pair (cdr o) then go (fuel - 1) (cdr o) else Some o
  in
  if n <= 0 then None else go n x
;;

let ex_3_13 () =
  let z = make_cycle (from_symbols [ "a"; "b"; "c" ]) in
  (* a large multiple of the ring's length, and the walk stands where
     it began *)
  let rec walk fuel o = if fuel = 0 then o else walk (fuel - 1) (cdr o) in
  let returns_to_start = walk 300 z == z in
  let last_pair_gives_up =
    match last_pair_bounded 100 z with
    | None -> true
    | Some _ -> false
  in
  returns_to_start, last_pair_gives_up
;;

(* 3.13a: the cycle-safe printer. Visited pairs are remembered by
   physical identity in an auxiliary list, and a re-entered pair is
   shown as [#cycle] instead of being walked again. *)
let show_cycle ?(fuel = 10_000) o =
  let budget = ref fuel in
  let tick () =
    decr budget;
    if !budget < 0 then invalid_arg "show_cycle: fuel exhausted"
  in
  let visited = ref [] in
  let seen p = List.exists (fun c -> c == p) !visited in
  let rec go o =
    match o with
    | Int n -> string_of_int n
    | Sym s -> s
    | Nil -> "()"
    | Proc _ -> "#proc"
    | Pair p ->
      tick ();
      if seen p
      then "#cycle"
      else (
        visited := p :: !visited;
        let items, dotted = spine p.cdr [ p.car ] in
        let parts = List.map go items in
        let parts =
          match dotted with
          | None -> parts
          | Some tail -> parts @ [ "."; tail ]
        in
        "(" ^ String.concat " " parts ^ ")")
  and spine tail items =
    match tail with
    | Pair p ->
      if seen p
      then List.rev items, Some "#cycle"
      else (
        visited := p :: !visited;
        tick ();
        spine p.cdr (p.car :: items))
    | Nil -> List.rev items, None
    | other -> List.rev items, Some (go other)
  in
  go o
;;

let ex_3_13a () =
  let z3 = make_cycle (from_symbols [ "a"; "b"; "c" ]) in
  let z2 = make_cycle (from_symbols [ "a"; "b" ]) in
  let plain = from_symbols [ "a"; "b" ] in
  show_cycle z3, show_cycle z2, show_cycle plain
;;
