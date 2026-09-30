//! Vulkan regression tests for the wgpu 30 renderer port.
use core::{Color, Font, Pixels, Point, Rectangle, Size, renderer::Quad};
use core::{Renderer as _, text::Renderer as _};
use iced_wgpu::{Engine, Renderer, core, graphics};

#[test]
#[ignore = "requires a Vulkan adapter; software Vulkan is supported"]
fn vulkan_quads_text_resize_and_msaa() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = pollster::block_on(
        instance.request_adapter(&wgpu::RequestAdapterOptions::default()),
    )
    .unwrap();
    eprintln!("Vulkan adapter: {:?}", adapter.get_info());
    let (device, queue) = pollster::block_on(
        adapter.request_device(&wgpu::DeviceDescriptor::default()),
    )
    .unwrap();
    for aa in [None, Some(graphics::Antialiasing::MSAAx4)] {
        let engine = Engine::new(
            &adapter,
            device.clone(),
            queue.clone(),
            wgpu::TextureFormat::Rgba8Unorm,
            aa,
            graphics::Shell::headless(),
        );
        let mut renderer =
            Renderer::new(engine, Font::with_name("Fira Sans"), Pixels(20.0));
        for (width, height, scale) in
            [(256, 96, 1.0), (512, 192, 2.0), (256, 96, 1.0)]
        {
            let bounds = Rectangle::with_size(Size::new(
                width as f32 / scale,
                height as f32 / scale,
            ));
            renderer.reset(bounds);
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle::new(
                        Point::new(8.0, 8.0),
                        Size::new(32.0, 32.0),
                    ),
                    ..Default::default()
                },
                Color::from_rgb(1.0, 0.0, 0.0),
            );
            renderer.fill_text(
                core::Text {
                    content: "Nastyfork café".into(),
                    bounds: bounds.size(),
                    size: Pixels(20.0),
                    line_height: core::text::LineHeight::default(),
                    font: Font::with_name("Fira Sans"),
                    align_x: core::text::Alignment::Left,
                    align_y: core::alignment::Vertical::Top,
                    shaping: core::text::Shaping::Advanced,
                    wrapping: core::text::Wrapping::None,
                },
                Point::new(55.0, 8.0),
                Color::WHITE,
                bounds,
            );
            #[cfg(feature = "geometry")]
            {
                use graphics::geometry::{Renderer as _, frame::Backend as _};
                let mut frame = iced_wgpu::geometry::Frame::new(bounds);
                frame.fill_rectangle(
                    Point::new(8.0, 55.0),
                    Size::new(32.0, 25.0),
                    Color::from_rgb(0.0, 1.0, 0.0),
                );
                renderer.draw_geometry(frame.into_geometry());
            }
            #[cfg(feature = "image")]
            let _image_allocation = {
                use core::image::{Handle, Image, Renderer as _};
                // An odd image width exercises row padding and atlas border uploads.
                let handle =
                    Handle::from_rgba(3, 5, [0u8, 0, 255, 255].repeat(15));
                let allocation = renderer.load_image(&handle).unwrap();
                renderer.draw_image(
                    Image::new(handle),
                    Rectangle::new(
                        Point::new(55.0, 55.0),
                        Size::new(24.0, 25.0),
                    ),
                    bounds,
                );
                allocation
            };
            #[cfg(feature = "svg")]
            {
                use core::svg::{Handle, Renderer as _, Svg};
                let handle=Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="25"><rect width="24" height="25" fill="yellow"/></svg>"#.as_slice());
                renderer.draw_svg(
                    Svg::new(handle),
                    Rectangle::new(
                        Point::new(90.0, 55.0),
                        Size::new(24.0, 25.0),
                    ),
                    bounds,
                );
            }
            let pixels = renderer.screenshot(
                &graphics::Viewport::with_physical_size(
                    Size::new(width, height),
                    scale,
                ),
                Color::TRANSPARENT,
            );
            let at = |x: f32, y: f32| {
                let i = (((y * scale) as usize) * width as usize
                    + (x * scale) as usize)
                    * 4;
                &pixels[i..i + 4]
            };
            assert_eq!(pixels.len(), (width * height * 4) as usize);
            assert_eq!(at(20.0, 20.0), [255, 0, 0, 255]);
            assert_eq!(at(250.0, 90.0), [0, 0, 0, 0]);
            assert!(
                (8..40)
                    .any(|y| (55..220).any(|x| at(x as f32, y as f32)[3] > 0)),
                "glyph pixels missing"
            );
            #[cfg(feature = "image")]
            assert_eq!(at(65.0, 65.0), [0, 0, 255, 255]);
            #[cfg(feature = "svg")]
            assert_eq!(at(100.0, 65.0), [255, 255, 0, 255]);
            #[cfg(feature = "geometry")]
            assert_eq!(at(20.0, 65.0), [0, 255, 0, 255]);
        }
    }
    let _ = device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
}
