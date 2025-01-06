# Tauri + Angular

This template should help get you started developing with Tauri and Angular.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) + [Angular Language Service](https://marketplace.visualstudio.com/items?itemName=Angular.ng-template).


# INSTALLAZIONE e CREAZIONE 

## Installazione tools "create-tauri-app"
> cargo install create-tauri-app --locked

## esecuzione "create-tauri-app"
> cargo create-tauri-app 

✔ Project name · pianifica

✔ Identifier · it.quasar-x7.pianifica.app

✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, deno, bun)

✔ Choose your package manager · npm

✔ Choose your UI template · Angular - (https://angular.dev/)

## creazione modulo nodejs
> cd pianifica

(per entra nella cartella del progetto)

>npm install

#### esecuzione da desktop
> npm run tauri dev 

#### pacchetti aggiuntivi installati
> npm install @tauri-apps/api

(per poter effeturare le chiamate tra frontend e backend)

# Clonazione Progetto da Repository Git
> clone https://github.com/Domenico-dellaPERUTA/pianificazione.git

#### installa moduli nodejs
> nmp i

#### installa rust/cargo, tauri-cli e LLDB (debug)
> Rust e Cargo vedi documentazione ufficiale
...

> cargo install tauri-cli

#### configura plugin Visual Studio 
- rust (extensions)
- CodeLLDB
- angular (Angular Language Service)
