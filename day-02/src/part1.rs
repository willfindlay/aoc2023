use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"Game (\d+): ((:?[^;]+;?)+)").unwrap();
    static ref COLOUR_RE: Regex = Regex::new(r"(\d+) (blue|green|red),?").unwrap();
}

#[derive(Default, Debug)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    pub fn power(&self) -> u32 {
        let min_balls = self.draws.iter().fold((0, 0, 0), |acc, draw| {
            (
                acc.0.max(draw.red),
                acc.1.max(draw.blue),
                acc.2.max(draw.green),
            )
        });
        return min_balls.0 * min_balls.1 * min_balls.2;
    }
}

#[derive(Default, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game::default();
        let Some(m) = GAME_RE.captures(s) else {
            anyhow::bail!("no match")
        };
        game.id = m[1].parse().unwrap();
        let rest = &m[2];
        let draws = rest.split(";");

        for draw_s in draws {
            let mut draw = Draw::default();
            for m in COLOUR_RE.captures_iter(draw_s) {
                let n: u32 = m[1].parse().unwrap();
                match &m[2] {
                    "blue" => draw.blue += n,
                    "green" => draw.green += n,
                    "red" => draw.red += n,
                    _ => unreachable!(),
                }
            }
            game.draws.push(draw);
        }

        return Ok(game);
    }
}
pub fn run(input: &str) -> anyhow::Result<String> {
    Ok(input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|g| {
            g.draws
                .iter()
                .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
        })
        .fold(0, |acc, g| acc + g.id)
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!("8", run(input)?);

        Ok(())
    }
}
