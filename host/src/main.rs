use crate::bindings::ochagavia::test::host_types::{Host, HostRng};
use anyhow::Context;
use wasmtime::component::{Component, HasData, Linker, Resource, ResourceAny, ResourceTable};
use wasmtime::{Engine, Store};

mod bindings {
    wasmtime::component::bindgen!({
        path: "../wit",
        with: {
            "ochagavia:test/host-types.rng": crate::Rng,
        },
    });
}

#[derive(Default)]
pub struct MyState {
    table: ResourceTable,
}

impl Host for MyState {}
impl HostRng for MyState {
    fn next_u32(&mut self, self_: Resource<Rng>, upper_bound_inclusive: u32) -> u32 {
        let rng = self.table.get_mut(&self_).unwrap();
        rng.next_u32(upper_bound_inclusive)
    }

    fn drop(&mut self, rep: Resource<Rng>) -> anyhow::Result<()> {
        println!("dropping rng");
        self.table.delete(rep)?;
        Ok(())
    }
}

impl HasData for Rng {
    type Data<'a> = &'a mut MyState;
}

#[derive(Default)]
pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn next_u32(&mut self, upper_bound_inclusive: u32) -> u32 {
        let next = self.state % (upper_bound_inclusive + 1);
        self.state += 1;
        next
    }
}

fn main() {
    let component_paths = [
        "../component/rust/real-component.wasm",
        "../component/python/component.wasm",
    ];

    for component_path in component_paths {
        println!("---\nTesting component: `{component_path}`\n---");

        let mut component = LoadedComponent::load(component_path).unwrap();
        let greeter = component.new_greeter().unwrap();

        for _ in 0..4 {
            let result = component.greet(greeter).unwrap();
            println!("Result: {}", result);
        }

        greeter.resource_drop(component.store).unwrap()
    }
}

struct LoadedComponent {
    store: Store<MyState>,
    world: bindings::MyWorld,
}

impl LoadedComponent {
    fn load(path: &str) -> wasmtime::Result<Self> {
        let engine = Engine::default();
        let component = Component::from_file(&engine, path).context("Component file not found")?;
        let mut store = Store::new(&engine, MyState::default());
        let mut linker = Linker::new(&engine);

        // Need to link the host-provided Rng type
        bindings::MyWorld::add_to_linker::<_, Rng>(&mut linker, |state| state)?;

        // Finally, instantiate the component
        let instance = bindings::MyWorld::instantiate(&mut store, &component, &linker)
            .context("Failed to instantiate the example world")?;

        Ok(Self {
            world: instance,
            store,
        })
    }

    fn new_greeter(&mut self) -> wasmtime::Result<ResourceAny> {
        let rng = self
            .store
            .data_mut()
            .table
            .push(Rng::default())
            .context("creating rng resource")?;

        self.world
            .ochagavia_test_guest_types()
            .greeter()
            .call_constructor(&mut self.store, rng)
            .context("calling greeter constructor")
    }

    fn greet(&mut self, greeter: ResourceAny) -> wasmtime::Result<String> {
        self.world
            .ochagavia_test_guest_types()
            .greeter()
            .call_greet(&mut self.store, greeter)
            .context("calling greet function")
    }
}
