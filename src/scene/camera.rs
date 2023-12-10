use crate::render::buffer::Buffer;
use cgmath::{Matrix4, Point3, Vector3, Deg, perspective, SquareMatrix};


use winit::{
    event::*,
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct CameraDescriptor {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct CameraLayout {
    pub bind_group_layout: wgpu::BindGroupLayout,
}


impl CameraLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            bind_group_layout: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("camera_bind_group_layout"),
            })
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, params: &CameraDescriptor) {
        let view = Matrix4::look_at_rh(params.eye, params.target, params.up);
        let proj = perspective(Deg(params.fovy), params.aspect, params.znear, params.zfar);
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * proj * view).into();
    }
}

pub struct Camera {
    pub params: CameraDescriptor,
    pub uniform: CameraUniform,
    pub ubuf: Option<Buffer<CameraUniform>>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            params: CameraDescriptor {
                eye: Point3::new(0.0, 0.0, 1.0),
                target: Point3::new(0.0, 0.0, 0.0),
                up: Vector3::unit_y(),
                aspect: 16.0 / 9.0,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
            uniform: CameraUniform::new(),
            ubuf: None,
        }
    }
}

impl Camera {
    pub fn new(device: &wgpu::Device, eye: Point3<f32>, target: Point3<f32>, aspect: f32) -> Self {
        let default_camera = Camera::default();
        let params = CameraDescriptor {
            eye,
            target,
            up: default_camera.params.up,
            aspect,
            fovy: default_camera.params.fovy,
            znear: default_camera.params.znear,
            zfar: default_camera.params.zfar,
        };
        let mut camera_uniform = default_camera.uniform;
        camera_uniform.update_view_proj(&params);

        let ubuf = Buffer::new(device, wgpu::BufferUsages::UNIFORM, &[camera_uniform]);

        Camera {
            params,
            uniform: camera_uniform,
            ubuf: Some(ubuf),
        }
    }


}


pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.params.target - camera.params.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.params.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.params.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.params.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.params.target - camera.params.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so 
            // that it doesn't change. The eye, therefore, still 
            // lies on the circle made by the target and eye.
            camera.params.eye = camera.params.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.params.eye = camera.params.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
