use super::*;

// use std::{ collections::HashMap, };

mod camerabuilder; pub use camerabuilder::CameraBuilder;
mod mousecamera; use mousecamera::{MouseCamera, MouseCameraBuilder};
mod wheelcamera; use wheelcamera::WheelCameraBuilder;
mod touchcamera; use touchcamera::{TouchCamera, TouchCameraBuilder};

/// Object to represent a camera
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone)]
pub struct Camera {
  width:  f32,
  height: f32,
  fov:    f32,
  view:   nalgebra::Matrix4<f32>,
  distance: f32,
}

impl Camera {
  /// Create a new camera applying a view modifier
  fn modify_view(&self, modifier: nalgebra::Matrix4<f32>) -> Camera {
    let mut c = self.clone(); c.view = modifier * c.view;
    c
  }

  /// Create a new camera by applying an orbit transformation
  fn orbit(&self, from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> Camera {
    self.modify_view(
      nalgebra::Matrix4::<f32>::from_euler_angles(
        (from_y - to_y) / self.height * self.fov,
        (from_x - to_x) / self.height * self.fov,
        0.0,
      )
    )
  }

  /// Create a new camera by applying a zoom `delta_y` at location `x,y`
  fn zoom(&self, x: f32, y: f32, delta: f32) -> Camera {
    let theta_x = ( x - 0.5*self.width)  / self.height * self.fov;
    let theta_y = ( y - 0.5*self.height) / self.height * self.fov;
    let translation = nalgebra::Vector3::<f32>::new(0.0, 0.0, delta / self.height * 1.0 * self.distance);
    let rotation = nalgebra::Rotation3::<f32>::from_euler_angles(-theta_y, -theta_x, 0.0);
    self.modify_view(nalgebra::Matrix4::<f32>::from(nalgebra::Translation3::<f32>::from( rotation * translation)))
  }

  /// Extract view matrix combined with projection matrix
  fn matrix4(&self) -> Result<nalgebra::Matrix4<f32>, Error> {
    Ok(self.projection_matrix4()? * self.view)
  }

  /// Extract projection matrix
  fn projection_matrix4(&self) -> Result<nalgebra::Matrix4<f32>, Error> {
    Ok(nalgebra::Matrix4::<f32>::new_perspective(
          self.width / self.height , self.fov,
          0.1f32, 200f32))
  }
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl Camera {
  /// Update the camera `width`
  pub fn width(mut self, width: f32) -> Camera { self.width = width; self }

  /// Update the camera `height`
  pub fn height(mut self, height: f32) -> Camera { self.height = height; self }

  /// Convert the camera to a 4x4 view-projection matrix
  pub fn as_matrix(&self) -> Result<Vec<f32>, JsError> {
    Ok(self.matrix4()?.as_slice().iter().cloned().collect())
  }

  /// Convert the camera to a 4x4 view matrix
  pub fn as_view_matrix(&self) -> Result<Vec<f32>, JsError> {
    Ok(self.view.as_slice().iter().cloned().collect())
  }

  /// Convert the camera to a 4x4 projection matrix
  pub fn as_projection_matrix(&self) -> Result<Vec<f32>, JsError> {
    Ok(self.projection_matrix4()?.as_slice().iter().cloned().collect())
  }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl Camera {
  /// Handle `mousedown` event
  pub fn on_mouse_down(self, event: web_sys::MouseEvent) -> Result<MouseCamera, JsError> {
    Ok(MouseCameraBuilder::default()
       .camera(self)
       .mouse_down(event.clone())
       .mouse_move(event)
       .build()?)
  }

  /// Handle `mousemove` event
  pub fn on_mouse_move(self, _event: web_sys::MouseEvent) -> Result<Camera, JsError> { Ok(self) }

  /// Handle `mouseup` event
  pub fn on_mouse_up(self, _event: web_sys::MouseEvent) -> Result<Camera, JsError> { Ok(self) }

  /// Handle `mousewheel` event
  pub fn on_wheel(self, event: web_sys::WheelEvent) -> Result<Camera, JsError> {
    WheelCameraBuilder::default()
    .camera(self)
    .build()?
    .on_wheel(event)
  }

  /// Handle touch events: `touchstart`, `touchend`, `touchcancel`, `touchmove`
  pub fn on_touch(self, event: web_sys::TouchEvent) -> Result<TouchCamera, JsError> {
    TouchCameraBuilder::default()
    .camera(self)
    .build()?
    .on_touch(event)
  }
}


/*
/* Legacy */
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Camera {
    width:    Option<f32>,
    height:   Option<f32>,
    fov:      f32,
    view:     nalgebra::Matrix4<f32>,
    modifier: nalgebra::Matrix4<f32>,
    distance: f32,

    mouse_down: Option<web_sys::MouseEvent>,
    touches_down: HashMap<i32, web_sys::Touch>,
    touches:    HashMap<i32, web_sys::Touch>,
}

impl Camera {
    pub fn width(&self) -> &Option<f32> { &self.width }
    pub fn height(&self) -> &Option<f32> { &self.height }
}

pub struct ControllerSignal {
    pub dtms: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub forward: f32,
    pub lift: f32,
    pub weight: f32,
}

impl Camera {
    pub fn on_controller_signal(&mut self, signal: ControllerSignal) -> Result<(), Box<dyn std::error::Error>> {
        let d = signal.dtms / 500.0 * self.distance;
        let t = nalgebra::Vector3::<f32>::new(0.0, 0.0, signal.forward * d)
            + nalgebra::Vector3::<f32>::new(0.0, -signal.lift * d, 0.0)
            + self.view.transform_vector(&nalgebra::Vector3::<f32>::new(0.0, signal.weight*d, 0.0));

        let f = signal.dtms / 500.0 * self.fov;
        self.view = nalgebra::Matrix4::<f32>::from_euler_angles(f * signal.pitch, f * signal.yaw, f * signal.roll)
            * nalgebra::Matrix4::<f32>::from(nalgebra::Translation3::<f32>::from(t))
            * self.view;
        Ok(())
    }
}

impl Camera {
    pub fn on_mouse_down(&mut self, event: web_sys::MouseEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.mouse_down = Some(event);
        Ok(())
    }
    pub fn on_mouse_move(&mut self, to: web_sys::MouseEvent) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(from) = self.mouse_down.as_ref() {
            self.modifier = self.orbit_matrix4(from.client_x() as f32, from.client_y() as f32, to.client_x() as f32, to.client_y() as f32);
        }
        Ok(())
    }
    pub fn on_mouse_up(&mut self, to: web_sys::MouseEvent) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(from) = self.mouse_down.as_ref() {
            self.view = self.orbit_matrix4(from.client_x() as f32, from.client_y() as f32, to.client_x() as f32, to.client_y() as f32)
                * self.view;
            self.modifier = nalgebra::Matrix4::<f32>::identity();
        }
        self.mouse_down = None;
        Ok(())
    }

    fn orbit_matrix4(&self, from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> nalgebra::Matrix4::<f32> {
        let mut r = nalgebra::Matrix4::<f32>::identity();
        if let Some(height) = self.height {
            let theta_x = (from_x - to_x) / height * self.fov;
            let theta_y = (from_y - to_y) / height * self.fov;
            r = nalgebra::Matrix4::<f32>::from_euler_angles(theta_y, theta_x, 0.0)
        }
        r
    }
}

impl Camera {
    pub fn on_wheel(&mut self, event: web_sys::WheelEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.view = self.zoom_matrix4(event.client_x() as f32, event.client_y() as f32, event.delta_y() as f32) * self.view;
        Ok(())
    }

    fn zoom_matrix4(&self, x: f32, y: f32, delta: f32) -> nalgebra::Matrix4::<f32> {
        let mut r = nalgebra::Matrix4::<f32>::identity();
        if let Some(width) = self.width {
            if let Some(height) = self.height {
                let theta_x = ( x - 0.5*width)  / height * self.fov;
                let theta_y = ( y - 0.5*height) / height * self.fov;
                let translation = nalgebra::Vector3::<f32>::new(0.0, 0.0, delta / height * 1.0 * self.distance);
                let rotation = nalgebra::Rotation3::<f32>::from_euler_angles(-theta_y, -theta_x, 0.0);
                r = nalgebra::Matrix4::<f32>::from(nalgebra::Translation3::<f32>::from( rotation * translation))
            }
        }
        r
    }
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl Camera {
    pub fn new() -> Camera {
        let eye    = nalgebra::Point3::new(0.0, 5.0, 0.0);
        let target = nalgebra::Point3::new(0.0, 0.0, 0.0);
        let up     = nalgebra::Vector3::new(0.0, 0.0, 1.0);
        let fov    = 45.0 * std::f32::consts::PI / 180f32;

        let view: nalgebra::Matrix4<f32> = nalgebra::Isometry3::<f32>::look_at_rh(&eye, &target, &up).to_homogeneous();
        let modifier = nalgebra::Matrix4::<f32>::identity();

        Camera { 
            width: None, 
            height: None,
            fov, 
            view, 
            modifier, 
            distance: nalgebra::distance(&eye, &target), 
            mouse_down: None,
            touches_down: HashMap::new(),
            touches: HashMap::new(),
        }
    }

    pub fn set(&mut self, width: f32, height: f32) {
        self.width = Some(width);
        self.height = Some(height);
        self.view = self.modifier * self.view;
        self.modifier = nalgebra::Matrix4::<f32>::identity();
        self.mouse_down = None;
        self.touches = HashMap::new();
        self.touches_down = HashMap::new();
    }
}

impl Camera {
    /*
    fn changed_touches<F>(touch_list: TouchList, f: F) -> Result<(), Box<dyn std::error::Error>> 
    {
    }
    */
    pub fn on_touch_event(&mut self, event: web_sys::TouchEvent) -> Result<(), Box<dyn std::error::Error>> {
        event.prevent_default();
        match event.type_().as_str() {
            "touchstart" => {
                if ! self.touches_down.is_empty() {
                    self.touch_modify()?; self.touch_apply()?;
                }
                let touch_list = event.changed_touches();
                for i in 0..touch_list.length() {
                    if let Some(touch) = touch_list.get(i) {
                        self.touches_down.insert(touch.identifier(), touch.clone());
                        self.touches.insert(touch.identifier(), touch.clone());
                    }
                }
                Ok(())
            },
            "touchmove" => {
                let touch_list = event.changed_touches();
                for i in 0..touch_list.length() {
                    if let Some(touch) = touch_list.get(i) {
                        self.touches.insert(touch.identifier(), touch);
                    }
                }
                if ! self.touches_down.is_empty() {
                    self.touch_modify()?;
                }
                Ok(())
            },
            "touchend" | "touchcancel" => {
                if ! self.touches_down.is_empty() {
                    self.touch_modify()?; self.touch_apply()?;
                }
                let touch_list = event.changed_touches();
                for i in 0..touch_list.length() {
                    if let Some(touch) = touch_list.get(i) {
                        self.touches_down.remove(&touch.identifier());
                        self.touches.remove(&touch.identifier());
                    }
                }
                Ok(())
            },
            _ => Err(format!("Event type {} is not supported", event.type_()).into()),
        }
    }

    fn touch_mid(touches: &HashMap<i32, web_sys::Touch>) -> (f32, f32) {
        let alpha: f32 = 1f32 / if touches.len() > 0 { touches.len() as f32 } else { 1f32 };
        touches.iter().fold((0f32, 0f32), |r, (_, t)| (r.0 + alpha*t.client_x() as f32, r.1 + alpha*t.client_y() as f32))
    }

    fn touch_delta(touches: &HashMap<i32, web_sys::Touch>, mid: &(f32, f32)) -> f32 {
        touches.iter()
            .fold(0f32, |r, (_, t)| { r + ((t.client_x() as f32 - mid.0).powi(2) + (t.client_y() as f32 - mid.1).powi(2)).sqrt() })
    }

    fn alpha(a: &web_sys::Touch, b: &web_sys::Touch) -> f32 {
        let l = ((b.client_x() as f32 - a.client_x() as f32).powi(2) + (b.client_y() as f32 - a.client_y() as f32).powi(2)).sqrt();
        if l > 1e-5 {
            if b.client_y() > a.client_y() {
                1f32 * ((b.client_x() as f32 - a.client_x() as f32)/l).acos()
            } else {
                -1f32 * ((b.client_x() as f32 - a.client_x() as f32)/l).acos()
            }
        } else {
            0f32
        }
    }

    fn touch_alpha(touches_from: &HashMap<i32, web_sys::Touch>, touches_to: &HashMap<i32, web_sys::Touch>) -> f32 {
        let mut reference: Option<(&web_sys::Touch, &web_sys::Touch)> = None;
        let mut alpha = 0f32; let mut count = 0;
        for (k, v_from) in touches_from.iter() {
            if let Some(v_to) = touches_to.get(k) {
                if reference.is_none() { 
                    reference = Some((v_from, v_to)); 
                } else {
                    count += 1;
                    alpha += Camera::alpha(reference.unwrap().1, v_to) - Camera::alpha(reference.unwrap().0, v_from);
                }
            }
        }
        alpha / if count > 0 { count as f32 } else { 1f32 }
    }

    fn touch_modify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (fr_x, fr_y) = Camera::touch_mid(&self.touches_down);
        let fr_l         = Camera::touch_delta(&self.touches_down, &(fr_x, fr_y));

        let (to_x, to_y) = Camera::touch_mid(&self.touches);
        let to_l         = Camera::touch_delta(&self.touches, &(to_x, to_y));
        let alpha        = Camera::touch_alpha(&self.touches_down, &self.touches);

        self.modifier = 
            self.zoom_matrix4(to_x, to_y, to_l - fr_l)
            * nalgebra::Matrix4::<f32>::from_euler_angles(0f32, 0f32, -alpha)
            * self.orbit_matrix4(fr_x, fr_y, to_x, to_y);

        Ok(())
    }

    fn touch_apply(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.view = self.modifier * self.view;
        self.modifier = nalgebra::Matrix4::<f32>::identity();
        self.touches_down = self.touches.clone();
        Ok(())
    }
}

impl Camera {
    pub fn to_matrix4(&self) -> Result<nalgebra::Matrix4<f32>, Box<dyn std::error::Error>> {
        let r = self.to_projection_matrix4()? * self.to_view_matrix4()?;
        Ok(r)
    }

    pub fn to_view_matrix4(&self) -> Result<nalgebra::Matrix4<f32>, Box<dyn std::error::Error>> {
        Ok(self.modifier * self.view)
    }

    pub fn to_projection_matrix4(&self) -> Result<nalgebra::Matrix4<f32>, Box<dyn std::error::Error>> {
        let mut r = nalgebra::Matrix4::<f32>::identity();
        if let Some(width) = self.width {
            if let Some(height) = self.height {
                r = nalgebra::Matrix4::<f32>::new_perspective(
                    width / height , self.fov,
                    0.1f32, 200f32);
            }
        }
        Ok(r)
    }
}
*/
