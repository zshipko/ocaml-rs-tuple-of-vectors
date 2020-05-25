let () =
  for i = 1 to 200 do
    let (a, b, c) = Ocaml_rust_starter.hello_world i in
    assert (Array.length a = i);
    assert (Array.length b = i);
    assert (Array.length c = i);
    let x = Ocaml_rust_starter.get_a_x a in
    assert (Array.length x = i);
    for i = 0 to i - 1 do
      assert (x.(i) = Printf.sprintf "%d" i)
    done;
    let s = Printf.sprintf "%d" i in
    Printf.printf "%s\n%!" s;
    if i mod 10 = 0 then
      let () = Gc.print_stat stdout in
      let () = Gc.minor () in
      Gc.full_major ()
  done;
  Gc.minor ();
  Gc.full_major ();
  Gc.print_stat stdout
