use renderer::Renderer;
use std::path::PathBuf;
mod ray;
mod renderer;
mod vec3;

fn main() {
    let size = (400, 400);
    let mut renderer = Renderer::new(size);
    renderer.render();
    renderer.output(renderer::Format::Ppm, PathBuf::from("out.ppm"));
}
