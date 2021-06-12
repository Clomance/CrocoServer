mod authorization;
pub use authorization::AuthorizationActivity;

mod messenger;
pub use messenger::MessengerActivity;

#[derive(Clone,Copy,Debug)]
pub enum ActivityResult{
    /// Пользователь вышел из активности.
    /// 
    /// Он всё ещё в сети.
    Closed,
    /// Пользователь отключился либо был от отключён.
    Disconnected,
}