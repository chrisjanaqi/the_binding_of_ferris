use bevy::prelude::*;

pub struct Materials {
    pub player: Handle<TextureAtlas>,
    pub tears: Handle<TextureAtlas>,
    pub ground: Handle<ColorMaterial>,
}
