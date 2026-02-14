pack-forger:
	./scripts/pack-forger.sh

up:
	cargo run -p vmctl -- up

down:
	cargo run -p vmctl -- down

provision:
	cargo run -p vmctl -- provision

reload:
	cargo run -p vmctl -- reload

.PHONY: pack-forger up down provision reload
