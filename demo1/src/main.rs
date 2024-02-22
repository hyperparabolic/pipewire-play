use pipewire::{context::Context, main_loop::MainLoop};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mainloop = MainLoop::new(None)?;
    let context = Context::new(&mainloop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let _listener = registry
        .add_listener_local()
        .global(|global| println!("New global: {:?}", global))
        .register();

    mainloop.run();

    Ok(())
}
