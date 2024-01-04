use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space0, space1},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

#[derive(Debug)]
struct SeedRange {
    seed: u64,
    range: u64,
}

impl<'a> IntoIterator for &'a SeedRange {
    type Item = u64;

    type IntoIter = SeedRageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SeedRageIterator { inner: self, i: 0 }
    }
}

struct SeedRageIterator<'a> {
    inner: &'a SeedRange,
    i: usize,
}

impl<'a> Iterator for SeedRageIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.i as u64) < self.inner.range {
            self.i += 1;
            Some(self.inner.seed + self.i as u64)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Alminac {
    seeds: Vec<SeedRange>,
    maps: Vec<AlminacMap>,
}

impl Alminac {
    // fn find_lowest_location(&self) -> u64 {
    //     let mut maps = self.maps.iter().rev();
    //     let location_map = maps.next().unwrap();

    //     dbg!(location_map.possible_destinations().collect::<Vec<_>>());

    //     for dst in location_map.possible_destinations() {
    //         let mut mapping = dst;
    //         for map in maps.clone() {
    //             mapping = map.get_rev(mapping);
    //         }
    //         if self.seeds.iter().flatten().any(|seed| seed == mapping) {
    //             return dst;
    //         }
    //     }
    //     0
    // }
    fn find_lowest_location(&self) -> u64 {
        let mut lowest = u64::MAX;
        for seed in self.seeds.iter().flatten() {
            let mut mapping = seed;
            for map in &self.maps {
                mapping = map.get(mapping);
            }
            if mapping < lowest {
                lowest = mapping;
            }
        }
        lowest
    }
}

#[derive(Debug)]
struct AlminacMap(Vec<Mapping>);

impl AlminacMap {
    fn get(&self, src: u64) -> u64 {
        for map in &self.0 {
            if let Some(x) = map.get(src) {
                return x;
            }
        }
        src
    }
}

#[derive(Debug)]
struct Mapping {
    src: u64,
    dst: u64,
    range: u64,
}

impl Mapping {
    fn get(&self, src: u64) -> Option<u64> {
        if src >= self.src && src < self.src + self.range {
            return Some((src - self.src) + self.dst);
        }
        None
    }
}

fn parse_range(input: &str) -> nom::IResult<&str, SeedRange> {
    let (input, (seed, range)) = separated_pair(complete::u64, space1, complete::u64)(input)?;
    Ok((input, SeedRange { seed, range }))
}

fn parse_seeds(input: &str) -> nom::IResult<&str, Vec<SeedRange>> {
    let (input, _) = tag("seeds: ")(input)?;
    separated_list1(space1, parse_range)(input)
}

fn parse_map(input: &str) -> nom::IResult<&str, AlminacMap> {
    let (input, _) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
    let (input, _) = terminated(tag(" map:"), line_ending)(input)?;
    let (input, mut mappings) = separated_list1(line_ending, parse_mapping)(input)?;
    mappings.sort_by(|a, b| a.dst.partial_cmp(&b.dst).unwrap());
    Ok((input, AlminacMap(mappings)))
}

fn parse_mapping(input: &str) -> nom::IResult<&str, Mapping> {
    let (input, dst) = terminated(complete::u64, space0)(input)?;
    let (input, src) = terminated(complete::u64, space0)(input)?;
    let (input, range) = terminated(complete::u64, space0)(input)?;
    Ok((input, Mapping { src, dst, range }))
}

fn parse_alminac(input: &str) -> nom::IResult<&str, Alminac> {
    let (input, seeds) = terminated(parse_seeds, line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, maps) =
        separated_list1(line_ending, terminated(parse_map, opt(line_ending)))(input)?;
    Ok((input, Alminac { seeds, maps }))
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let (_, alminac) = parse_alminac(input).map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;
    Ok(alminac.find_lowest_location().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() -> anyhow::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", run(input)?);

        Ok(())
    }

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let (input, seeds) = parse_seeds(input).expect("should parse");
        assert_eq!(2, seeds.len());
        assert_eq!("", input);
    }

    #[test]
    fn test_parse_map() {
        let input = "soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";
        let (_, map) = parse_map(input).expect("should parse");
        assert_eq!(3, map.0.len());
        assert_eq!(0, map.0[0].dst);
        assert_eq!(15, map.0[0].src);
        assert_eq!(37, map.0[0].range);
    }

    #[test]
    fn test_parse_alminac() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let (_, _) = parse_alminac(input).expect("should parse");
    }
}
