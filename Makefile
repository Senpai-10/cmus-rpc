MAIN_SRC=cmus-rpc.py
EXE_NAME=cmus-rpc
LIB_NAME=cmus_rpc_lib
INSTALL_PATH=~/.local/bin

run:
	python3 ${MAIN_SRC}

install: 
	@echo "1. copying main script"
	cp -pv ${MAIN_SRC} ${INSTALL_PATH}/${EXE_NAME}
	@echo -e "\n2. copying library"
	rm -rfv ${LIB_NAME}/__pycache__
	cp -rpv ${LIB_NAME} ${INSTALL_PATH}/${LIB_NAME}

uninstall: 
	rm -v ${INSTALL_PATH}/${EXE_NAME}
	rm -rfv ${INSTALL_PATH}/${LIB_NAME}
