# So new files are owned by the user.
UID := $(shell id -u)
GID := $(shell id -g)

.PHONY: check-clean-git-history check-conventional-commits-linting check-rust-formatting check-python-formatting check-yaml-formatting fix-rust-formatting fix-python-formatting fix-yaml-formatting check-rust-linting check-github-actions-workflows-linting compile unit-test end-to-end-test release publish-binary publish-crate dogfood publish-docker

check-clean-git-history:
	docker build -t check-clean-git-history -f ci/check-clean-git-history.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-clean-git-history $(FROM)

check-conventional-commits-linting:
	docker build -t check-conventional-commits-linting -f ci/check-conventional-commits-linting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-conventional-commits-linting $(FROM)

check-rust-formatting:
	docker build -t check-rust-formatting -f ci/check-rust-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-rust-formatting

check-shell-formatting:
	docker pull mvdan/shfmt:v3.9.0-alpine@sha256:35917e76b35abf2c13d55b5c20f2e34a01e9b6ae71b443d916897bc7f1873451
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) mvdan/shfmt:v3.9.0-alpine --simplify --diff ci/* 

check-python-formatting:
	docker build -t check-python-formatting -f ci/check-python-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-python-formatting

check-yaml-formatting:
	docker pull ghcr.io/google/yamlfmt:0.17.0@sha256:b4ebf4ff064f5bcf779ef4799dad1fc52542e137677699210aea2de2b270e97f
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) ghcr.io/google/yamlfmt:0.17.0 -verbose -lint -dstar .github/workflows/*

fix-rust-formatting:
	docker build -t fix-rust-formatting -f ci/fix-rust-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) fix-rust-formatting

fix-shell-formatting:
	docker pull mvdan/shfmt:v3.11.0-alpine@sha256:394d755b6007056a2e6d7537ccdbdcfca01b9855ba91e99df0166ca039c9d422
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) mvdan/shfmt:v3.11.0-alpine --simplify --write ci/* 

fix-python-formatting:
	docker build -t fix-python-formatting -f ci/fix-python-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) fix-python-formatting

fix-yaml-formatting:
	docker pull ghcr.io/google/yamlfmt:0.17.0@sha256:b4ebf4ff064f5bcf779ef4799dad1fc52542e137677699210aea2de2b270e97f
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) ghcr.io/google/yamlfmt:0.17.0 -verbose -dstar .github/workflows/*

check-rust-linting:
	docker build -t check-rust-linting -f ci/check-rust-linting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-rust-linting

check-github-actions-workflows-linting:
	docker pull rhysd/actionlint:1.7.7@sha256:887a259a5a534f3c4f36cb02dca341673c6089431057242cdc931e9f133147e9
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) rhysd/actionlint:1.7.7 -verbose -color

compile:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) compile

unit-test:
	docker build -t unit-test -f ci/unit-test.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) unit-test

end-to-end-test: compile
	docker build -t end-to-end-test -f ci/end-to-end-test.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) end-to-end-test

release:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) compile --release

publish-binary: release
	docker build -t publish-binary -f ci/publish-binary.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) -e GH_TOKEN publish-binary $(RELEASE)

publish-crate:
	docker build -t publish-crate -f ci/publish-crate.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) -e CARGO_REGISTRY_TOKEN publish-crate

dogfood: release
	docker build -t consistent-whitespace -f Dockerfile .
	docker run --rm -v $(PWD):/workspace consistent-whitespace

publish-docker: release
	./ci/publish-docker.sh ${RELEASE}
