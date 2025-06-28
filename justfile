default:
    @just --list

# Run 'cargo run' on the project
run *ARGS:
    cargo run {{ARGS}}

# Watch for changes and automatically restart
watch:
	bacon
