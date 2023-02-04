/// This represents a RGB pattern
pub enum RGBPattern {
    /// Simple static RGB color.
    Static { color: (u8, u8, u8) },
    /// Blinking color.
    /// You can specify the speed with `BlinkSpeed`
    Blink {
        color: (u8, u8, u8),
        speed: BlinkSpeed,
    },

    /// Custom pattern.
    /// You have 32 possible different colors that can cycle or not.
    Custom {
        colors: [(u8, u8, u8); 32],
        repeat: bool,
    },
}

/// This represents standard blinking speeds
pub enum BlinkSpeed {
    VerySlow, // 2 per iteration
    Slow,     // 4 per iteration
    Medium,   // 8 per iteration
    Fast,     // 16 per iteration
    VeryFast, // 32 per iteration
}

pub fn led_set(pattern: RGBPattern) {
    let c: [(u8, u8, u8); 32];
    let cycle: bool;

    match pattern {
        RGBPattern::Static { color } => {
            c.fill(color);
            cycle = true;
        }
        RGBPattern::Blink { color, speed } => {
            cycle = true;
            match speed {
                BlinkSpeed::VerySlow => {
                    c = repeat_pattern(2, &color);
                }
                BlinkSpeed::Slow => {
                    c = repeat_pattern(4, &color);
                }
                BlinkSpeed::Medium => {
                    c = repeat_pattern(8, &color);
                }
                BlinkSpeed::Fast => {
                    c = repeat_pattern(16, &color);
                }
                BlinkSpeed::VeryFast => {
                    c = repeat_pattern(32, &color);
                }
            }
        }
        RGBPattern::Custom { colors, repeat } => {
            c = colors;
            cycle = repeat;
        }
    }

    led_send(&c, cycle);

}

fn repeat_pattern(n: usize, pattern: &[(u8, u8, u8)]) -> [(u8, u8, u8); 32] {
    let mut result = [(0, 0, 0); 32];
    let mut index = 0;
    for _ in 0..n {
        for &(r, g, b) in pattern {
            result[index] = (r, g, b);
            index += 1;
        }
    }
    result
}

fn led_send(pattern: &[(u8, u8, u8); 32], repeat: bool) {
    //TODO: send pattern
}
