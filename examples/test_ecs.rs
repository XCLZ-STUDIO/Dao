use std::any::Any;
use std::fmt::Debug;

trait Component: Debug {}

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

fn main() {
    let entity = Entity {
        components: vec![
            Box::new(Posable { x: 42.0 }),
            Box::new(Massable { mass: 43.0 }),
        ]
    };

    for component in entity.components {
        if let Some(posable) = component.downcast_ref::<Posable>() {
            println!("posable x = {:?}", posable.x);
        } else if let Some(massable) = component.downcast_ref::<Massable>() {
            println!("massable mass = {:?}", massable.mass);
        }
    }
}
