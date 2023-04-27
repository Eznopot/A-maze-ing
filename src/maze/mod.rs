use rand::{distributions::Uniform, prelude::Distribution};
use std::{thread, time};
use std::collections::BinaryHeap;

pub struct Maze {
    pub maze: Vec<Vec<u8>>,
    pub entry: usize,
    pub exit: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut init_maze: Vec<Vec<u8>> = Vec::with_capacity(height);
        let step = Uniform::new(1, width - 2);
        let mut rng = rand::thread_rng();
        let mut entry = step.sample(&mut rng);
        let exit = step.sample(&mut rng);

        if entry % 2 == 0 {
            entry += 1;
        }

        for i in 0..height {
            let mut tmp = Vec::with_capacity(width);
            for l in 0..width {
                if l == entry && i == 0 {
                    tmp.push(0);
                } else if l == exit && i == height - 1 {
                    tmp.push(9);
                } else if i == 0 || i == width - 1 || l == 0 || l == height - 1 || i % 2 == 0 || l % 2 == 0 {
                    tmp.push(1);
                } else {
                    tmp.push(9);
                }
            }
            init_maze.push(tmp.clone())
        }

        let mut nb_nine = 1;

        while nb_nine > 0 {
            nb_nine = 0;
            for i in 1..height - 1 {
                for l in 1..width - 1 {
                    if init_maze[i][l] == 1 {
                        let mut count = 0;
                        if init_maze[i - 1][l] == 9 {
                            count += 1;
                        }
                        if init_maze[i + 1][l] == 9 {
                            count += 1;
                        }
                        if init_maze[i][l - 1] == 9 {
                            count += 1;
                        }
                        if init_maze[i][l + 1] == 9 {
                            count += 1;
                        }
                        if ((count <= 1 && count > 0) || i == height - 1) && step.sample(&mut rng) % 3 == 0 {
                                init_maze[i][l] = 9;
                            }
                        }
                    if init_maze[i][l] == 9 {
                        let mut count = 0;
                        nb_nine += 1;
                        if init_maze[i - 1][l] == 0 {
                            count += 1;
                        }
                        if init_maze[i + 1][l] == 0 {
                            count += 1;
                        }
                        if init_maze[i][l - 1] == 0 {
                            count += 1;
                        }
                        if init_maze[i][l + 1] == 0 {
                            count += 1;
                        }
                        if count >= 1 {
                            init_maze[i][l] = 0;
                            nb_nine -= 1;
                        }
                    }
                }
            }
            if (init_maze[height - 1][exit] == 9) && (init_maze[height - 1 - 1][exit] == 0) {
                init_maze[height - 1][exit] = 0;
            } else if init_maze[height - 1][exit] == 9 {
                nb_nine += 1;
            }
            println!("---------------------------------");
            Maze::print(&Maze {
                maze: init_maze.clone(),
                entry,
                exit,
            });

            println!("---------------------------------");
            thread::sleep(time::Duration::from_millis(25));
        };

        return Maze {
            maze : init_maze,
            entry,
            exit
        }
    }

    /*
    function Dijkstra(Graph, source):
    dist[source] ← 0                        // Initialisation de la distance depuis la source
    Q ← set of all nodes in Graph           // Ajout de tous les noeuds dans un tas (ou une file) Q
    while Q is not empty:
        u ← node in Q with smallest dist[u]  // On choisit le noeud u dans Q avec la plus petite distance dist[u]
        remove u from Q
        for each neighbor v of u:
            alt ← dist[u] + length(u, v)     // On calcule la distance alternative alt à partir de la source jusqu'à v
            if alt < dist[v]:                // Si alt est plus petit que la distance actuelle à v
                dist[v] ← alt               // On met à jour la distance à v
                prev[v] ← u                 // On met à jour le noeud précédent sur le chemin le plus court à v
    return prev, dist

    */
    pub fn dijkstra(&self) -> Maze {
        let rows = self.maze.len();
        let cols = self.maze[0].len();
    
        let mut dist = vec![vec![usize::MAX; cols]; rows];
        let start_row = 0;
        let start_col = self.entry;
        dist[start_row][start_col] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((0, self.entry));
    
        let mut prev = vec![vec![None; cols]; rows];
    
        while let Some((_, node)) = heap.pop() {
            let row = node / cols;
            let col = node % cols;
            if node == (rows - 1) * cols + self.exit {
                break;
            }
            for (dr, dc) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let nrow = row as i32 + dr;
                let ncol = col as i32 + dc;
                if nrow < 0 || nrow >= rows as i32 || ncol < 0 || ncol >= cols as i32 {
                    continue;
                }
                let nrow = nrow as usize;
                let ncol = ncol as usize;
                if self.maze[nrow][ncol] == 1 {
                    continue;
                }
                let alt = dist[row][col] + 1;
                if alt < dist[nrow][ncol] {
                    dist[nrow][ncol] = alt;
                    prev[nrow][ncol] = Some(node);
                    heap.push((alt, nrow * cols + ncol));
                }
            }
        }

        let mut maze = self.maze.clone();
        maze[rows -1][self.exit] = 5;
        let mut node = (rows - 1) * cols + self.exit;
        while let Some(prev_node) = prev[node / cols][node % cols] {
            maze[prev_node / cols][prev_node % cols] = 5;
            node = prev_node;
            println!("---------------------------------");
            Maze::print(&Maze {
                maze: maze.clone(),
                entry: self.entry,
                exit: self.exit
            });

            println!("---------------------------------");
            thread::sleep(time::Duration::from_millis(50));
        }


        return Maze {
            maze : maze,
            entry: self.entry,
            exit: self.exit
        };
    }
    
    


    pub fn print(&self) {
        for i in 0..self.maze.len() {
            for l in 0..self.maze[i].len() {
                if self.maze[i][l] == 0 {
                    print!("  ");
                } else if self.maze[i][l] == 1 {
                    print!("██");
                } else if self.maze[i][l] == 9 {
                    print!("░░");
                } else if self.maze[i][l] == 5 {
                    print!("▓▓");
                }
            }
            println!("");
        }
    }
}