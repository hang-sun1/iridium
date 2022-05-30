use std::time::{Duration, Instant};

use glam::{Vec3, Quat, Vec4, Mat4};

#[derive(Clone, Copy)]
pub(crate) struct Movement {
    pub(crate) strafe_forward: bool,
    pub(crate) strafe_back: bool,
    pub(crate) strafe_left: bool,
    pub(crate) strafe_right: bool,
    pub(crate) pitch_up: bool,
    pub(crate) pitch_down: bool,
    pub(crate) yaw_left: bool,
    pub(crate) yaw_right: bool,
}

pub(crate) struct Camera {
    fov: f32,
    near: f32,
    far: f32,
    aspect_ratio: f32,
    rotation: Vec3,
    cam_position: Vec3,
    cam_dest: Vec3,
    looking_at: Vec3,
    view_mat: Mat4,
    perspective_mat: Mat4,
}

impl Camera {
    pub(crate) fn new(camera_pos: Vec3, camera_target: Vec3, fov: f32, near: f32, far: f32,
        aspect_ratio: f32) -> Self {
        
        let perspective_mat = Mat4::perspective_lh(fov.to_radians(), aspect_ratio, near, far);
        let view_mat = Self::view_matrix(Vec3::ZERO, camera_pos);

        Self {
            fov,
            near,
            far,
            aspect_ratio,
            rotation: Vec3::ZERO,
            cam_position: camera_pos,
            cam_dest: camera_pos,
            looking_at: camera_target,
            view_mat,
            perspective_mat,
        }
    }

    fn view_matrix(rot: Vec3, camera_pos: Vec3) -> Mat4 {
        let x_rot = Mat4::from_rotation_x(rot.x);
        let y_rot = Mat4::from_rotation_y(rot.y);
        let z_rot = Mat4::from_rotation_z(rot.z);

        let rot = z_rot * y_rot * x_rot;
        let trans = Mat4::from_translation(camera_pos);

        rot * trans
    }

    pub(crate) fn model_view_proj(&self, model: Mat4) -> Mat4 {
        self.perspective_mat * self.view_mat * model
    }

    pub(crate) fn add_movement(&mut self, movements: Movement) {
        if movements.any_movement() {
            let cam_dir = Vec3::new(
                -self.rotation.x.cos() * self.rotation.y.sin(),
                self.rotation.x.sin(),
                self.rotation.x.cos() * self.rotation.y.sin()
            ).normalize_or_zero();
            let cam_dir = (self.looking_at - self.cam_position).normalize();
            // let speed = delta_t.as_micros() as f32 / 100000.0 * 0.19;
            let speed = 0.018;
            
            if movements.strafe_forward {
                self.cam_dest += speed * cam_dir;
            }
            if movements.strafe_back {
                self.cam_dest -= speed * cam_dir;
            }
            if movements.strafe_left {
                self.cam_dest += cam_dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalize() * speed;
            }
            if movements.strafe_right {
                self.cam_dest -= cam_dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalize() * speed;
            }
        }
    }

    pub(crate) fn update(&mut self, delta_t: Duration) {
        let s = delta_t.as_secs_f32() / 0.00833333;
        self.cam_position = self.cam_position.lerp(self.cam_dest, s);
        self.view_mat = Self::view_matrix(self.rotation, self.cam_position);
    }

    pub(crate) fn perspective_mat(&self) -> Mat4 {
        self.perspective_mat
    }

    pub(crate) fn view_mat(&self) -> Mat4 {
        self.view_mat
    }
}

impl Movement {
    fn any_movement(&self) -> bool {
        self.strafe_forward ||
        self.strafe_back ||
        self.strafe_left || 
        self.strafe_right ||
        self.pitch_up ||
        self.pitch_down ||
        self.yaw_left ||
        self.yaw_right
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            strafe_forward: false,
            strafe_back: false,
            strafe_left: false,
            strafe_right: false,
            pitch_up: false,
            pitch_down: false,
            yaw_left: false,
            yaw_right: false,
        }
    }
}