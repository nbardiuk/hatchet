cargo-args=
ifdef features
	cargo-args=--features $(features)
endif

.PHONY: tdd
tdd:
	cargo watch --clear --shell "time cargo test $(only) $(cargo-args) -q -- --nocapture"

.PHONY: test
test:
	cargo test $(only) $(cargo-args)

.PHONY: clean
clean:
	cargo clean

.PHONY: run
run:
	cargo run $(cargo-args)
