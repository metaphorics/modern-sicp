(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs of SICP section 2.2 *)

(** The named definitions behind the listings of section 2.2, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_2_2] through [Replay], so the book's result comments are
    true by construction.

    The section's hard spot is hierarchy: Scheme builds trees out of
    pairs that hold anything, including other pairs. OCaml's [list] is
    homogeneous, so tree programs here run over a [variant] type with
    one constructor for leaves and one for the branching case, and the
    picture language's painters become plain functions from frames to
    the segments they draw. The book states the compound painters
    before the frames and vectors they rest on; a Scheme file tolerates
    that forward reference because nothing runs until every [define]
    has been read, while OCaml resolves each name where it is written.
    This module therefore orders the definitions OCaml's way, and the
    book walks the layers in the same order, primitives first. The
    pattern-matched sum types ([Trees.tree], [Picture.segment],
    [Picture.painter]) stay transparent because matching on them is the
    section's subject. *)

(** Sequences over OCaml's built-in [list], subsection 2.2.1: the
    recursive [list_ref], [length], and [append] the book writes by
    hand, the iterative [length_iter], and [scale_list] before and
    after the abstraction to [map]. These are the book's procedures
    carried over verbatim, for the small lists the section feeds them;
    [List.nth], [List.length], [List.append @], and [List.map] are
    their library equivalents. *)
module List_ops : sig
  (** The sequence of @ref{2.2.1}'s opening figure, [\[1; 2; 3; 4\]]. *)
  val one_through_four : int list

  val squares : int list
  val odds : int list

  (** [list_ref items n] is the [n]-th item of [items], counting from
      zero. The index must be inside [items]. *)
  val list_ref : 'a list -> int -> 'a

  (** [length items] is the number of items in [items], the book's
      recursive plan. *)
  val length : 'a list -> int

  (** [length_iter items] is [length items] computed by an iterative
      process carried by a counter. *)
  val length_iter : 'a list -> int

  (** [append list1 list2] is the list of [list1]'s items followed by
      [list2]'s. *)
  val append : 'a list -> 'a list -> 'a list

  (** [scale_list items factor] multiplies every item of [items] by
      [factor], the recursive definition. *)
  val scale_list : int list -> int -> int list

  (** [map f items] applies [f] to every item of [items], the book's
      higher-order definition. *)
  val map : ('a -> 'b) -> 'a list -> 'b list

  (** [scale_list_map items factor] is [scale_list items factor]
      defined in terms of [map]. *)
  val scale_list_map : int list -> int -> int list
end

(** Trees, subsection 2.2.2. Scheme's [cons] trees can hold anything at
    a branching point; the [tree] variant makes that shape explicit:
    [Leaf] carries a number, [Node] carries the list of subtrees. *)
module Trees : sig
  type tree =
    | Leaf of int
    | Node of tree list

  (** The top level of [x] as a plain list, the view [length] takes:
      [\[Node [Leaf 1; Leaf 2]; Leaf 3; Leaf 4\]]. *)
  val x_list : tree list

  (** The section's running example, [((1 2) 3 4)] read as a tree:
      [Node [Node [Leaf 1; Leaf 2]; Leaf 3; Leaf 4]]. *)
  val x : tree

  (** The book's [(list x x)], a tree list holding [x] twice. *)
  val xx : tree list

  (** [count_leaves tree] is the number of [Leaf] values anywhere in
      [tree]. *)
  val count_leaves : tree -> int

  (** [scale_tree tree factor] multiplies every leaf of [tree] by
      [factor], the recursive definition. *)
  val scale_tree : tree -> int -> tree

  (** [scale_tree_map tree factor] is [scale_tree tree factor] defined
      by mapping over the subtrees. *)
  val scale_tree_map : tree -> int -> tree
end

(** Sequence operations as conventional interfaces, subsection 2.2.3:
    [map], [filter], and [accumulate] as the book defines them, the two
    enumerators, the signal-flow pipelines built from them, and the
    nested-mapping programs [prime_sum_pairs] and [permutations].
    [accumulate] is OCaml's [List.fold_right] in the book's argument
    order. *)
module Seq_ops : sig
  val map : ('a -> 'b) -> 'a list -> 'b list

  (** [filter predicate sequence] keeps the items of [sequence] that
      satisfy [predicate], in order. *)
  val filter : ('a -> bool) -> 'a list -> 'a list

  (** [accumulate op initial sequence] folds [op] over [sequence]
      right-to-left, starting from [initial]. *)
  val accumulate : ('a -> 'acc -> 'acc) -> 'acc -> 'a list -> 'acc

  (** [enumerate_interval low high] is [\[low; low+1; …; high\]], empty
      when [low > high]. *)
  val enumerate_interval : int -> int -> int list

  (** [enumerate_tree tree] is the list of the leaves of [tree], in
      left-to-right order. *)
  val enumerate_tree : Trees.tree -> int list

  val fib : int -> int
  val square : int -> int
  val odd : int -> bool
  val even : int -> bool

  (** [sum_odd_squares_tree tree] adds the squares of the odd leaves of
      [tree], the tree-recursive first definition. *)
  val sum_odd_squares_tree : Trees.tree -> int

  (** [sum_odd_squares tree] is [sum_odd_squares_tree tree] as a
      signal-flow pipeline. *)
  val sum_odd_squares : Trees.tree -> int

  (** [even_fibs_slow n] is the book's first definition of even-fibs,
      before the signal-flow abstraction. *)
  val even_fibs_slow : int -> int list

  (** [even_fibs n] is the list of the even Fibonacci numbers
      [Fib 0] through [Fib n]. *)
  val even_fibs : int -> int list

  (** [list_fib_squares n] is the list of squares of [Fib 0] through
      [Fib n]. *)
  val list_fib_squares : int -> int list

  (** [product_of_squares_of_odd_elements sequence] multiplies the
      squares of the odd items of [sequence]. *)
  val product_of_squares_of_odd_elements : int list -> int

  type personnel_record =
    { is_programmer : bool
    ; salary : int
    }

  (** [salary_of_highest_paid_programmer records] is the largest salary
      among the programmer records, or 0 when there is none. *)
  val salary_of_highest_paid_programmer : personnel_record list -> int

  (** [flatmap proc seq] appends the results of [proc] over [seq]. *)
  val flatmap : ('a -> 'b list) -> 'a list -> 'b list

  (** [remove item sequence] drops every occurrence of [item] from
      [sequence]. *)
  val remove : 'a -> 'a list -> 'a list

  val prime_sum : int * int -> bool
  val make_pair_sum : int * int -> int * int * int

  (** [prime_sum_pairs n] is the list of triples [i, j, i+j] with
      [1 ≤ j < i ≤ n] and [i + j] prime. *)
  val prime_sum_pairs : int -> (int * int * int) list

  (** [permutations s] is the list of all orderings of [s]'s items. *)
  val permutations : 'a list -> 'a list list
end

(** The picture language, subsection 2.2.4. A painter is a function
    from a frame to the segments it draws there, so painting has no
    side effect; a separate renderer turns a painter's segments into an
    SVG document. [up_split], [rotate_180], and [flip_horiz] are
    defined here because [corner_split] and the second [square_limit]
    need them; the book leaves [up_split] and the rotations to
    Exercises 2.44 and 2.50. *)
module Picture : sig
  module Vect : sig
    type t =
      { x : float
      ; y : float
      }

    val make_vect : float -> float -> t
    val xcor_vect : t -> float
    val ycor_vect : t -> float
    val add_vect : t -> t -> t
    val sub_vect : t -> t -> t
    val scale_vect : float -> t -> t
  end

  module Frame : sig
    type t =
      { origin : Vect.t
      ; edge1 : Vect.t
      ; edge2 : Vect.t
      }

    val make_frame : Vect.t -> Vect.t -> Vect.t -> t
    val origin_frame : t -> Vect.t
    val edge1_frame : t -> Vect.t
    val edge2_frame : t -> Vect.t
  end

  type segment = Vect.t * Vect.t

  (** A painter draws no pixels itself: applied to a frame it returns
      the segments it would draw there, in frame coordinates. *)
  type painter = Frame.t -> segment list

  (** [frame_coord_map frame] maps the unit square onto [frame]. *)
  val frame_coord_map : Frame.t -> Vect.t -> Vect.t

  (** [segments_to_painter segments] is the painter that draws
      [segments], shifted and scaled by the frame coordinate map. *)
  val segments_to_painter : segment list -> painter

  (** [transform_painter painter origin corner1 corner2] confines
      [painter] to the frame those three unit-square points describe. *)
  val transform_painter : painter -> Vect.t -> Vect.t -> Vect.t -> painter

  val flip_vert : painter -> painter
  val flip_horiz : painter -> painter
  val shrink_to_upper_right : painter -> painter
  val rotate_90 : painter -> painter
  val rotate_180 : painter -> painter
  val squash_inwards : painter -> painter

  (** [beside left right] paints [left] in the left half of the frame
      and [right] in the right half. *)
  val beside : painter -> painter -> painter

  (** [below bottom top] paints [bottom] in the lower half of the frame
      and [top] in the upper half. *)
  val below : painter -> painter -> painter

  val wave_segments : segment list
  val rogers_segments : segment list
  val wave : painter
  val rogers : painter
  val wave2 : painter
  val wave4 : painter
  val flipped_pairs : painter -> painter
  val up_split : painter -> int -> painter
  val right_split : painter -> int -> painter
  val corner_split : painter -> int -> painter
  val square_limit : painter -> int -> painter

  val square_of_four
    :  (painter -> painter)
    -> (painter -> painter)
    -> (painter -> painter)
    -> (painter -> painter)
    -> painter
    -> painter

  val flipped_pairs_of_four : painter -> painter
  val square_limit_of_four : painter -> int -> painter

  (** The one drawing operation this edition needs: turn the segments a
      painter produced into SVG text. *)
  module Svg : sig
    val line : segment -> string
    val document : view_box:string -> size:int -> string -> string
    val view_box : segment list -> string
  end

  (** [render_to_string frame painter] is the SVG document for the
      segments [painter] draws in [frame], sized 200 by 200 pixels with
      a five percent margin. *)
  val render_to_string : Frame.t -> painter -> string

  (** [write_svg ~path ~size frame painter] writes that document to
      [path], creating [path]'s directory when it is missing. *)
  val write_svg : path:string -> size:int -> Frame.t -> painter -> unit
end
