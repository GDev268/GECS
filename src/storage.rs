use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::Deref,
    rc::Rc,
};

use crate::entity::{generate_id, Component, Entity};

pub trait Archetype {
    fn set_component(&mut self, entity: usize, component: Box<dyn Component>);

    fn get_component(&self, entity: usize) -> Option<Rc<RefCell<dyn Component>>>;

    fn kill(&mut self, entity: usize);

    fn add_entity(&mut self);

    fn get_type_id(&self) -> TypeId;
}

struct ComponentArchetype<C: Component> {
    components: Vec<Option<Rc<RefCell<C>>>>,
}

impl<C: Component> ComponentArchetype<C> {
    pub fn new(size: usize) -> Self {
        let mut components = Vec::with_capacity(size);

        for _ in 0..size {
            components.push(None);
        }

        Self { components }
    }
}

impl<C: Component> Archetype for ComponentArchetype<C> {
    fn set_component(&mut self, entity: usize, component: Box<dyn Component>) {
        self.components[entity] = Some(Rc::new(RefCell::new(
            *component
                .to_box_any()
                .downcast()
                .expect("Failed to downcast component to any"),
        )));
    }

    fn get_component(&self, entity: usize) -> Option<Rc<RefCell<dyn Component>>> {
        self.components
            .get(entity)
            .unwrap()
            .as_ref()
            .cloned()
            .map(|component| component as Rc<RefCell<dyn Component>>)
    }

    fn kill(&mut self, entity: usize) {
        self.components[entity] = None;
    }

    fn add_entity(&mut self) {
        self.components.push(None);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<C>()
    }
}

pub struct Storage {
    resources: HashMap<TypeId, Rc<RefCell<dyn Any>>>,

    archetypes: HashMap<TypeId, Rc<RefCell<dyn Archetype>>>,

    entities: Vec<Entity>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            archetypes: HashMap::new(),
            entities: Vec::new(),
        }
    }

    pub fn add_resource<T: Any>(&mut self, resource: T) {
        self.resources
            .insert(resource.type_id(), Rc::new(RefCell::new(resource)));
    }

    pub fn get_resource_ref<T: Any>(&self) -> Option<Ref<T>> {
        if let Some(resource) = self.resources.get(&TypeId::of::<T>()) {
            let borrow = resource.as_ref().borrow();

            Some(Ref::map(borrow, |any| any.downcast_ref::<T>().unwrap()))
        } else {
            None
        }
    }

    pub fn get_resource_mut<T: Any>(&self) -> Option<RefMut<T>> {
        if let Some(resource) = self.resources.get(&TypeId::of::<T>()) {
            let borrow = resource.as_ref().borrow_mut();

            Some(RefMut::map(borrow, |any| any.downcast_mut::<T>().unwrap()))
        } else {
            None
        }
    }

    pub fn new_archetype<C: Component>(&mut self) {
        self.archetypes.entry(TypeId::of::<C>()).or_insert_with(|| {
            Rc::new(RefCell::new(ComponentArchetype::<C>::new(
                self.entities.len(),
            )))
        });
    }

    pub fn get_archetype(&mut self, id: TypeId) -> Option<Rc<RefCell<dyn Archetype>>> {
        self.archetypes.get(&id).cloned()
    }

    pub fn insert_component(&mut self, entity: usize, component: impl Component) {
        let component: Box<dyn Component> = Box::new(component);

        self.archetypes
            .get(&(*component).type_id())
            .expect("Failed to get the component archetype!")
            .deref()
            .borrow_mut()
            .set_component(entity, component);
    }

    pub fn spawn(&mut self) {
        for archetype in self.archetypes.values() {
            archetype.borrow_mut().add_entity();
        }

        self.entities.push(Entity::new(generate_id()));
    }

    pub fn kill(&mut self, entity: Entity) {
        for archetype in self.archetypes.values() {
            archetype
                .deref()
                .borrow_mut()
                .kill(entity.get_id() as usize);
        }
    }
}
