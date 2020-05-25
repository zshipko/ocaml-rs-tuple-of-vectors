let () =
  for i = 1 to 200 do
    let (a, b, c) = Ocaml_rust_starter.hello_world i in
    assert (Array.length a = i);
    assert (Array.length b = i);
    assert (Array.length c = i);
    Printf.printf "%d\n%!" i;
    if i mod 10 = 0 then
      let () = Gc.print_stat stdout in
      let () = Gc.minor () in
      Gc.full_major ()
  done;
  Gc.minor ();
  Gc.full_major ();
  Gc.print_stat stdout
