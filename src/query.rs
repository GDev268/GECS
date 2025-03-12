use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::rc::Rc;
use uuid::Uuid;
use crate::entity::{AnyEcs, Component, ComponentBorrow, Entity};
use crate::world::{World, WorldData};

pub trait QueryFilter {
    fn apply_filter<'a>(world: &World, entities: &mut Vec<&'a Entity>);
}

pub trait QueryFilterFn {
    fn apply_filters<'a>(world: &World, entities: &mut Vec<&Entity>) -> Vec<Uuid>;
}

impl<Z: QueryFilter> QueryFilterFn for Z {
    fn apply_filters(world: &World, entities: &mut Vec<&Entity>) -> Vec<Uuid> {
        Z::apply_filter(world, entities);

        return entities.iter().map(|entity| entity.id).collect();
    }
}

impl<Z: QueryFilter> QueryFilterFn for (Z,) {
    fn apply_filters(world: &World, entities: &mut Vec<&Entity>) -> Vec<Uuid> {
        Z::apply_filter(world, entities);

        return entities.iter().map(|entity| entity.id).collect();
    }
}

impl<Y: QueryFilter, Z: QueryFilter> QueryFilterFn for (Y, Z) {
    fn apply_filters(world: &World, entities: &mut Vec<&Entity>) -> Vec<Uuid> {
        // Apply filter for the first element of the tuple
        Y::apply_filter(world, entities);

        // Apply filter for the second element of the tuple
        Z::apply_filter(world, entities);

        return entities.iter().map(|entity| entity.id).collect();
    }
}

pub struct With<C: Component>(PhantomData<C>);

impl<C:Component> QueryFilter for With<C> {
    fn apply_filter<'a>(world: &World, entities: &mut Vec<&'a Entity>) {
        entities.retain(|entity| {world.has_component::<C>(entity)})
    }
}

pub struct Without<C: Component>(PhantomData<C>);

impl<C:Component> QueryFilter for Without<C> {
    fn apply_filter<'a>(world: &World, entities: &mut Vec<&'a Entity>) {
        entities.retain(|entity| {!world.has_component::<C>(entity)})
    }
}

pub struct NoFilter;

impl QueryFilter for NoFilter {
    fn apply_filter<'a>(_world: &World, _entities: &mut Vec<&'a Entity>) {
        // Do nothing, meaning no filtering happens.
    }
}

pub trait Queryable {
    type QueryResult<'a>;

    fn type_id() -> TypeId;

    fn get_component<'a>(components: &mut impl Iterator<Item = &'a Rc<RefCell<dyn Component>>>) -> Self::QueryResult<'a>;
}

impl<'b, C: Component> Queryable for &'b C {
    type QueryResult<'a> = Ref<'a,C>;

    fn type_id() -> TypeId {
        TypeId::of::<C>()
    }

    fn get_component<'a>(components: &mut impl Iterator<Item=&'a Rc<RefCell<dyn Component>>>) -> Self::QueryResult<'a> {
        Ref::map(components.next().unwrap().borrow(), |component| {
            component.as_any_ref().downcast_ref().unwrap()
        })
    }
}

impl<'b, C: Component> Queryable for &'b mut C {
    type QueryResult<'a> = RefMut<'a,C>;

    fn type_id() -> TypeId {
        TypeId::of::<C>()
    }

    fn get_component<'a>(components: &mut impl Iterator<Item=&'a Rc<RefCell<dyn Component>>>) -> Self::QueryResult<'a> {
        RefMut::map(components.next().unwrap().borrow_mut(), |component| {
            component.as_mut_any().downcast_mut().unwrap()
        })
    }
}


pub trait QueryableFn {
    type QueryResult<'a>;

    fn type_ids() -> Vec<TypeId>;

    fn get_components<'a>(components: &mut impl Iterator<Item=&'a Rc<RefCell<dyn Component>>>) -> Self::QueryResult<'a>;
}

impl<Z: Queryable + ComponentBorrow> QueryableFn for Z {
    type QueryResult<'a> = Z::QueryResult<'a>;

    fn type_ids() -> Vec<TypeId> {
        return vec![Z::type_id()];
    }

    fn get_components<'a>(components: &mut impl Iterator<Item=&'a Rc<RefCell<dyn Component>>>) -> Self::QueryResult<'a> {
        Z::get_component(components)
    }
}

impl<Z: Queryable + ComponentBorrow> QueryableFn for (Z,) {
    type QueryResult<'a> = Z::QueryResult<'a>;

    fn type_ids() -> Vec<TypeId> {
        return vec![Z::type_id()];
    }

    fn get_components<'a>(components: &mut impl Iterator<Item=&'a Rc<RefCell<dyn Component>>>) -> Self::QueryResult<'a> {
        Z::get_component(components)
    }
}

impl<Y: Queryable + ComponentBorrow, Z: Queryable + ComponentBorrow> QueryableFn for (Y, Z) {
    type QueryResult<'a> = (Y::QueryResult<'a>, Z::QueryResult<'a>);

    fn type_ids() -> Vec<TypeId> {
        return vec![Y::type_id(),Z::type_id()];
    }

    fn get_components<'a>(components: &mut impl Iterator<Item=&'a Rc<RefCell<dyn Component>>>) -> Self::QueryResult<'a> {
        (Y::get_component(components),Z::get_component(components))
    }
}

#[derive(Default)]
pub struct Query<Q: QueryableFn, F: QueryFilterFn = NoFilter> {
    pub bundles: Vec<(Uuid,Vec<Rc<RefCell<dyn Component>>>)>,
    phantom_query: PhantomData<Q>,
    phantom_filter: PhantomData<F>,
}


impl<Q: QueryableFn + 'static,F: QueryFilterFn + 'static> WorldData for Query<Q,F> {
    
    fn take(world: &mut World) -> Self {
        let types = Q::type_ids();
        let num_types = types.len();
        let mut archetypes = Vec::with_capacity(num_types);

        let mut filtering_entities:Vec<&Entity> = world.storage.entities.iter().collect();

        let entities = F::apply_filters(world, &mut filtering_entities);

        let mut bundles:Vec<(Uuid, Vec<Rc<RefCell<dyn Component>>>)> = Vec::new();

        for type_id in types {
            archetypes.push(world.get_archetype_const(type_id).expect("Failed to get archetype!"));
        }

        for entity in entities.iter() {
            let mut cache = Vec::with_capacity(num_types);
            for archetype in archetypes.iter() {
                if let Some(component) = archetype.borrow_mut().get_component_with_id(entity) {
                    cache.push(component);
                } else {
                    continue;
                }
            }

            bundles.push((*entity, cache));
        }

        return Self{bundles, phantom_query: PhantomData, phantom_filter: PhantomData};
    }

    fn release(self, world: &mut World) {

    }
}