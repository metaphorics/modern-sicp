(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program zero in SICP section 2.1
   exercise 2.6 *)

(** [one] is [add_1 zero] substituted by hand: [fun f x -> f (zero f
    x)] reduces to [fun f x -> f x], applying [f] once. [two] is
    [add_1 one] the same way: [fun f x -> f (one f x)] reduces to
    [fun f x -> f (f x)]. [church_add a b] applies [f] as many times
    as [a] does, starting from where [b] left off, rather than calling
    [add_1] [a] times. *)

let zero _f x = x
let add_1 n f x = f (n f x)
let one f x = f x
let two f x = f (f x)
let church_add a b f x = a f (b f x)
let church_to_int n = n (fun x -> x + 1) 0
let ex_2_06 () = church_to_int (church_add one two)
