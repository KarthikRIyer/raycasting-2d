use kiss3d::event::WindowEvent;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::window::Window;
use nalgebra::{Point2, Point3, Vector2};

struct Boundary {
    a: Point2<f32>,
    b: Point2<f32>,
}

impl Boundary {
    fn draw(&self, window: &mut Window) {
        window.draw_planar_line(&self.a, &self.b, &Point3::new(1.0, 1.0, 1.0));
    }
}

struct Ray {
    pos: Point2<f32>,
    dir: Vector2<f32>,
}

impl Ray {
    fn new(pos: Point2<f32>, angle: f32) -> Ray {
        Ray {
            pos,
            dir: Vector2::new(angle.to_radians().cos(), angle.to_radians().sin()),
        }
    }

    fn draw(&self, window: &mut Window) {
        window.draw_planar_line(&self.pos, &(&self.pos + 10.0 * &self.dir), &Point3::new(1.0, 1.0, 1.0));
    }

    fn cast(&self, wall: &Boundary) -> Option<Point2<f32>> {
        let x1 = wall.a.x;
        let y1 = wall.a.y;
        let x2 = wall.b.x;
        let y2 = wall.b.y;

        let x3 = self.pos.x;
        let y3 = self.pos.y;
        let x4 = self.pos.x + self.dir.x;
        let y4 = self.pos.y + self.dir.y;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if den == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;
        if t > 0.0 && t < 1.0 && u > 0.0 {
            return Some(Point2::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1)));
        }
        return None;
    }
}

struct Particle {
    pos: Point2<f32>,
    rays: Vec<Ray>,
}

impl Particle {
    fn new() -> Particle {
        let mut rays: Vec<Ray> = Vec::new();
        for a in 0..360 {
            rays.push(Ray::new(Point2::new(0.0, 0.0), a as f32))
        }
        Particle {
            pos: Point2::new(0.0, 0.0),
            rays,
        }
    }

    fn distance(p1: &Point2<f32>, p2: &Point2<f32>) -> f32 {
        return ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
    }

    fn update(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;

        for ray in &mut self.rays {
            ray.pos.x = x;
            ray.pos.y = y;
        }
    }

    fn look(&self, walls: &Vec<Boundary>, window: &mut Window) {
        for ray in &self.rays {
            let mut closest: Option<Point2<f32>> = None;
            let mut record = 500000000.0;
            for wall in walls {
                let pt = ray.cast(wall);
                if pt.is_some() {
                    let d = Particle::distance(&self.pos, &pt.unwrap());
                    if d < record {
                        record = d;
                        closest = pt;
                    }
                }
            }
            if closest.is_some() {
                window.draw_planar_line(&self.pos, &closest.unwrap(), &Point3::new(1.0, 1.0, 1.0));
            }
        }
    }

    fn show(&self, window: &mut Window) {
        for ray in &self.rays {
            ray.draw(window)
        }
    }
}

fn main() {
    let mut window = Window::new("Raycasting");

    let mut camera = kiss3d::planar_camera::FixedView::new();

    let mut walls: Vec<Boundary> = Vec::new();
    walls.push(Boundary {
        a: Point2::new(300.0, 100.0),
        b: Point2::new(300.0, 300.0),
    });
    walls.push(Boundary {
        a: Point2::new(-300.0, -100.0),
        b: Point2::new(300.0, 300.0),
    });
    walls.push(Boundary {
        a: Point2::new(-300.0, 100.0),
        b: Point2::new(400.0, -250.0),
    });
    walls.push(Boundary {
        a: Point2::new(300.0, 80.0),
        b: Point2::new(400.0, -250.0),
    });

    let mut last_pos = Point2::new(0.0f32, 0.0f32);

    let mut particle = Particle::new();

    while window.render_with(None, Some(&mut camera), None) {

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::CursorPos(x, y, _) => {
                    let window_size = Vector2::new(window.size()[0] as f32, window.size()[1] as f32);
                    last_pos = Point2::new(x as f32, y as f32);
                    let sel_pos = camera.unproject(&last_pos, &window_size);
                    // println!("Cursor pos: ({} , {})", x, y);
                    particle.update(sel_pos.x, sel_pos.y as f32);
                }
                _ => {}
            }
        }

        for wall in &walls {
            wall.draw(&mut window);
        }
        particle.show(&mut window);
        particle.look(&walls, &mut window);
    }
}
