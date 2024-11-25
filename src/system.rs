use crate::world::World;

pub trait System {
    fn execute(&self, world: &mut World);
}

pub trait WorldData: 'static {
    fn take(world: &mut World) -> Self;

    fn release(self, world: &mut World);
}


pub trait SystemParam {
    type Data: WorldData;

    type Result<'a>;

    fn fetch(data: &mut Self::Data) -> Self::Result<'_>;
}


impl<WD: WorldData> SystemParam for &WD {
    type Data = WD;
    type Result<'a> = &'a WD;

    fn fetch(data: &mut Self::Data) -> Self::Result<'_> {
        &(*data)
    }
}

impl<WD: WorldData> SystemParam for &mut WD {
    type Data = WD;
    type Result<'a> = &'a mut WD;

    fn fetch(data: &mut Self::Data) -> Self::Result<'_> {
        data
    }
}


pub trait SystemParamFn<Params> {
    fn execute(&self, world: &mut World);
}

pub struct SystemStore<Params>(Box<dyn SystemParamFn<Params>>);
impl<Params> System for SystemStore<Params> {
    fn execute(&self, world: &mut World) {
        self.0.execute(world);
    }
}

pub trait IntoSystem<Result: System> {
    fn into_system(self) -> Result;
}

impl<F, Params> IntoSystem<SystemStore<Params>> for F
where
    F: SystemParamFn<Params> + 'static,
{
    fn into_system(self) -> SystemStore<Params> {
        SystemStore(Box::new(self))
    }
}


impl<Function, X: SystemParam, Y: SystemParam> SystemParamFn<(X, Y)> for Function
where
    for<'a> &'a Function: Fn(X::Result<'_>, Y::Result<'_>) + Fn(X, Y),
{
    fn execute(&self, world: &mut World) {
        // Dynamically take parameters from the world (in this case, hardcoded for simplicity)
        let mut x = X::Data::take(world);
        let mut y = Y::Data::take(world);

        // Call the function with fetched parameters
        (&self)(X::fetch(&mut x), Y::fetch(&mut y));

        // Release the data back into the world
        x.release(world);
        y.release(world);
    }
}
