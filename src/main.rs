mod materials;
mod shapes;
mod utils;

fn main() {
    let mut app = utils::app::create("Rust Renderer", "0.0.1");
    let mut profiler = utils::profiler::create();

    app.start();

    while app.is_running {
        profiler.start("Rendering");
        app.run();
        profiler.end("Rendering");
    }

    app.shutdown();
}
