use speedy2d::window::VirtualKeyCode;

pub struct KeySystem {
    pub pressed_key: Vec<VirtualKeyCode>
}

impl KeySystem {
   pub fn new() -> Self {
       Self { pressed_key: vec![] }
   } 

    pub fn on_key_pressed(&mut self, key: VirtualKeyCode) {
        if !self.pressed_key.contains(&key) {
            self.pressed_key.push(key)
        }
    }

    pub fn on_key_released(&mut self, key: VirtualKeyCode) {
        if self.pressed_key.contains(&key) {
            let index = self.pressed_key
                .iter()
                .position(|x| *x == key)
                .unwrap();
            self.pressed_key.remove(index);
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_key.contains(&key)
    }

}
