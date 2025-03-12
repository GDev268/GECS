use std::{
    any::TypeId,
    cell::{RefCell},
    collections::HashMap,
    rc::Rc,
};

use uuid::Uuid;

use crate::entity::{AnyEcs, Component, Entity};

pub trait Archetype {
    fn set_component(&mut self, entity: &Entity, component: Box<dyn Component>);

    fn get_component(&self, entity: &Entity) -> Option<Rc<RefCell<dyn Component>>>;

    fn get_component_with_id(&self, id: &Uuid) -> Option<Rc<RefCell<dyn Component>>>;

    fn kill(&mut self, entity: &Entity);

    fn add_entity(&mut self, entity: &Entity);

    fn type_id(&self) -> TypeId;

    fn get_size(&self) -> usize;

    fn get_components(&self) -> HashMap<Uuid,Rc<RefCell<dyn Component>>>;
}

pub struct ComponentArchetype<C: Component> {
    pub components: HashMap<Uuid, Option<Rc<RefCell<C>>>>,
}

impl<C: Component> Archetype for ComponentArchetype<C> {
    fn set_component(&mut self, entity: &Entity, component: Box<dyn Component>) {

        let test: Option<Rc<RefCell<C>>>  = Some(Rc::new(RefCell::new(
            *component
                .as_any_box()
                .downcast()
                .expect("Failed to downcast component to any"),
        )));


        self.components
            .get_mut(&entity.id).unwrap().replace(test.unwrap());
    }

    fn get_component(&self, entity: &Entity) -> Option<Rc<RefCell<dyn Component>>> {
        self.components
            .get(&entity.id)
            .unwrap()
            .as_ref()
            .cloned()
            .map(|component| component as Rc<RefCell<dyn Component>>)
    }

    fn get_component_with_id(&self, id: &Uuid) -> Option<Rc<RefCell<dyn Component>>> {
        self.components
            .get(&id)
            .unwrap()
            .as_ref()
            .cloned()
            .map(|component| component as Rc<RefCell<dyn Component>>)
    }

    fn kill(&mut self, entity: &Entity) {
        self.components.get(&entity.id).replace(&None);
    }

    fn add_entity(&mut self, entity: &Entity) {
        self.components.entry(entity.id).or_insert((|| None)());
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn get_size(&self) -> usize {
        self.components.len()
    }

    fn get_components(&self) -> HashMap<Uuid,Rc<RefCell<dyn Component>>> {
        let mut component_map: HashMap<Uuid, Rc<RefCell<dyn Component>>> = HashMap::with_capacity(self.get_size());

        for (id, component) in self.components.iter() {
            let test = component.clone().unwrap();

            component_map.entry(*id).insert_entry(test);
        }

        return component_map;
    }
}

pub struct Storage {
    pub archetypes: HashMap<TypeId, Rc<RefCell<dyn Archetype>>>,
    pub entities: Vec<Entity>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            archetypes: HashMap::new(),
            entities: Vec::new(),
        }
    }


    pub fn entity_len(&self) -> usize {
        self.entities.len()
    }


}
