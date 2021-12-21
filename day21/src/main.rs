use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(4, 8), 739785);
    println!("part 1: {}", calc1(2, 5));

    // Part 2
    assert_eq!(calc2(4, 8), 444356092776315);
    println!("part 2: {}", calc2(2, 5));
}

struct Player {
    position: usize,
    score: usize,
}

struct Game {
    players: Vec<Player>,
    turn: usize,
    roll_count: usize,
    prev_roll: usize,
}

impl Game {
    fn new(p1: usize, p2: usize) -> Self {
        let p1 = Player {
            position: p1,
            score: 0,
        };
        let p2 = Player {
            position: p2,
            score: 0,
        };
        Game {
            players: vec![p1, p2],
            turn: 0,
            roll_count: 0,
            prev_roll: 0,
        }
    }

    fn turn(&mut self) -> bool {
        let p = &mut self.players[self.turn];
        let mut spaces = 0;
        for _ in 0..3 {
            self.prev_roll = self.prev_roll % 100 + 1;
            self.roll_count += 1;
            spaces += self.prev_roll
        }
        p.position = (p.position + spaces - 1) % 10 + 1;
        p.score += p.position;
        self.turn = (self.turn + 1) % 2;
        p.score >= 1000
    }

    fn loser_score(&self) -> usize {
        self.players.iter().map(|x| x.score).min().unwrap()
    }

    fn play(&mut self) -> usize {
        while !self.turn() {}
        self.roll_count * self.loser_score()
    }
}

fn calc1(p1: usize, p2: usize) -> usize {
    let mut g = Game::new(p1, p2);
    g.play()
}

struct DiracGame {
    cache: HashMap<(usize, usize, usize, usize, usize), (usize, usize)>,
}

fn pos(pos: usize, roll: usize) -> usize {
    (pos + roll - 1) % 10 + 1
}

impl DiracGame {
    fn new() -> Self {
        DiracGame {
            cache: HashMap::new(),
        }
    }

    fn memo_play(
        &mut self,
        p1_pos: usize,
        p2_pos: usize,
        p1_score: usize,
        p2_score: usize,
        turn: usize,
    ) -> (usize, usize) {
        let item = self.cache.get(&(p1_pos, p2_pos, p1_score, p2_score, turn));
        if item == None {
            let result = self.play(p1_pos, p2_pos, p1_score, p2_score, turn);
            self.cache
                .insert((p1_pos, p2_pos, p1_score, p2_score, turn), result);
            result
        } else {
            *item.unwrap()
        }
    }

    fn play(
        &mut self,
        p1_pos: usize,
        p2_pos: usize,
        p1_score: usize,
        p2_score: usize,
        turn: usize,
    ) -> (usize, usize) {
        if p1_score >= 21 {
            return (1, 0);
        }
        if p2_score >= 21 {
            return (0, 1);
        }
        let outcomes = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let mut universes = (0, 0);
        for (roll, freq) in &outcomes {
            let u = match turn {
                0 => {
                    let pos = pos(p1_pos, *roll);
                    self.memo_play(pos, p2_pos, p1_score + pos, p2_score, 1)
                }
                1 => {
                    let pos = pos(p2_pos, *roll);
                    self.memo_play(p1_pos, pos, p1_score, p2_score + pos, 0)
                }
                _ => panic!(),
            };
            universes.0 += u.0 * freq;
            universes.1 += u.1 * freq;
        }
        universes
    }
}

fn calc2(p1: usize, p2: usize) -> usize {
    let mut g = DiracGame::new();
    let (s1, s2) = g.memo_play(p1, p2, 0, 0, 0);
    s1.max(s2)
}
