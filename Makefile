p_install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

p_test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

p_format:
	black *.py

p_lint:
	ruff check *.py mylib/*.py

check:
	cargo check

build:
	cargo build

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

release:
	cargo build --release