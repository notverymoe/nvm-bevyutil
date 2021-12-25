/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use bevy::prelude::PluginGroup;

mod orthographic_camera_scaler;
pub use orthographic_camera_scaler::*;

#[derive(Default, Clone, Copy)]
pub struct CameraPlugins;

impl PluginGroup for CameraPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(OrthographicCameraScalerPlugin);
    }
}