(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.20 *)

(** Exercise 3.20, replaced for this edition: the book asks for the
    environment diagram of the procedural pairs. This edition instead
    traces the aliasing those closures create: [z]'s two slots are made
    to designate the very same pair object [x], so one mutation through
    [(cdr z)] is visible through [x]. The witness is physical equality
    of what the slots hold. *)

open Sicp_ch3.Sec_3_3.Mpairs

(* The procedural pair of the section: closures over two cells. *)
type 'a proc_pair =
  { pcar : unit -> 'a
  ; pcdr : unit -> 'a
  ; set_pcar : 'a -> unit
  ; set_pcdr : 'a -> unit
  }

let proc_cons x y =
  let x = ref x in
  let y = ref y in
  { pcar = (fun () -> !x)
  ; pcdr = (fun () -> !y)
  ; set_pcar = (fun v -> x := v)
  ; set_pcdr = (fun v -> y := v)
  }
;;

let ex_3_20 () =
  let x = proc_cons (mint 1) (mint 2) in
  let z = proc_cons x x in
  (* (set-car! (cdr z) 17): the mutation reaches x through z's cdr. *)
  (z.pcdr ()).set_pcar (mint 17);
  let car_x = show (x.pcar ()) in
  (* both slots of z hold the very same pair object as x *)
  let slots_alias_x = z.pcar () == x && z.pcdr () == x in
  car_x, slots_alias_x
;;
