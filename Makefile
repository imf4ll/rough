install:
	cargo install --path .
	mkdir ${HOME}/.config/rough
	cp ./assets/config.json ${HOME}/.config/rough/

update:
	cargo install --path .
