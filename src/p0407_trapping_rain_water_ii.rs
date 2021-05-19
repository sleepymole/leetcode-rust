#![allow(dead_code)]
pub struct Solution;

use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Eq)]
struct Cell {
    x: usize,
    y: usize,
    h: i32,
}

impl Cell {
    fn new(x: usize, y: usize, h: i32) -> Self {
        Cell { x, y, h }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.h.cmp(&self.h)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        if m <= 1 {
            return 0;
        }
        let n = height_map[0].len();
        if n <= 1 {
            return 0;
        }
        let mut vis = vec![vec![false; n]; m];
        let mut q = BinaryHeap::new();
        for i in 0..m {
            q.push(Cell::new(i, 0, height_map[i][0]));
            q.push(Cell::new(i, n - 1, height_map[i][n - 1]));
            vis[i][0] = true;
            vis[i][n - 1] = true;
        }
        for j in 1..n - 1 {
            q.push(Cell::new(0, j, height_map[0][j]));
            q.push(Cell::new(m - 1, j, height_map[m - 1][j]));
            vis[0][j] = true;
            vis[m - 1][j] = true;
        }
        let mut volume = 0;
        let mut low = 0;
        while let Some(cell) = q.pop() {
            if cell.h < low {
                volume += low - cell.h;
            } else {
                low = cell.h;
            }
            if cell.x > 0 && !vis[cell.x - 1][cell.y] {
                vis[cell.x - 1][cell.y] = true;
                q.push(Cell::new(
                    cell.x - 1,
                    cell.y,
                    height_map[cell.x - 1][cell.y],
                ));
            }
            if cell.x + 1 < m && !vis[cell.x + 1][cell.y] {
                vis[cell.x + 1][cell.y] = true;
                q.push(Cell::new(
                    cell.x + 1,
                    cell.y,
                    height_map[cell.x + 1][cell.y],
                ));
            }
            if cell.y > 0 && !vis[cell.x][cell.y - 1] {
                vis[cell.x][cell.y - 1] = true;
                q.push(Cell::new(
                    cell.x,
                    cell.y - 1,
                    height_map[cell.x][cell.y - 1],
                ));
            }
            if cell.y + 1 < n && !vis[cell.x][cell.y + 1] {
                vis[cell.x][cell.y + 1] = true;
                q.push(Cell::new(
                    cell.x,
                    cell.y + 1,
                    height_map[cell.x][cell.y + 1],
                ));
            }
        }
        volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_rain_water() {
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1]
            ]),
            4
        );
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![3, 3, 3, 3, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 2, 1, 2, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 3, 3, 3, 3]
            ]),
            10
        );
    }
}
