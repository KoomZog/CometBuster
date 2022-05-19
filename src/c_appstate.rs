#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    SetupMaterials,
    SpawnStart,
    InGame,
    Paused,
}