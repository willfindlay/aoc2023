# aoc year, used in get-input
year := "2023"

# list targets
help:
        just --list

# watch a day
watch day:
        cargo watch -w {{day}} \
                -s "just check {{day}} || true" \
                -s "just test {{day}} || true" \
                -s "just run {{day}} || true"

# run a day
run day:
        cargo run -p {{day}} --bin part1
        cargo run -p {{day}} --bin part2

# run a day
release-run day:
        cargo run --release -p {{day}} --bin part1
        cargo run --release -p {{day}} --bin part2

# check a day
check day:
        cargo check -p {{day}}

# lint a day
lint day:
        cargo clippy -p {{day}}

# test a day
test day:
        cargo test -p {{day}}

# generate the template for a day. day should be formatted like day-01
generate day:
        cargo generate --path ./template --name {{day}}
        just get-input {{day}}

# You can find SESSION by using Chrome tools:
# 1) Go to https://adventofcode.com/2022/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 5) Click https://adventofcode.com under "Cookies"
# 6) Grab the value for session. Fill it into your .env file
# 
# example .env:
#
# ```
# SESSION=PASTE_COOKIE_VALUE_HERE
# ```
#
# get the input for a day's puzzle
get-input day:
    ./scripts/get-aoc-input.rs --day {{day}} --year {{year}} --current-working-directory {{justfile_directory()}}