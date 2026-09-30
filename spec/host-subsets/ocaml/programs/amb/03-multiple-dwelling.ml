let () =
  let baker = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let cooper = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let fletcher = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let miller = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let smith = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (baker <> 5);
  require (cooper <> 1);
  require (fletcher <> 1);
  require (fletcher <> 5);
  require (miller > cooper);
  require (smith - fletcher <> 1);
  require (fletcher - smith <> 1);
  require (baker <> cooper);
  require (baker <> fletcher);
  require (baker <> miller);
  require (baker <> smith);
  require (cooper <> fletcher);
  require (cooper <> miller);
  require (cooper <> smith);
  require (fletcher <> miller);
  require (fletcher <> smith);
  require (miller <> smith);
  print_endline
    ("baker " ^ string_of_int baker ^ ", cooper " ^ string_of_int cooper ^ ", fletcher "
     ^ string_of_int fletcher ^ ", miller " ^ string_of_int miller ^ ", smith "
     ^ string_of_int smith)
