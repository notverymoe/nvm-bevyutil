/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use bevy::{
    ecs::{
        component::Component,
        system::{Query, Res}
    },
    app::Plugin,
    render::camera::{
        Camera, 
        OrthographicProjection, 
        ScalingMode
    }, 
    window::Windows
};

#[derive(Default, Clone, Copy)]
pub struct OrthographicCameraScalerPlugin;

impl Plugin for OrthographicCameraScalerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(orthographic_camera_scaler);
    }
}

#[derive(Component)]
pub enum OrthographicScaleMode {
    None,
    Width(f32),
    Height(f32),
    Contain(f32, f32),
    Cover(f32, f32),
}

pub fn orthographic_camera_scaler(
    windows: Res<Windows>,
    mut query: Query<(&OrthographicScaleMode, &Camera, &mut OrthographicProjection)>
) {
    for (scale, camera, mut projection,) in query.iter_mut() {
        let a = {
            let window = windows.get(camera.window).unwrap();
            window.width()/window.height()
        };

        let (w, h, contain) = match *scale {
            OrthographicScaleMode::None          => continue,
            OrthographicScaleMode::Width(w)      => { (w,   w/a, true ) },
            OrthographicScaleMode::Height(h)     => { (h*a, h,   true ) },
            OrthographicScaleMode::Contain(w, h) => { (w,   h,   true ) },
            OrthographicScaleMode::Cover(w, h)   => { (w,   h,   false) },
        };

        let target_width = h*a;
        let (w, h) = if (target_width >= w) == contain { (target_width, h) } else { (w, w/a) };

        projection.scaling_mode = ScalingMode::None;
        projection.left   = -0.5*w;
        projection.right  =  0.5*w;
        projection.top    =  0.5*h;
        projection.bottom = -0.5*h;
    }
}
