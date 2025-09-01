DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace

UID := $(shell id -u)
GID := $(shell id -g)
DOCKER_RUN_WRITE_OPTS := $(DOCKER_RUN_OPTS) -u $(UID):$(GID)

.PHONY: default
default: compile

# renovate: depName=ghcr.io/developerc286/clean_git_history
CLEAN_GIT_HISTORY_VERSION=1.0.4@sha256:5783341a3377a723e409e72b9ec0826a75ba944288d030978355de05ef65b186

.PHONY: check-clean-git-history
check-clean-git-history:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/developerc286/clean_git_history:$(CLEAN_GIT_HISTORY_VERSION) $(FROM)

# renovate: depName=ghcr.io/developerc286/conventional_commits_linter
CONVENTIONAL_COMMITS_LINTER_VERSION=0.15.0@sha256:b631a3cdcbed28c8938a2a6b63e16ecfd0d7ff71c28e878815adf9183e1fb599

.PHONY: check-conventional-commits-linting
check-conventional-commits-linting:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/developerc286/conventional_commits_linter:$(CONVENTIONAL_COMMITS_LINTER_VERSION) --allow-angular-type-only $(FROM)

.PHONY: check-rust-formatting
check-rust-formatting:
	cargo fmt --all -- --check --config=group_imports=StdExternalCrate

.PHONY: check-shell-formatting
check-shell-formatting:
	shfmt --simplify --diff ci/*

.PHONY: check-python-formatting
check-python-formatting:
	autopep8 --exit-code --diff --aggressive --aggressive --max-line-length 120 --recursive end-to-end-tests/

.PHONY: check-yaml-formatting
check-yaml-formatting:
	yamlfmt -verbose -lint -dstar .github/workflows/*

.PHONY: fix-rust-formatting
fix-rust-formatting:
	cargo fmt --all -- --config=group_imports=StdExternalCrate

.PHONY: fix-shell-formatting
fix-shell-formatting:
	shfmt --simplify --write ci/*

.PHONY: fix-python-formatting
fix-python-formatting:
	autopep8 --in-place --aggressive --aggressive --max-line-length 120 --recursive end-to-end-tests/

.PHONY: fix-yaml-formatting
fix-yaml-formatting:
	yamlfmt -verbose -dstar .github/workflows/*

.PHONY: check-rust-linting
check-rust-linting:
	cargo clippy --verbose -- -D warnings

.PHONY: check-github-actions-workflows-linting
check-github-actions-workflows-linting:
	actionlint -verbose -color

.PHONY: compile
compile:
	cargo build --verbose

.PHONY: unit-test
unit-test:
	cargo test --verbose

.PHONY: end-to-end-test
end-to-end-test: compile
	cd end-to-end-tests/ && behave

.PHONY: release
release:
	cargo build --release --verbose

.PHONY: publish-binary
publish-binary: release
	./ci/publish-binary.sh ${RELEASE}

.PHONY: publish-crate
publish-crate:
	cargo publish --verbose

.PHONY: dogfood-docker
dogfood-docker: release
	docker build -t consistent-whitespace -f Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) consistent-whitespace

.PHONY: publish-docker
publish-docker: release
	./ci/publish-docker.sh ${RELEASE}
