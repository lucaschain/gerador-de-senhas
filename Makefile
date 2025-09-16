build:
	@echo "Limpando a pasta docs"
	rm -rf docs/*
	@echo "Buildando o projeto..."
	dx build --release
	cp -r target/dx/senhas/release/web/public/* docs/
	@echo "Adicionando assets..."
	cp -a assets/. docs/
	@echo "Build completa! Os arquivos estão na pasta docs/"

.PHONY: build
