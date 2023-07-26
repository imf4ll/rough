install:
	cargo install --path .
	mkdir ${HOME}/.config/rough
	cp ./assets/config.json ${HOME}/.config/rough/

update:
	git pull origin master
	cargo install --path .
