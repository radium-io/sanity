pub mod gameover;
pub mod intro;
pub mod loading;
pub mod room;

pub use gameover::GameOverState;
pub use intro::IntroState;
pub use loading::LoadingState;
pub use room::RoomState;

#[derive(Default)]
pub struct Sanity {
    pub game_over: bool,
}
