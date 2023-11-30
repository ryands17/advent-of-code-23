default:
	@just --list

# run the app, e.g.: run 01 2 => runs part 2 for the first day's challenge
run DAY PART:
	cargo run -p day-{{DAY}} --bin part{{PART}}
