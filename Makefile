MAIN_SRC=cmus-rpc.py
EXE_NAME=cmus-rpc
INSTALL_PATH=~/.local/bin

run:
	python3 ${MAIN_SRC} --debug

install: 
	@echo "1. copying main script"
	cp -pv ${MAIN_SRC} ${INSTALL_PATH}/${EXE_NAME}

uninstall: 
	rm -v ${INSTALL_PATH}/${EXE_NAME}
