pub mod gameover;
pub mod loading;
pub mod room;

pub use gameover::GameOverState;
pub use loading::LoadingState;
pub use room::RoomState;

#[derive(Default)]
pub struct Sanity {
    pub game_over: bool,
}
