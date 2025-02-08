use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct DiplopodSegments(pub Vec<Entity>);

#[derive(Default, Resource)]
pub struct LastSpecialSpawn(pub u32);

#[derive(Resource)]
pub struct Sounds {
    pub eat_food: Handle<AudioSource>,
    pub eat_poison: Handle<AudioSource>,
    pub special_spawn: Handle<AudioSource>,
    pub super_food: Handle<AudioSource>,
    pub antidote: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct DefaultFontHandle(pub Handle<Font>);

#[derive(Default, Resource)]
pub struct Highscore(pub u16);

#[derive(Default, Resource)]
pub struct Lastscore(pub u16);
