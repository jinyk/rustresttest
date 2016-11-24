help:
	@echo "Docker image for testing Rust"

build:
	docker build -t jinyk/rusttest .

run:
	docker stop rusttest
	docker rm rusttest
	docker run -v /Users/jinyk/Documents/code/_tmp/rusttest/code:/code --name rusttest -h rusttest -ti jinyk/rusttest zsh
