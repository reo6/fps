default:
    @just --list

run *ARGS:
    cargo run {{ARGS}}

watch:
	bacon
