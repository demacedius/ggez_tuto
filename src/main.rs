use ggez::conf::WindowMode;
use ggez::{event, ContextBuilder};
use ggez_tuto::GameState;
fn main() {
    let (context, event_loop) =
        &mut match ContextBuilder::new("first_game_ggez", "de Macedo Anthony")
            .window_mode(WindowMode::default().dimensions(1080.0, 720.0))
            .build()
        {
            Ok((context, event_loop)) => (context, event_loop),
            Err(error) => panic!(error),
        };

    let game_state = &mut GameState::new();

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Merci d'avoir jouer"),
        Err(error) => println!("L'erreur est: {}", error),
    };
}
