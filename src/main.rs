use piston_window::*;
use rand::*;

const HEIGHT: f64 = 720.0;
const WIDTH: f64 = 1280.0;

struct Bubble {
    speed: f64,
    x: f64,
    y: f64,
    r: f64,
}

impl Bubble {
    pub fn new(num: impl Into<Option<f64>>) -> Bubble {
        let r = (random::<f64>() * (WIDTH / 8.0)) + 5.0;

        let (speed, y) = if let Some(y) = num.into() {
            (0.0, y)
        } else {
            let speed = random::<f64>() * 90.0 + 10.0;
            let y = random::<f64>() * (HEIGHT + r);
            (speed, y)
        };

        Bubble {
            speed,
            x: random::<f64>() * WIDTH,
            y,
            r,
        }
    }
}

fn main() {
    let color_burbuja = [10.0, 87.0 / 255.0, 9.0, 1.0];

    let color_fondo = [221.0 / 255.0, 105.0 / 255.0, 49.0 / 255.0, 1.0];

    let mut bubble_vec: Vec<Bubble> = get_bubbles();

    let mut window: PistonWindow = WindowSettings::new("Lava Lamp", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = window.events;

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            let bubbs = &bubble_vec;

            window.draw_2d(&e, |c, g, _| {
                clear(color_fondo, g);

                for b in bubbs {
                    ellipse(
                        color_burbuja,
                        [b.x - b.r, b.y - b.r, b.r * 2.0, b.r * 2.0],
                        c.transform,
                        g,
                    )
                }
            });
        }
        if let Some(u) = e.update_args() {
            let bubbs = &mut bubble_vec;

            for burbuja in bubbs {
                burbuja.y = burbuja.speed * u.dt;
                if burbuja.y + burbuja.r <= 0.0 {
                    burbuja.y = HEIGHT + burbuja.r
                };
            }
        }
    }
}

fn get_bubbles() -> Vec<Bubble> {
    let mut bubbles = Vec::new();
    let n = (random::<u64>() % 15) + 10;

    for _ in 0..n {
        bubbles.push(Bubble::new(HEIGHT));
        bubbles.push(Bubble::new(0.0));
        bubbles.push(Bubble::new(None));
    }
    bubbles
}
