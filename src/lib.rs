#![allow(dead_code)]

use std::{rc::Rc, time::{Duration}};
use glam::{Mat4, Vec3};
use winit::{window::Window, dpi::PhysicalSize, event::{WindowEvent, Event}};
use world::World;
use renderer::Renderer;

mod imgui;
mod camera;
mod world;
mod renderer;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct BoidInstance {
    mvp: Mat4,
}

pub struct App {
    world: world::World,
    renderer: renderer::Renderer,
    instance_data: Vec<BoidInstance>,
}

impl App {
    pub async fn new(window: Rc<Window>) -> Self {
        Self {
            world: World::new(0.0, 12),
            renderer: Renderer::new(window).await,
            instance_data: Vec::with_capacity(50),
        }
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size)
    }

    pub fn input(&mut self, win_event: &WindowEvent, event: &Event<()>) -> bool {
        self.renderer.input(win_event, event)
    }

    pub fn render(&mut self, delta_t: Duration) {
        let cur_cam = self.renderer.camera();
        self.world.fill_instance_buffer(&mut self.instance_data, cur_cam.view_mat(), cur_cam.perspective_mat());
        self.renderer.fill_instance_buffer(&self.instance_data);
        self.renderer.render(delta_t).expect("rendering failed somehow");
    }

    pub fn add_boid(&mut self, pos: Vec3) {
        self.world.add_boid(pos);
    }
}
