VERSION=v$(shell cargo read-manifest| jq -r .version)

.PHONY:
install:
	cargo install --path .

.PHONY:
format:
	cargo clippy --fix --allow-dirty
	cargo fmt

.PHONY:
release:
	git tag -a $(VERSION) -m "release $(VERSION)"
	git push origin $(VERSION)
	cargo publish

.PHONY:
docker:
	./build.sh