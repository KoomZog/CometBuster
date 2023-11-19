use bevy::ecs::schedule::States;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    SetupMaterials,
    SpawnStart,
    InGame,
    Paused,
}