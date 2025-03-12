use std::fmt::Debug;
use std::{
    any::{Any, TypeId},
    vec,
};

use uuid::Uuid;

use crate::world::World;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: Uuid,
}

pub struct EntityBuilder<'a> {
    pub entity: Entity,
    components: Vec<(TypeId, Box<dyn Component>)>,
    pub world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity: Entity, world: &'a mut World) -> Self {
        Self {
            entity,
            components: Vec::new(),
            world,
        }
    }

    #[inline]
    pub fn with_components<B: Bundle>(mut self, bundle: B) -> Self {
        self.components.extend(bundle.fetch_components());

        self
    }

    #[inline]
    pub fn with<C: Component>(mut self, component: C) -> Self {
        self.components
            .push((TypeId::of::<C>(),Box::new(component)));

        self
    }


    #[inline]
    pub fn spawn(self) {
        for (_, archetype) in self.world.storage.archetypes.iter() {
            archetype.borrow_mut().add_entity(&self.entity);

        }

        for (type_id, component) in self.components {
            self.world.storage.archetypes.get(&type_id).unwrap().borrow_mut().set_component(&self.entity, component);
        }

        self.world.storage.entities.push(self.entity);
    }
}

pub trait Component: AnyEcs + Debug {}

pub trait AnyEcs: Any {
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
    fn as_any_ref(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T: Any> AnyEcs for T {
    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

    fn as_any_ref(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub trait Bundle {
    fn fetch_components(self) -> Vec<(TypeId, Box<dyn Component>)>;
}

impl<Z: Component> Bundle for Z {
    fn fetch_components(self) -> Vec<(TypeId, Box<dyn Component>)> {
        return vec![(TypeId::of::<Z>(), Box::new(self))];
    }
}
impl<Y: Component, Z: Component> Bundle for (Y, Z) {
    fn fetch_components(self) -> Vec<(TypeId, Box<dyn Component>)> {
        return vec![
            (TypeId::of::<Y>(), Box::new(self.0)),
            (TypeId::of::<Z>(), Box::new(self.1)),
        ];
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
