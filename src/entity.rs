use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

use gecs_macro::bundle_macro;

pub fn generate_id() -> u32 {
    static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    ID_COUNTER.fetch_add(1, Ordering::Relaxed) as u32
}

pub trait Component: EcsAny {}

pub trait EcsAny: Any {
    fn to_box_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any> EcsAny for T {
    fn to_box_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }
}

pub trait ComponentBorrow {
    type Component: Component;
}
impl<'a, C: Component> ComponentBorrow for &'a C {
    type Component = C;
}
impl<'a, C: Component> ComponentBorrow for &'a mut C {
    type Component = C;
}

pub trait Bundle {
    fn components(self) -> Vec<Box<dyn Component>>;
}

//Use cargo expand to see the macro result
macro_rules! bundle_gen {
    ($_: ident) => {};
    ($_: ident $($ty: ident)*) => {
        bundle_macro!($($ty)*);

        bundle_gen!($($ty)*);
    }
}

bundle_gen!(B1 B1 B2 B3 B4 B5 B6 B7 B8 B9 B10 B11 B12 B13 B14 B15 B16 B17 B18 B19 B20);

pub struct Entity {
    id: u32,
    gen: u32,
    tag: Vec<String>,
    name: String,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            gen: 0,
            tag: Vec::new(),
            name: String::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
