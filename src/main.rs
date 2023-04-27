mod maze;

use crossterm::{Result,
};

fn main() -> Result<()> {
    let maze = maze::Maze::new(10, 10);
    maze.print();
    let res = maze.dijkstra();
    res.print();
    Ok(())
}