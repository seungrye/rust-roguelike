use std::cmp;

use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator};

use super::Rect;

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
}

impl Map {
    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }
    /// 파라메터로 받은 x, y 좌표가 지도 영엳을 벗어나지 않고, 지나다닐 수 있는 위치인지 확인함.
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.width * self.height) as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            width: 80,
            height: 50,
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50],
            blocked: vec![false; 80 * 50],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> rltk::prelude::Point {
        rltk::prelude::Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(_idx1 % w, _idx1 / w);
        let p2 = Point::new(_idx2 % w, _idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exists = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exists.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exists.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exists.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exists.push((idx + w, 1.0))
        };

        exists
    }

    fn is_opaque(&self, _idx: usize) -> bool {
        self.tiles[_idx] == TileType::Wall
    }
}
