use std::any::{Any};
use std::fmt::Debug;

trait Component: Debug {}

macro_rules! sys_update_list {
    ( $( $x:expr ),* ) => {
        fn sys_update(o: Box<dyn Any>) {$(call_component($x, &*o);)*}
    };
}

#[derive(Debug)]
struct Posable {
    x: f64,
}

#[derive(Debug)]
struct Massable {
    mass: f64,
}

impl Component for Posable {}

impl Component for Massable {}

struct Entity {
    components: Vec<Box<dyn Any>>,
}

fn sys_posable(x: &Posable) -> f64 {
    println!("Possable!!");
    x.x
}

fn sys_masable(x: &Massable) -> f64 {
    println!("Masable!!");
    x.mass
}

sys_update_list! {
    sys_posable,
    sys_masable
}

fn main() {
    let sys_update: fn(Box<dyn Any>) = sys_update;

    let entity = Entity {
        components: vec![
            Box::new(Posable { x: 42.0 }),
            Box::new(Massable { mass: 43.0 }),
        ]
    };

    for component in entity.components {
        sys_update(component);
    }
}

fn call_component<F, T: Any>(func: F, val: &dyn Any) -> ()
    where F: Fn(&T) -> f64 {
    if let Some(value) = val.downcast_ref::<T>() {
        func(value);
    }
}
