MAIN_SRC=cmus-rpc
EXE_NAME=cmus-rpc
INSTALL_PATH=/usr/bin

run:
	./${MAIN_SRC} --debug

install:
	@echo "1. copying main script"
	cp -pv ${MAIN_SRC} ${INSTALL_PATH}/${EXE_NAME}

uninstall:
	rm -v ${INSTALL_PATH}/${EXE_NAME}
