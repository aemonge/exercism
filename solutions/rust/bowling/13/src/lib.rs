const PINS: u16 = 10;
const PINS_IX: usize = PINS as usize;

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
    rolls: [[Point; 2]; PINS_IX],
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
        if pins == PINS {
            Point::Strike
        } else {
            Point::Frame(pins)
        }
    }

    fn get_second_points(remaining_pins: u16, hit_pins: u16) -> Point {
        match remaining_pins {
            0 => Point::Spare(PINS - hit_pins, hit_pins),
            _ => Point::Frame(hit_pins),
        }
    }

    fn is_valid_rol(&mut self, pins: u16) -> Result<u16, Error> {
        if pins > PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        let remaining_pins = match self.turn {
            [i, 0] if i <= PINS_IX => {
                PINS.checked_sub(pins) // PINS - pins,
                    .ok_or(Error::NotEnoughPinsLeft)?
            }
            [i, 1] if i < PINS_IX => match self.rolls[i][0] {
                Point::Frame(x) => PINS // PINS - (prev + curront; pins),
                    .checked_sub(pins + x)
                    .ok_or(Error::NotEnoughPinsLeft)?,
                Point::Strike => PINS
                    .checked_sub(pins)
                    .ok_or(Error::NotEnoughPinsLeft)?,
                _ => return Err(Error::NotEnoughPinsLeft),
            },
            [10, 1] => PINS
                .checked_sub(pins + self.fill_ball[0])
                .ok_or(Error::NotEnoughPinsLeft)?,
            _ => return Err(Error::NotEnoughPinsLeft),
        };
        Ok(remaining_pins)
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let remaining_pins = self.is_valid_rol(pins)?;
        match self.turn {
            [PINS_IX, 0]
                if Point::Strike == self.rolls[9][0]
                    || matches!(self.rolls[9][1], Point::Spare(_, _)) =>
            {
                self.fill_ball[0] = pins;
                self.turn = [PINS_IX, 1];
                Ok(())
            }
            [PINS_IX, 0] => Err(Error::GameComplete),
            [PINS_IX, 1]
                if Point::Strike == self.rolls[9][0] || self.fill_ball[0] == PINS =>
            {
                self.fill_ball[1] = pins;
                Ok(())
            }
            [PINS_IX, 1] => Err(Error::GameComplete),
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
        // NOTE: last_rolls index will be inverted since we are iterating backwards
        for roll in self
            .rolls
            .iter()
            .rev()
        {
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
                        score += PINS + last_rolls[0] + last_rolls[1];

                        last_rolls[1] = last_rolls[0];
                        last_rolls[0] = PINS;
                    }
                    Point::Spare(x, y) => {
                        score += PINS + last_rolls[0];

                        last_rolls[0] = *x;
                        last_rolls[1] = *y;
                        rolls_iter.next();
                    }
                    Point::Nil => {}
                }
            }
        }
        Some(score)
    }
}
