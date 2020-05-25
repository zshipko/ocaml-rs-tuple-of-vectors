use ocaml::{FromValue, Pointer};

unsafe extern "C" fn finalize_a(v: ocaml::Value) {
    let ptr: ocaml::Pointer<A> = ocaml::Pointer::from_value(v);
    println!("DROP A");
    ptr.drop_in_place();
}

unsafe extern "C" fn finalize_b(v: ocaml::Value) {
    let ptr: ocaml::Pointer<B> = ocaml::Pointer::from_value(v);
    println!("DROP B");
    ptr.drop_in_place();
}

unsafe extern "C" fn finalize_c(v: ocaml::Value) {
    let ptr: ocaml::Pointer<C> = ocaml::Pointer::from_value(v);
    println!("DROP C");
    ptr.drop_in_place();
}

#[derive(Clone)]
struct A {
    x: String,
    y: f32,
    z: ocaml::Int,
}

impl ocaml::Custom for A {
    const TYPE: ocaml::custom::CustomType = ocaml::custom::CustomType {
        name: "a\0",
        fixed_length: None,
        ops: ocaml::custom::CustomOps {
            finalize: Some(finalize_a),
            ..ocaml::custom::DEFAULT_CUSTOM_OPS
        },
    };

    const USED: usize = 1;
    const MAX: usize = 1000;
}

impl ocaml::Custom for B {
    const TYPE: ocaml::custom::CustomType = ocaml::custom::CustomType {
        name: "b\0",
        fixed_length: None,
        ops: ocaml::custom::CustomOps {
            finalize: Some(finalize_b),
            ..ocaml::custom::DEFAULT_CUSTOM_OPS
        },
    };

    const USED: usize = 1;
    const MAX: usize = 1000;
}

impl ocaml::Custom for C {
    const TYPE: ocaml::custom::CustomType = ocaml::custom::CustomType {
        name: "c\0",
        fixed_length: None,
        ops: ocaml::custom::CustomOps {
            finalize: Some(finalize_c),
            ..ocaml::custom::DEFAULT_CUSTOM_OPS
        },
    };

    const USED: usize = 1;
    const MAX: usize = 1000;
}

#[derive(Clone)]
struct B {
    x: A,
    y: bool,
    z: (ocaml::Int, ocaml::Int),
}

#[derive(Clone)]
struct C {
    x: f32,
    y: f64,
    z: f64,
}

#[ocaml::func]
pub fn hello_world(n: ocaml::Int) -> (Vec<Pointer<A>>, Vec<Pointer<B>>, Vec<Pointer<C>>) {
    let mut a = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        a.push(Pointer::alloc_custom(A {
            x: format!("{}", i),
            y: i as f32,
            z: i as ocaml::Int,
        }))
    }

    let mut b = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        b.push(Pointer::alloc_custom(B {
            x: a[i].as_ref().clone(),
            y: i % 2 == 0,
            z: (i as ocaml::Int, i as ocaml::Int + 1),
        }))
    }

    let mut c = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        c.push(Pointer::alloc_custom(C {
            x: i as f32,
            y: i as f64,
            z: i as f64,
        }))
    }

    (a, b, c)
}
