target/setup-yew.touchfile:
	cargo install --locked trunk
	mkdir -p target
	touch target/setup-yew.touchfile


run-frontend: target/setup-yew.touchfile
	cd ./napoli-pain && trunk serve
