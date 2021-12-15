use crate::grid::{Grid, points, adjacent8_points};

fn inc(grid: &mut Grid<u8>, flashed: &mut Grid<bool>, x: usize, y: usize) {
	let v = &mut grid[[x, y]];
	*v += 1;
	let v = *v;
	if v > 9 && !flashed[[x, y]] {
		flashed[[x, y]] = true;
		for (x, y) in adjacent8_points(grid, x, y) {
			inc(grid, flashed, x, y);
		}
	}
}

fn advance(grid: &mut Grid<u8>) -> usize {
	let mut flashed = Grid::blank(grid, false);
	for (x, y) in points(grid) {
		inc(grid, &mut flashed, x, y);
	}
	let mut flashes = 0;
	for (x, y, &f ) in flashed.iter() {
		if f {
			flashes += 1;
			grid[[x, y]] = 0;
		}
	}
	flashes
}

#[aoc(day11, part1)]
fn day11_part1(input: &str) -> usize {
	let mut grid = Grid::from_number_grid(input);
	let mut flashes = 0;
	for _ in 0..100 {
		flashes += advance(&mut grid);
	}
	flashes
}
#[aoc(day11, part2)]
fn day11_part2(input: &str) -> usize {
	let mut grid = Grid::from_number_grid(input);
	for i in 1.. {
		let flashes = advance(&mut grid);
		if flashes == grid.width() * grid.height() {
			return i;
		}
	}
	panic!();
}

#[cfg(test)]
const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn test_p1() {
	assert_eq!(1656, day11_part1(EXAMPLE));
}
#[test]
fn test_p2() {
	assert_eq!(195, day11_part2(EXAMPLE));
}