target/setup-yew.touchfile:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk
	npx tailwindcss
	mkdir -p target
	touch target/setup-yew.touchfile


run-frontend: target/setup-yew.touchfile
	cd ./napoli-pain && trunk serve
