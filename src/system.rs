use crate::world::{World, WorldData};

trait SystemParam {
    type Value: WorldData;

    type Result<'a>;

    fn fetch(data: &mut Self::Value) -> Self::Result<'_>;
}

impl<WD: WorldData> SystemParam for &WD {
    type Value = WD;
    type Result<'a> = &'a WD;

    fn fetch(value: &mut Self::Value) -> Self::Result<'_> {
        &(*value)
    }
}

impl<WD: WorldData> SystemParam for &mut WD {
    type Value = WD;
    type Result<'a> = &'a mut WD;

    fn fetch(value: &mut Self::Value) -> Self::Result<'_> {
        value
    }
}

pub trait System {
    fn execute(&self, world: &mut World);
}

pub trait SystemParamFn<Params> {
    fn execute(&self, world: &mut World);
}

impl<Function, Z: SystemParam> SystemParamFn<Z> for Function
where
        for<'a> &'a Function: Fn(Z::Result<'_>) + Fn(Z),
{
    fn execute(&self, world: &mut World) {
        let mut z = Z::Value::take(world);
        (&self)(Z::fetch(&mut z));
        z.release(world);
    }
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

#[derive(Default)]
pub struct Systems(Vec<Box<dyn System>>);
impl Systems {
    /// Run every system once.
    pub fn run(&self, world: &mut World) {
        for system in &self.0 {
            system.execute(world);
        }
    }

    /// Add a new system to run.
    pub fn push<Sys: System + 'static>(&mut self, system: impl IntoSystem<Sys>) {
        self.0.push(Box::new(system.into_system()));
    }
}


impl<Function, X: SystemParam, Y: SystemParam> SystemParamFn<(X, Y)> for Function
where
        for<'a> &'a Function: Fn(X::Result<'_>, Y::Result<'_>) + Fn(X, Y),
{
    fn execute(&self, world: &mut World) {
        // Dynamically take parameters from the world (in this case, hardcoded for simplicity)
        let mut x = X::Value::take(world);
        let mut y = Y::Value::take(world);

        // Call the function with fetched parameters
        (&self)(X::fetch(&mut x), Y::fetch(&mut y));

        // Release the data back into the world
        x.release(world);
        y.release(world);
    }   
}
