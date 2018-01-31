extern crate sdl2;
extern crate vulkano;
extern crate vulkano_sdl2;

use vulkano_sdl2::VkSurfaceBuild;

#[test]
fn create_surface() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Vulkan", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let instance = {
        let extensions = window.required_vk_instance_extensions().unwrap();
        println!("{:?}", extensions);
        vulkano::instance::Instance::new(None, &extensions, None)
            .expect("failed to create instance")
    };

    let _surface = window.build_vk_surface(instance).unwrap();

    let _dimensions = VkSurfaceBuild::drawable_size(&window);
}
