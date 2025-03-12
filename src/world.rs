use std::{any::TypeId, cell::RefCell, collections::HashMap, rc::Rc};

use uuid::Uuid;

use crate::{entity::{Component, Entity, EntityBuilder}, storage::{Archetype, ComponentArchetype, Storage}};
use crate::system::{IntoSystem, System};

pub struct World {
    pub storage: Storage,
    //pub systems: Rc<RefCell<Systems>>,
}

impl World {
    pub fn new() -> Self {
        Self { storage: Storage::new(), /*systems: Rc::new(RefCell::new(Systems::default()))*/ }
    }

    pub fn new_archetype<C: Component>(&mut self) {
        self.storage.archetypes.entry(TypeId::of::<C>()).or_insert_with(|| {
            Rc::new(RefCell::new(ComponentArchetype::<C> {
                components: HashMap::with_capacity(self.storage.entities.len()),
            }))
        });
    }

    pub fn get_archetype<C: Component>(&mut self) -> Option<Rc<RefCell<dyn Archetype>>> {
        self.storage.archetypes.get(&TypeId::of::<C>()).cloned()
    }

    pub fn get_archetype_const(&mut self,type_id: TypeId) -> Option<Rc<RefCell<dyn Archetype>>> {
        self.storage.archetypes.get(&type_id).cloned()
    }

    pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
        let box_component: Box<dyn Component> = Box::new(component);

        self.storage.archetypes
            .get(&TypeId::of::<C>())
            .unwrap()
            .borrow_mut()
            .set_component(entity, box_component);
    }

    pub fn create_entity(&mut self) -> EntityBuilder<'_> {
        let entity_builder = EntityBuilder::new(
            Entity{
                id: Uuid::new_v4(),
            },
            self);

        return entity_builder;
    }

    pub fn kill(&mut self, entity: &Entity) {
        if let Some(index) = self.storage.entities.iter().position(|entity| entity.id == entity.id) {
            for (_, archetype) in self.storage.archetypes.iter() {
                archetype.borrow_mut().kill(entity);
            }

            self.storage.entities.remove(index);
        }
    }

    pub fn has_component<C: Component>(&self, entity: &Entity) -> bool {
        if let Some(archetype) = self.storage.archetypes.get(&TypeId::of::<C>()) {
            return archetype.borrow().get_component(entity).is_some();
        }

        return false;
    }

    /*#[inline]
    pub fn add_system<S: System + 'static>(&mut self, system: impl IntoSystem<S>) {
        self.systems.borrow_mut().push(system);
    }

    #[inline]
    pub fn run_once(&mut self) {
        self.systems.clone().borrow().run(self);
    }*/

}


pub trait WorldData: 'static {

    fn take(world: &mut World) -> Self;

    fn release(self, world: &mut World);
}