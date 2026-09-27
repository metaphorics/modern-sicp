(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Expressions, values, and types: the named definitions behind the
    listings of section 0.2. *)

let average_of_two_ints a b = Float.of_int (a + b) /. 2.0
let warm_enough celsius = celsius >= 18.0
let greet name = "hello, " ^ name ^ "!"
