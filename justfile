default:
	@just --list

# run the app, e.g.: run 01 2 => runs part 2 for the first day's challenge
run DAY PART:
	cargo run -p day-{{DAY}} --bin part{{PART}}

# run the tests for the part, e.g.: test 01 2 => runs tests for part 2 for the first day's challenge
test DAY PART:
	cargo test -p day-{{DAY}} --lib -- part{{PART}}::tests -- --nocapture
