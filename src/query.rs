use std::{any::TypeId, cell::{Ref, RefCell, RefMut}, marker::PhantomData, rc::Rc};

use crate::entity::{Component, ComponentBorrow};


pub trait Queryable {
    /// The type that will be returned from querying with this.
    type QueryResult<'a>;

    /// The `TypeId`s for the components this query needs.
    fn type_id() -> TypeId;
}

impl<'b, A: Component> Queryable for &'b A {
    type QueryResult<'a> = Ref<'a, A>;

    fn type_id() -> TypeId {
        TypeId::of::<A>()
    }
}
impl<'b, A: Component> Queryable for &'b mut A {
    type QueryResult<'a> = RefMut<'a, A>;

    fn type_id() -> TypeId {
        TypeId::of::<A>()
    }
}
pub trait QueryableFn {
    type QueryResult<'a>;

    fn type_ids() -> Vec<TypeId>;
}


/// Tuple implementation for two components.
impl<Y: Queryable + ComponentBorrow, Z: Queryable + ComponentBorrow> QueryableFn for (Y, Z)
{
    type QueryResult<'a> = (Y::QueryResult<'a>, Z::QueryResult<'a>);

    fn type_ids() -> Vec<TypeId> {
        vec![Y::type_id(), Z::type_id()]
    }
}

/// Tuple implementation for single component tuples.
impl<Z: Queryable + ComponentBorrow> QueryableFn for (Z,)
{
    type QueryResult<'a> = (Z::QueryResult<'a>,);

    fn type_ids() -> Vec<TypeId> {
        vec![Z::type_id()]
    }
}

impl<Z: Queryable + ComponentBorrow> QueryableFn for Z
{
    type QueryResult<'a> = (Z::QueryResult<'a>,);

    fn type_ids() -> Vec<TypeId> {
        vec![Z::type_id()]
    }
}

pub struct Query<Q: QueryableFn> {
    _bundle_ty: PhantomData<Q>,
}

impl<Q: QueryableFn> Query<Q> {
    fn test(&self) -> Vec<TypeId> {
        Q::type_ids()
    }
}