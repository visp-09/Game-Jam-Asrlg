use crate::prelude::*;
const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
use std::cmp::max;
use std::cmp::min;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Space,
    Floor,
}

pub struct Room {
    pub area: Rect, // To change as we mess with the design
                    // of rooms
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub player_start: Point,
    pub rooms: Vec<Room>,           // nodes
    pub paths: Vec<(usize, usize)>, // edges
    pub rng: RandomNumberGenerator,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}

pub fn map_point(p: Point) -> usize {
    ((p.y * MAP_WIDTH) + p.x) as usize
}

impl Map {
    pub fn new(rng: RandomNumberGenerator) -> Self {
        let mut m = Map {
            tiles: vec![TileType::Floor; NUM_TILES],
            player_start: Point::new(5, 5),
            rooms: Vec::new(),
            paths: Vec::new(),
            rng: rng,
        };
        m.fill(TileType::Space);
        m.draw_room(Rect::with_size(1, 1, 20, 20));
        m
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    fn draw_room(&mut self, room: Rect) {
        self.rooms.push(Room { area: room });
        room.for_each(|p| {
            if self.in_bounds(p) {
                let idx = map_idx(p.x, p.y);
                self.tiles[idx] = TileType::Floor;
            }
        });
        for x in room.x1 - 1..room.x2 + 1 {
            if self.in_bounds(Point {
                x: x,
                y: room.y1 - 1,
            }) {
                let idx = map_idx(x, room.y1 - 1);
                self.tiles[idx] = TileType::Wall;
            }
        }
        for x in room.x1 - 1..room.x2 + 1 {
            if self.in_bounds(Point { x: x, y: room.y2 }) {
                let idx = map_idx(x, room.y2);
                self.tiles[idx] = TileType::Wall;
            }
        }
        for y in room.y1..room.y2 {
            if self.in_bounds(Point {
                x: room.x1 - 1,
                y: y,
            }) {
                let idx = map_idx(room.x1 - 1, y);
                self.tiles[idx] = TileType::Wall;
            }
        }
        for y in room.y1..room.y2 {
            if self.in_bounds(Point { x: room.x2, y: y }) {
                let idx = map_idx(room.x2, y);
                self.tiles[idx] = TileType::Wall;
            }
        }
    }

    fn fill(&mut self, tile: TileType) {
        self.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn add_coridor(&mut self, r1: usize, r2: usize) {
        self.paths.push((r1, r2));
    }

    fn draw_coridor(&mut self, (r1, r2): (usize, usize)) {
        if r1 == r2 {
            return;
        }

        let p1 = self.rooms[r1].area.center();
        let p2 = self.rooms[r2].area.center();
        let dx = (p2.x - p1.x).abs();
        let dy = (p2.y - p1.y).abs();
        let delta;
        if dx > dy {
            delta = Point { x: 0, y: 1 };
        } else {
            delta = Point { x: 1, y: 0 };
        }

        let l = VectorLine::new(p1, p2);
        l.for_each(|p| {
            if self.in_bounds(p) {
                let idx = map_idx(p.x, p.y);
                self.tiles[idx] = TileType::Floor;
            }

            if self.in_bounds(p + delta) {
                let idx = map_point(p + delta);
                self.tiles[idx] = TileType::Floor;
            }

            if self.in_bounds(p - delta) {
                let idx = map_point(p - delta);
                self.tiles[idx] = TileType::Floor;
            }

            if self.in_bounds(p + delta * 2)
                && !self.rooms[r1].area.point_in_rect(p + delta * 2)
                && !self.rooms[r2].area.point_in_rect(p + delta * 2)
            {
                let idx = map_point(p + delta * 2);
                self.tiles[idx] = TileType::Wall;
            }

            if self.in_bounds(p - delta * 2)
                && !self.rooms[r1].area.point_in_rect(p - delta * 2)
                && !self.rooms[r2].area.point_in_rect(p - delta * 2)
            {
                let idx = map_point(p - delta * 2);
                self.tiles[idx] = TileType::Wall;
            }
        });
    }

    pub fn build_random_room(&mut self) {
        let parent_idx = self.rng.range(0, self.rooms.len());
        let parent_center = self.rooms[parent_idx].area.center();

        let lower_x = max(1, parent_center.x - 50);
        let upper_x = min(MAP_WIDTH - 20, parent_center.x + 50);

        let lower_y = max(1, parent_center.y - 50);
        let upper_y = min(MAP_HEIGHT - 20, parent_center.y + 50);

        let mut overlap = true;
        let mut room = Rect::with_size(
            self.rng.range(lower_x, upper_x),
            self.rng.range(lower_y, upper_y),
            self.rng.range(25, 50),
            self.rng.range(25, 50),
        );
        while overlap {
            overlap = false;
            room = Rect::with_size(
                self.rng.range(lower_x, upper_x),
                self.rng.range(lower_y, upper_y),
                self.rng.range(25, 50),
                self.rng.range(25, 50),
            );
            for r in self.rooms.iter() {
                if r.area.intersect(&room) {
                    overlap = true;
                }
            }
        }
        self.draw_room(room);
        self.add_coridor(parent_idx, self.rooms.len() - 1);
        for path in 0..self.paths.len() {
            self.draw_coridor(self.paths[path]);
        }
    }

    pub fn get_position_of_enemies(&mut self) -> Vec<Point> {
        let mut v = Vec::new();
        for i in 0..self.rooms.len() {
            v.push(self.rooms[i].area.center());
        }
        v
    }
}
