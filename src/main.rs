use std::{
    cell::{Ref, RefCell, RefMut}, collections::HashMap, rc::Rc
};
use uuid::Uuid;
use crate::entity::{Component, ComponentBorrow, Entity};
use crate::query::Query;
use crate::world::World;

pub mod entity;
pub mod world;
pub mod storage;
pub mod query;
pub mod system;

#[derive(Debug)]
struct ComponentTest {
    test_field: String,
}

impl Component for ComponentTest {}
impl ComponentBorrow for ComponentTest { type Component = ComponentTest; }

fn main() {
    let mut world = World::new();

    world.new_archetype::<ComponentTest>();

    world.create_entity().with(ComponentTest{test_field: "Hello Default!".to_string()}).spawn();

    println!("{:?}",world.storage.entities);

    let archetype = world.get_archetype::<ComponentTest>().unwrap();

    let components = archetype.borrow_mut().get_components();


    let mut entity = world.storage.entities[0].clone();


    let component: Ref<'_, ComponentTest> = get_component(&components,&mut entity);

    println!("TestComponent Value: {:?}", component.test_field);

    drop(component);


    let mut component: RefMut<'_,ComponentTest> = get_mut_component(&components, &mut entity);

    component.test_field = String::from("Modified with the get_mut_component()");

    drop(component);


    let component: Ref<'_, ComponentTest> = get_component(&components,&mut entity);

    println!("TestComponent Value: {:?}", component.test_field);

    drop(component);



    //world.add_system(test);

    //world.run_once()
}



fn test(query2: &mut Query<&ComponentTest>) {
    println!("TEST THIS SHIT!");
}

fn get_component<'a,'b, C:Component>(components: &'a HashMap<Uuid, Rc<RefCell<dyn Component>>>,entity: &'b Entity) -> Ref<'a,C>{
    let test = components.get(&entity.id).unwrap();

    Ref::map(test.borrow(), |component| component.as_any_ref().downcast_ref().unwrap())
}

fn get_mut_component<'a,'b, C:Component>(components: &'a HashMap<Uuid, Rc<RefCell<dyn Component>>>,entity: &'b Entity) -> RefMut<'a,C>{
    let test = components.get(&entity.id).unwrap();

    RefMut::map(test.borrow_mut(), |component| component.as_mut_any().downcast_mut().unwrap())
}
