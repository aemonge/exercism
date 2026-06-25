#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Point {
    Strike,
    Spare(u16, u16),
    Frame(u16),
    Nil,
}

pub struct BowlingGame {
    rolls: [[Point; 2]; 10],
    turn: [usize; 2],
    fill_ball: [u16; 2],
}

impl BowlingGame {
    pub fn new() -> Self {
        let rolls = [
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
            [Point::Nil, Point::Nil],
        ];
        BowlingGame {
            rolls,
            turn: [0, 0],
            fill_ball: [0, 0],
        }
    }

    fn get_first_points(pins: u16) -> Point {
        if pins == 10 {
            Point::Strike
        } else {
            Point::Frame(pins)
        }
    }

    fn get_second_points(remaining_pins: u16, hit_pins: u16) -> Point {
        match remaining_pins {
            0 => Point::Nil,
            i if i == hit_pins => Point::Spare(10 - hit_pins, hit_pins),
            _ => Point::Frame(hit_pins),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.turn {
            [10, 0]
                if Point::Strike == self.rolls[9][0]
                    || matches!(self.rolls[9][1], Point::Spare(_, _)) =>
            {
                self.fill_ball[0] = pins;
                self.turn = [10, 1];
                Ok(())
            }
            [10, 0] => Err(Error::GameComplete),
            [10, 1] if Point::Strike == self.rolls[9][0] || self.fill_ball[0] == 10 => {
                self.fill_ball[1] = pins;
                Ok(())
            }
            [10, 1] => Err(Error::GameComplete),
            [i, 0] => {
                let point = Self::get_first_points(pins);
                if point == Point::Strike {
                    self.turn = [i + 1, 0];
                } else {
                    self.turn = [i, 1];
                }
                self.rolls[i] = [point, Point::Nil];
                Ok(())
            }
            [i, 1] => {
                let previous_rol = &self.rolls[i];
                let remaining_pins = match previous_rol[0] {
                    Point::Frame(x) => 10 - x,
                    Point::Strike => 0,
                    Point::Spare(_, _) => 0,
                    Point::Nil => 10,
                };
                self.rolls[i][1] = Self::get_second_points(remaining_pins, pins);
                self.turn = [i + 1, 0];
                Ok(())
            }

            _ => Err(Error::GameComplete),
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.rolls[9][0] == Point::Nil {
            return None;
        }
        let mut score = 0;
        let mut last_rolls = self.fill_ball;
        dbg!(&self.fill_ball);
        // NOTE: last_rolls index will be inverted since we are iterating backwards
        for roll in self
            .rolls
            .iter()
            .rev()
        {
            dbg!(&roll, score);
            let mut rolls_iter = roll
                .iter()
                .rev();
            while let Some(throw) = rolls_iter.next() {
                match throw {
                    Point::Frame(x) => {
                        score += x;

                        last_rolls[1] = last_rolls[0];
                        last_rolls[0] = *x;
                    }
                    Point::Strike => {
                        dbg!(&last_rolls);
                        dbg!(&score, 10 + last_rolls[1] + last_rolls[0]);
                        score += 10 + last_rolls[0] + last_rolls[1];
                        dbg!(&score);

                        last_rolls[1] = last_rolls[0];
                        last_rolls[0] = 10;
                    }
                    Point::Spare(x, y) => {
                        score += 10 + last_rolls[0];

                        last_rolls[0] = *x;
                        last_rolls[1] = *y;
                        rolls_iter.next();
                    }
                    Point::Nil => {}
                }
            }
            dbg!(score);
            eprintln!("============");
        }
        Some(score)
    }
}
