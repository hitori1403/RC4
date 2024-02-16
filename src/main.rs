struct Rc4 {
    state: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    fn new(key: &[u8]) -> Self {
        let mut state = Self {
            state: [0; 256],
            i: 0,
            j: 0,
        };
        state.ksa(key);
        state
    }
    fn ksa(&mut self, key: &[u8]) {
        self.state.iter_mut().enumerate().for_each(|(i, x)| {
            *x = i as u8;
        });
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j
                .wrapping_add(self.state[i])
                .wrapping_add(key[i % key.len()]);
            self.state.swap(i, j.into());
        }
    }
    fn prga(&mut self) -> u8 {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.state[self.i as usize]);
        self.state.swap(self.i.into(), self.j.into());
        self.state[self.state[self.i as usize].wrapping_add(self.state[self.j as usize]) as usize]
    }
    pub fn apply(&mut self, input: &[u8]) -> Vec<u8> {
        input.iter().map(|x| x ^ self.prga()).collect()
    }
}

fn main() {
    let mut rc4 = Rc4::new(b"White Whistle");
    assert_eq!(
        rc4.apply(b"Made in Abyss"),
        [187, 159, 250, 115, 30, 113, 48, 212, 212, 69, 128, 99, 82]
    );
}
