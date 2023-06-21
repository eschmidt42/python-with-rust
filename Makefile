SHELL = /bin/bash

.PHONY: help
help:
	@echo "Commands:"
	@echo "venv    : creates the virtual environment in .venv."
	@echo "install : install dependencies into virtual environment."
	@echo "compile : update the environment requirements after changes to dependencies in pyproject.toml."
	@echo "update  : install dependencies into virtual environment after 'make compile'."
	@echo "test    : run pytests."
	@echo "build   : build rust bits"

# create a virtual environment
.PHONY: venv
venv:
	python3 -m venv .venv
	source .venv/bin/activate && \
	python3 -m pip install pip==23.0.1 setuptools==67.6.1 wheel==0.40.0 && \
	pip install pip-tools==6.12.3

# ==============================================================================
# install requirements
# ==============================================================================

req-file := config/requirements.txt


# environment for production
.PHONY: install
install: venv
	source .venv/bin/activate && \
	pip-sync $(req-file) && \
	pip install -e .

# ==============================================================================
# update dependencies after changes to pyproject.toml and `make compile`
# ==============================================================================

.PHONY: update
update: 
	source .venv/bin/activate && \
	pip-sync $(req-file) && \
	pip install -e .

# ==============================================================================
# build rust component
# ==============================================================================

# environment for production
.PHONY: build
build: venv
	source .venv/bin/activate && \
	maturin develop

# ==============================================================================
# compile requirements
# ==============================================================================

.PHONY: compile
compile:
	source .venv/bin/activate && \
	pip-compile pyproject.toml -o $(req-file) --resolver=backtracking


# ==============================================================================
# run tests
# ==============================================================================

.PHONY: test
test:
	source .venv/bin/activate && \
	pytest -vx .
