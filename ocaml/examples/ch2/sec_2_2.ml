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
    book walks the layers in the same order, primitives first. *)

(** Sequences over OCaml's built-in [list], subsection 2.2.1: the
    recursive [list_ref], [length], and [append] the book writes by
    hand, the iterative [length_iter], and [scale_list] before and
    after the abstraction to [map]. *)
module List_ops = struct
  let one_through_four = [ 1; 2; 3; 4 ]

  let rec list_ref items n =
    match items, n with
    | first :: _, 0 -> first
    | _ :: rest, _ -> list_ref rest (n - 1)
    | [], _ -> invalid_arg "list_ref: index past the end"
  ;;

  let squares = [ 1; 4; 9; 16; 25 ]

  let rec length items =
    match items with
    | [] -> 0
    | _ :: cdr -> 1 + length cdr
  ;;

  let odds = [ 1; 3; 5; 7 ]

  let length_iter items =
    let rec iter a count =
      match a with
      | [] -> count
      | _ :: cdr -> iter cdr (1 + count)
    in
    iter items 0
  ;;

  let rec append list1 list2 =
    match list1 with
    | [] -> list2
    | car :: cdr -> car :: append cdr list2
  ;;

  let rec scale_list items factor =
    match items with
    | [] -> []
    | car :: cdr -> (car * factor) :: scale_list cdr factor
  ;;

  let rec map f items =
    match items with
    | [] -> []
    | car :: cdr -> f car :: map f cdr
  ;;

  let scale_list_map items factor = map (fun x -> x * factor) items
end

(** Trees, subsection 2.2.2. Scheme's [cons] trees can hold anything at
    a branching point; the [tree] variant makes that shape explicit:
    [Leaf] carries a number, [Node] carries the list of subtrees. *)
module Trees = struct
  type tree =
    | Leaf of int
    | Node of tree list

  let x_list = [ Node [ Leaf 1; Leaf 2 ]; Leaf 3; Leaf 4 ]
  let x = Node x_list

  let rec count_leaves tree =
    match tree with
    | Leaf _ -> 1
    | Node children ->
      List.fold_left (fun acc sub_tree -> acc + count_leaves sub_tree) 0 children
  ;;

  let xx = [ x; x ]

  let rec scale_tree tree factor =
    match tree with
    | Leaf n -> Leaf (n * factor)
    | Node children ->
      Node (List.map (fun sub_tree -> scale_tree sub_tree factor) children)
  ;;

  let rec scale_tree_map tree factor =
    match tree with
    | Leaf n -> Leaf (n * factor)
    | Node children ->
      Node
        (List_ops.map
           (fun sub_tree ->
              match sub_tree with
              | Leaf n -> Leaf (n * factor)
              | Node _ -> scale_tree_map sub_tree factor)
           children)
  ;;
end

(** Sequence operations as conventional interfaces, subsection 2.2.3:
    [map], [filter], and [accumulate] as the book defines them, the two
    enumerators, the signal-flow pipelines built from them, and the
    nested-mapping programs [prime_sum_pairs] and [permutations].
    [accumulate] is OCaml's [List.fold_right] in the book's argument
    order. *)
module Seq_ops = struct
  let map = List_ops.map

  let rec filter predicate sequence =
    match sequence with
    | [] -> []
    | car :: cdr ->
      if predicate car then car :: filter predicate cdr else filter predicate cdr
  ;;

  let rec accumulate op initial sequence =
    match sequence with
    | [] -> initial
    | car :: cdr -> op car (accumulate op initial cdr)
  ;;

  let rec enumerate_interval low high =
    if low > high then [] else low :: enumerate_interval (low + 1) high
  ;;

  let rec enumerate_tree tree =
    match tree with
    | Trees.Leaf n -> [ n ]
    | Trees.Node children ->
      List.fold_left (fun acc sub_tree -> acc @ enumerate_tree sub_tree) [] children
  ;;

  let fib = Sicp_ch1.Sec_1_2.Fibonacci.recursive
  let square n = n * n
  let odd n = n mod 2 <> 0
  let even n = n mod 2 = 0

  let rec sum_odd_squares_tree tree =
    match tree with
    | Trees.Leaf n -> if odd n then square n else 0
    | Trees.Node children ->
      List.fold_left (fun acc sub_tree -> acc + sum_odd_squares_tree sub_tree) 0 children
  ;;

  let sum_odd_squares tree =
    accumulate ( + ) 0 (map square (filter odd (enumerate_tree tree)))
  ;;

  (* the book's first definition of even-fibs, before the abstraction *)
  let even_fibs_slow n =
    let rec next k =
      if k > n
      then []
      else (
        let f = fib k in
        if even f then f :: next (k + 1) else next (k + 1))
    in
    next 0
  ;;

  let even_fibs n =
    accumulate (fun x acc -> x :: acc) [] (filter even (map fib (enumerate_interval 0 n)))
  ;;

  let list_fib_squares n =
    accumulate (fun x acc -> x :: acc) [] (map square (map fib (enumerate_interval 0 n)))
  ;;

  let product_of_squares_of_odd_elements sequence =
    accumulate ( * ) 1 (map square (filter odd sequence))
  ;;

  type personnel_record =
    { is_programmer : bool
    ; salary : int
    }

  let salary_of_highest_paid_programmer records =
    accumulate
      max
      0
      (map
         (fun record -> record.salary)
         (filter (fun record -> record.is_programmer) records))
  ;;

  let flatmap proc seq = accumulate ( @ ) [] (map proc seq)
  let remove item sequence = filter (fun x -> x <> item) sequence
  let prime_sum pair = Sicp_ch1.Sec_1_2.Primality.is_prime (fst pair + snd pair)
  let make_pair_sum pair = fst pair, snd pair, fst pair + snd pair

  let prime_sum_pairs n =
    List_ops.map
      make_pair_sum
      (filter
         prime_sum
         (flatmap
            (fun i -> map (fun j -> i, j) (enumerate_interval 1 (i - 1)))
            (enumerate_interval 1 n)))
  ;;

  let rec permutations s =
    match s with
    | [] -> [ [] ]
    | _ -> flatmap (fun x -> map (fun p -> x :: p) (permutations (remove x s))) s
  ;;
end

(** The picture language, subsection 2.2.4. A painter is a function
    from a frame to the segments it draws there, so painting has no
    side effect; a separate renderer turns a painter's segments into an
    SVG document. [up_split] and [rotate_180] are defined here because
    [corner_split] and the second [square_limit] need them; the book
    leaves both to Exercises 2.44 and 2.50. *)
module Picture = struct
  module Vect = struct
    type t =
      { x : float
      ; y : float
      }

    let make_vect x y = { x; y }
    let xcor_vect v = v.x
    let ycor_vect v = v.y
    let add_vect a b = { x = a.x +. b.x; y = a.y +. b.y }
    let sub_vect a b = { x = a.x -. b.x; y = a.y -. b.y }
    let scale_vect s v = { x = s *. v.x; y = s *. v.y }
  end

  module Frame = struct
    type t =
      { origin : Vect.t
      ; edge1 : Vect.t
      ; edge2 : Vect.t
      }

    let make_frame origin edge1 edge2 = { origin; edge1; edge2 }
    let origin_frame f = f.origin
    let edge1_frame f = f.edge1
    let edge2_frame f = f.edge2
  end

  type segment = Vect.t * Vect.t
  type painter = Frame.t -> segment list

  let frame_coord_map frame =
    fun v ->
    Vect.add_vect
      (Frame.origin_frame frame)
      (Vect.add_vect
         (Vect.scale_vect (Vect.xcor_vect v) (Frame.edge1_frame frame))
         (Vect.scale_vect (Vect.ycor_vect v) (Frame.edge2_frame frame)))
  ;;

  let segments_to_painter segment_list =
    fun frame ->
    let m = frame_coord_map frame in
    List.map (fun (start_v, end_v) -> m start_v, m end_v) segment_list
  ;;

  let transform_painter painter origin corner1 corner2 =
    fun frame ->
    let m = frame_coord_map frame in
    let new_origin = m origin in
    painter
      (Frame.make_frame
         new_origin
         (Vect.sub_vect (m corner1) new_origin)
         (Vect.sub_vect (m corner2) new_origin))
  ;;

  let flip_vert painter =
    transform_painter
      painter
      (Vect.make_vect 0.0 1.0)
      (Vect.make_vect 1.0 1.0)
      (Vect.make_vect 0.0 0.0)
  ;;

  let flip_horiz painter =
    transform_painter
      painter
      (Vect.make_vect 1.0 0.0)
      (Vect.make_vect 0.0 0.0)
      (Vect.make_vect 1.0 1.0)
  ;;

  let shrink_to_upper_right painter =
    transform_painter
      painter
      (Vect.make_vect 0.5 0.5)
      (Vect.make_vect 1.0 0.5)
      (Vect.make_vect 0.5 1.0)
  ;;

  let rotate_90 painter =
    transform_painter
      painter
      (Vect.make_vect 1.0 0.0)
      (Vect.make_vect 1.0 1.0)
      (Vect.make_vect 0.0 0.0)
  ;;

  let rotate_180 painter =
    transform_painter
      painter
      (Vect.make_vect 1.0 1.0)
      (Vect.make_vect 0.0 1.0)
      (Vect.make_vect 1.0 0.0)
  ;;

  let squash_inwards painter =
    transform_painter
      painter
      (Vect.make_vect 0.0 0.0)
      (Vect.make_vect 0.65 0.35)
      (Vect.make_vect 0.35 0.65)
  ;;

  let beside painter1 painter2 =
    let split_point = Vect.make_vect 0.5 0.0 in
    let paint_left =
      transform_painter
        painter1
        (Vect.make_vect 0.0 0.0)
        split_point
        (Vect.make_vect 0.0 1.0)
    in
    let paint_right =
      transform_painter
        painter2
        split_point
        (Vect.make_vect 1.0 0.0)
        (Vect.make_vect 0.5 1.0)
    in
    fun frame -> paint_left frame @ paint_right frame
  ;;

  let below painter1 painter2 =
    let split_point = Vect.make_vect 0.0 0.5 in
    let paint_bottom =
      transform_painter
        painter1
        (Vect.make_vect 0.0 0.0)
        (Vect.make_vect 1.0 0.0)
        split_point
    in
    let paint_top =
      transform_painter
        painter2
        split_point
        (Vect.make_vect 1.0 0.5)
        (Vect.make_vect 0.0 1.0)
    in
    fun frame -> paint_bottom frame @ paint_top frame
  ;;

  let wave_segments : segment list =
    [ Vect.make_vect 0.006 0.396, Vect.make_vect 0.203 0.508
    ; Vect.make_vect 0.203 0.508, Vect.make_vect 0.307 0.412
    ; Vect.make_vect 0.307 0.412, Vect.make_vect 0.295 0.352
    ; Vect.make_vect 0.295 0.352, Vect.make_vect 0.381 0.314
    ; Vect.make_vect 0.381 0.314, Vect.make_vect 0.425 0.145
    ; Vect.make_vect 0.425 0.145, Vect.make_vect 0.408 0.000
    ; Vect.make_vect 0.126 0.000, Vect.make_vect 0.247 0.145
    ; Vect.make_vect 0.247 0.145, Vect.make_vect 0.334 0.145
    ; Vect.make_vect 0.334 0.145, Vect.make_vect 0.375 0.249
    ; Vect.make_vect 0.375 0.249, Vect.make_vect 0.368 0.345
    ; Vect.make_vect 0.590 0.000, Vect.make_vect 0.616 0.145
    ; Vect.make_vect 0.616 0.145, Vect.make_vect 0.634 0.345
    ; Vect.make_vect 0.634 0.345, Vect.make_vect 0.733 0.368
    ; Vect.make_vect 0.733 0.368, Vect.make_vect 0.754 0.222
    ; Vect.make_vect 0.754 0.222, Vect.make_vect 0.760 0.000
    ; Vect.make_vect 0.855 0.000, Vect.make_vect 0.856 0.187
    ; Vect.make_vect 0.856 0.187, Vect.make_vect 0.904 0.432
    ; Vect.make_vect 0.904 0.432, Vect.make_vect 1.000 0.440
    ; Vect.make_vect 1.000 0.633, Vect.make_vect 0.876 0.628
    ; Vect.make_vect 0.876 0.628, Vect.make_vect 0.766 0.632
    ; Vect.make_vect 0.766 0.632, Vect.make_vect 0.682 0.661
    ; Vect.make_vect 0.682 0.661, Vect.make_vect 0.610 0.691
    ; Vect.make_vect 0.610 0.691, Vect.make_vect 0.613 0.766
    ; Vect.make_vect 0.613 0.766, Vect.make_vect 0.578 0.807
    ; Vect.make_vect 0.578 0.807, Vect.make_vect 0.513 0.816
    ; Vect.make_vect 0.513 0.816, Vect.make_vect 0.347 0.818
    ; Vect.make_vect 0.347 0.818, Vect.make_vect 0.266 0.781
    ; Vect.make_vect 0.266 0.781, Vect.make_vect 0.278 0.715
    ; Vect.make_vect 0.278 0.715, Vect.make_vect 0.340 0.691
    ; Vect.make_vect 0.340 0.691, Vect.make_vect 0.353 0.617
    ; Vect.make_vect 0.353 0.617, Vect.make_vect 0.254 0.615
    ; Vect.make_vect 0.254 0.615, Vect.make_vect 0.157 0.634
    ; Vect.make_vect 0.157 0.634, Vect.make_vect 0.046 0.618
    ; Vect.make_vect 0.046 0.618, Vect.make_vect 0.000 0.608
    ]
  ;;

  let rogers_segments : segment list =
    [ Vect.make_vect 0.35 0.55, Vect.make_vect 0.42 0.70
    ; Vect.make_vect 0.42 0.70, Vect.make_vect 0.57 0.72
    ; Vect.make_vect 0.57 0.72, Vect.make_vect 0.65 0.58
    ; Vect.make_vect 0.65 0.58, Vect.make_vect 0.60 0.42
    ; Vect.make_vect 0.60 0.42, Vect.make_vect 0.44 0.40
    ; Vect.make_vect 0.44 0.40, Vect.make_vect 0.35 0.55
    ; Vect.make_vect 0.44 0.58, Vect.make_vect 0.48 0.58
    ; Vect.make_vect 0.56 0.58, Vect.make_vect 0.60 0.58
    ; Vect.make_vect 0.51 0.55, Vect.make_vect 0.51 0.49
    ; Vect.make_vect 0.45 0.45, Vect.make_vect 0.58 0.47
    ; Vect.make_vect 0.20 0.20, Vect.make_vect 0.35 0.35
    ; Vect.make_vect 0.35 0.35, Vect.make_vect 0.62 0.34
    ; Vect.make_vect 0.62 0.34, Vect.make_vect 0.80 0.20
    ]
  ;;

  let wave = segments_to_painter wave_segments
  let rogers = segments_to_painter rogers_segments
  let wave2 = beside wave (flip_vert wave)
  let wave4 = below wave2 wave2

  let flipped_pairs painter =
    let painter2 = beside painter (flip_vert painter) in
    below painter2 painter2
  ;;

  let rec up_split painter n =
    if n = 0
    then painter
    else (
      let smaller = up_split painter (n - 1) in
      below painter (beside smaller smaller))
  ;;

  let rec right_split painter n =
    if n = 0
    then painter
    else (
      let smaller = right_split painter (n - 1) in
      beside painter (below smaller smaller))
  ;;

  let rec corner_split painter n =
    if n = 0
    then painter
    else (
      let up = up_split painter (n - 1) in
      let right = right_split painter (n - 1) in
      let top_left = beside up up in
      let bottom_right = below right right in
      let corner = corner_split painter (n - 1) in
      beside (below painter top_left) (below bottom_right corner))
  ;;

  let square_limit painter n =
    let quarter = corner_split painter n in
    let half = beside (flip_horiz quarter) quarter in
    below (flip_vert half) half
  ;;

  let square_of_four tl tr bl br =
    fun painter ->
    let top = beside (tl painter) (tr painter) in
    let bottom = beside (bl painter) (br painter) in
    below bottom top
  ;;

  let flipped_pairs_of_four painter =
    let combine4 = square_of_four Fun.id flip_vert Fun.id flip_vert in
    combine4 painter
  ;;

  let square_limit_of_four painter n =
    let combine4 = square_of_four flip_horiz Fun.id rotate_180 flip_vert in
    combine4 (corner_split painter n)
  ;;

  (** The one drawing operation this edition needs: turn the segments a
      painter produced into SVG text. *)
  module Svg = struct
    let line (a, b) =
      Printf.sprintf
        {|<line x1="%g" y1="%g" x2="%g" y2="%g"/>|}
        (Vect.xcor_vect a)
        (Vect.ycor_vect a)
        (Vect.xcor_vect b)
        (Vect.ycor_vect b)
    ;;

    let document ~view_box ~size body =
      Printf.sprintf
        {|<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="%s"><g stroke="black" stroke-width="0.008" stroke-linecap="round" fill="none">%s</g></svg>|}
        size
        size
        view_box
        body
    ;;

    (** The smallest box that contains [segments], grown by five
        percent on every side so strokes are not clipped. *)
    let view_box segments =
      let xs =
        List.concat_map (fun (a, b) -> [ Vect.xcor_vect a; Vect.xcor_vect b ]) segments
      in
      let ys =
        List.concat_map (fun (a, b) -> [ Vect.ycor_vect a; Vect.ycor_vect b ]) segments
      in
      let min_x = List.fold_left min (List.hd xs) xs in
      let max_x = List.fold_left max (List.hd xs) xs in
      let min_y = List.fold_left min (List.hd ys) ys in
      let max_y = List.fold_left max (List.hd ys) ys in
      Printf.sprintf
        "%g %g %g %g"
        (min_x -. 0.05)
        (min_y -. 0.05)
        (max_x -. min_x +. 0.1)
        (max_y -. min_y +. 0.1)
    ;;
  end

  let render_to_string frame painter =
    let segments = painter frame in
    Svg.document
      ~view_box:(Svg.view_box segments)
      ~size:200
      (String.concat "" (List.map Svg.line segments))
  ;;

  let write_svg ~path ~size frame painter =
    let segments = painter frame in
    let document =
      Svg.document
        ~view_box:(Svg.view_box segments)
        ~size
        (String.concat "" (List.map Svg.line segments))
    in
    let rec ensure_dir dir =
      if not (Sys.file_exists dir)
      then (
        ensure_dir (Filename.dirname dir);
        Sys.mkdir dir 0o755)
    in
    ensure_dir (Filename.dirname path);
    let out = open_out path in
    output_string out document;
    close_out out
  ;;
end
