mod game;
mod player;
mod keysystem;
mod enemy;

use std::ops::Mul;
use std::thread;
use std::time::Duration;

use game::Game;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{KeyScancode, ModifiersState, MouseButton, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper, WindowPosition, WindowSize, WindowStartupInfo};

const FPS: u32 = 60;
const FRAME_DURATION: u32 = 1000 / FPS; // ms

#[derive(Debug)]
enum GameEvents {
    Update,
    Render,
}

struct GameWindowHandler {
    pub game: Game,
    size_pixels: Vector2<u32>,
    mouse_position: Vector2<f32>,
}

impl WindowHandler<GameEvents> for GameWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper<GameEvents>, _info: WindowStartupInfo) { 
        let event_sender = helper.create_user_event_sender();
        helper.request_redraw();
        thread::spawn(move || {
            loop {
                event_sender.send_event(GameEvents::Update).unwrap();
                event_sender.send_event(GameEvents::Render).unwrap();
                thread::sleep(Duration::from_millis(FRAME_DURATION as u64));
            }
        });
    }
    fn on_user_event(&mut self, helper: &mut WindowHelper<GameEvents>, user_event: GameEvents) { 
        let ts = FRAME_DURATION;
        match user_event {
            GameEvents::Update => self.game.update(ts as f32),
            GameEvents::Render => helper.request_redraw()
        };
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<GameEvents>, size_pixels: Vector2<u32>) {
        self.size_pixels = size_pixels;
    }


    fn on_draw(&mut self, helper: &mut WindowHelper<GameEvents>, graphics: &mut Graphics2D) {
        graphics.draw_rectangle(
            Rectangle::new(Vector2::ZERO, self.size_pixels.mul(helper.get_scale_factor() as u32).into_f32()),
            Color::from_rgba(0.1, 0.1, 0.1, 1.)
        );
        // graphics.clear_screen(Color::from_rgba(0., 0., 0., 0.));
        self.game.render(graphics);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<GameEvents>, position: Vector2<f32>) {
        self.mouse_position = position;
        self.game.set_mouse_position(position);
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<GameEvents>, button: MouseButton) { }

    fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper<GameEvents>, button: MouseButton) { }

    fn on_key_down(&mut self, helper: &mut WindowHelper<GameEvents>, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        if let Some(key_code) = virtual_key_code {
            self.game.keysystem.on_key_pressed(key_code);
        }
    }

    fn on_key_up( &mut self, helper: &mut WindowHelper<GameEvents>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        if let Some(key_code) = virtual_key_code {
            self.game.keysystem.on_key_released(key_code);
        }
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<GameEvents>, unicode_codepoint: char) { 
        if unicode_codepoint == 'f' { self.game.add_random_enemy(); }
    }

    fn on_keyboard_modifiers_changed(&mut self, _helper: &mut WindowHelper<GameEvents>, state: ModifiersState) {} 
}


fn main() {
    println!("Game started");
    let window = Window::new_with_user_events(
        "Survivor",
        WindowCreationOptions::new_windowed(
            WindowSize::ScaledPixels((600., 400.).into()),
            Some(WindowPosition::Center)
        )
    ).unwrap();
    let game = Game::new();
    let window_handler = GameWindowHandler { 
        game, 
        size_pixels: Vector2::new(1200, 800),
        mouse_position: Vector2::ZERO, 
    };

    window.run_loop(window_handler);
}
