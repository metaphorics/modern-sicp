(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.33: literal lists become true lazy lists.  Under the
    section evaluator a lazy list is what the program's own [cons]
    builds, a compound call whose operands stay thunks, while a list
    written in literal notation evaluates its elements as it is
    built.  The literal is therefore not the structure the program's
    lazy-list procedures expect: an armed element that the walk never
    reaches still stops the program.  The fix lifts the list clause so
    that a literal cell delays its element and its tail exactly as the
    program's [cons] does, and the literal becomes the same lazy
    structure. *)

(** [ex_4_33 ()] answers, in order: the second element of the literal
    [[1 / 0; 42]] under the section evaluator; the second element of
    the same list built with the program's [cons]; the second element
    of the literal under the lifted list clause; and [list_ref] walking
    the literal [[1 / 0; 42; 1 / 0; 7]] to index 3 under the lifted
    clause. *)
val ex_4_33 : unit -> string list
