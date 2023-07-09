use crate::{verline, HEIGHT, WIDTH};

pub const MAPHEIGHT: usize = 24;
pub const MAPWIDTH: usize = 24;

pub struct RayCaster {
    player: Player,
    map: [[usize; MAPWIDTH]; MAPHEIGHT],
    plane: Vector<f64>,
}

struct Player {
    pub pos: Vector<f64>,
    pub dir: Vector<f64>,
}

struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            player: Player {
                pos: Vector { x: 22.0, y: 12.0 },
                dir: Vector { x: -1.0, y: 0.0 },
            },

            map: [
                [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
                [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
                [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
              ],

            plane: Vector { x: 0.0, y: 0.66 },
        }
    }

    pub fn draw(&self, frame: &mut [u8]) -> Result<(), String> {
        let width;
        let height;
        unsafe {
            width = WIDTH;
            height = HEIGHT;
        }

        for x in 0..width {
            let cameraX = 2.0 * x as f64 / width as f64 - 1.0;
            let rayDir = Vector {
                x: self.player.dir.x + self.plane.x * cameraX,
                y: self.player.dir.y + self.plane.y * cameraX,
            };

            let mut mapPos = Vector {
                x: self.player.pos.x as i32,
                y: self.player.pos.y as i32,
            };

            let mut sideDist = Vector { x: 0., y: 0. };

            let deltaDist = Vector {
                x: f64::abs(1.0 / rayDir.x),
                y: f64::abs(1.0 / rayDir.y),
            };

            let perpWallDist: f64;
            let mut step = Vector { x: 0, y: 0 };

            let mut hit = false;
            let mut side = 0;
            if rayDir.x < 0. {
                step.x = -1;
                sideDist.x = (self.player.pos.x - mapPos.x as f64) * deltaDist.x;
            } else {
                step.x = 1;
                sideDist.x = (mapPos.x as f64 + 1.0 - self.player.pos.x) * deltaDist.x;
            }

            if rayDir.y < 0. {
                step.y = -1;
                sideDist.y = (self.player.pos.y - mapPos.y as f64) * deltaDist.y;
            } else {
                step.y = 1;
                sideDist.y = (mapPos.y as f64 + 1.0 - self.player.pos.y) * deltaDist.y;
            }
            // println!("Starting DDA");

            // Perform DDA
            while !hit {
                if sideDist.x < sideDist.y {
                    sideDist.x += deltaDist.x;
                    mapPos.x += step.x;
                    side = 0;
                } else {
                    sideDist.y += deltaDist.y;
                    mapPos.y += step.y;
                    side = 1;
                }

                if self.map[mapPos.x as usize][mapPos.y as usize] > 0 {
                    hit = true;
                }
            }
            // println!("Finished DDA");

            // Correct fisheye effect
            if side == 0 {
                perpWallDist =
                    (mapPos.x as f64 - self.player.pos.x + (1.0 - step.x as f64) / 2.0) / rayDir.x;
            } else {
                perpWallDist =
                    (mapPos.y as f64 - self.player.pos.y + (1.0 - step.y as f64) / 2.0) / rayDir.y;
            }

            let lineHeight = (height as f64 / perpWallDist) as i32;

            let mut drawStart = -lineHeight / 2 + height as i32 / 2;
            if drawStart < 0 {
                drawStart = 0;
            }
            let mut drawEnd = lineHeight / 2 + height as i32 / 2;
            if drawEnd >= height as i32 {
                drawEnd = height as i32 - 1;
            }
            // println!("Finished line height");

            let mut rgba: [u8; 4] = match self.map[mapPos.x as usize][mapPos.y as usize] {
                1 => [255, 0, 0, 255],
                2 => [0, 255, 0, 255],
                3 => [0, 0, 255, 255],
                4 => [255, 255, 255, 255],
                _ => [255, 255, 0, 255],
            };

            if side == 1 {
                rgba.div_assign(2);
            }

            verline(
                frame,
                x as usize,
                drawStart as usize,
                drawEnd as usize,
                &rgba,
                0.,
            );
            // return Ok(());
        }

        Ok(())
    }

    pub fn change_direction(&mut self, dir: Direction) {
        let moveSpeed = 0.2;
        let rotSpeed = 0.1;
        match dir {
            Direction::Up => {
                if self.map[(self.player.pos.x + self.player.dir.x * moveSpeed) as usize]
                    [(self.player.pos.y) as usize]
                    == 0
                {
                    self.player.pos.x += self.player.dir.x * moveSpeed;
                }

                if self.map[(self.player.pos.x) as usize]
                    [(self.player.pos.y + self.player.dir.y * moveSpeed) as usize]
                    == 0
                {
                    self.player.pos.y += self.player.dir.y * moveSpeed;
                }
            }

            Direction::Down => {
                if self.map[(self.player.pos.x - self.player.dir.x * moveSpeed) as usize]
                    [(self.player.pos.y) as usize]
                    == 0
                {
                    self.player.pos.x -= self.player.dir.x * moveSpeed;
                }

                if self.map[(self.player.pos.x) as usize]
                    [(self.player.pos.y - self.player.dir.y * moveSpeed) as usize]
                    == 0
                {
                    self.player.pos.y -= self.player.dir.y * moveSpeed;
                }
            }

            Direction::Right => {
                let oldDirX = self.player.dir.x;
                self.player.dir.x = self.player.dir.x * f64::cos(-rotSpeed)
                    - self.player.dir.y * f64::sin(-rotSpeed);
                self.player.dir.y =
                    oldDirX * f64::sin(-rotSpeed) + self.player.dir.y * f64::cos(-rotSpeed);

                let oldPlaneX = self.plane.x;
                self.plane.x =
                    self.plane.x * f64::cos(-rotSpeed) - self.plane.y * f64::sin(-rotSpeed);
                self.plane.y = oldPlaneX * f64::sin(-rotSpeed) + self.plane.y * f64::cos(-rotSpeed);
            }

            Direction::Left => {
                let oldDirX = self.player.dir.x;
                self.player.dir.x =
                    self.player.dir.x * f64::cos(rotSpeed) - self.player.dir.y * f64::sin(rotSpeed);
                self.player.dir.y =
                    oldDirX * f64::sin(rotSpeed) + self.player.dir.y * f64::cos(rotSpeed);

                let oldPlaneX = self.plane.x;
                self.plane.x =
                    self.plane.x * f64::cos(rotSpeed) - self.plane.y * f64::sin(rotSpeed);
                self.plane.y = oldPlaneX * f64::sin(rotSpeed) + self.plane.y * f64::cos(rotSpeed);
            }
        }
    }
}

trait DivAssign {
    fn div_assign(&mut self, rhs: u8);
}

impl DivAssign for [u8; 4] {
    fn div_assign(&mut self, rhs: u8) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;
    }
}
