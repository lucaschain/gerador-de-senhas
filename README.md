# gerador de senhas
modifique isto como quiser, pois sem licensa de uso tu pode fazer oque quiser.

## Como fazer o build

Sempre que mudar o código, antes de dar o `git push`, você deve fazer o build do projeto.

Pra fazer o build, instale a ferramenta `make` e rode o comando:

```
make build
```

O que o comando faz?

- limpa os conteúdos da pasta `docs/`
- faz o build do projeto
- joga os arquivos da pasta `assets/` pra dentro da pasta `docs/`
- joga os arquivos da build pra dentro da pasta `docs/`


# como compilar e testar
digite:
```
cargo install cargo-binstall #(pra instalar via cargo install precisa de cmake e nasm netwide assemblier)

cargo binstall dioxus-cli

dx serve #caso queira testar antes de compilar

dx build --release #pra poder compilar
```