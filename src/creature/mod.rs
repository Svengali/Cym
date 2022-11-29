use bevy::prelude::{*};

use bevy_sprite::{TextureAtlasSprite, TextureAtlas};











#[derive(Bundle, Clone, Default)]
pub struct CreatureDisplayBundle {
    /// The specific sprite from the texture atlas to be drawn
    pub sprite: TextureAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,

    pub transform: Transform,

    pub global_transform: GlobalTransform,
}


#[derive(Bundle, Clone, Default)]
pub struct BrainNeuronBundle {
}

#[derive(Bundle, Clone, Default)]
pub struct StateMachineNeuronBundle {
}




#[derive(Bundle, Clone, Default)]
pub struct CreatureNeuronBundle {
    display: CreatureDisplayBundle,

    brain: BrainNeuronBundle,

}


