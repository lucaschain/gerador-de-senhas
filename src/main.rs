use dioxus::prelude::*;
use dioxus_web::launch;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use wasm_bindgen::JsCast;
use web_sys::{Blob, Url, window};
use js_sys::Array;

#[component]
fn App() -> Element {
    let mut quantidade = use_signal(|| 8);
    let mut senha = use_signal(|| String::new());
    let mut escuro = use_signal(|| false);
    let mut uppercase = use_signal(|| true);
    let mut numbers = use_signal(|| true);
    let mut special = use_signal(|| true);

    let tema_fundo = if *escuro.read() { "#1e1e1e" } else { "#f0f0f0" };
    let tema_texto = if *escuro.read() { "#ffffff" } else { "#000000" };
    let sidebar_bg = if *escuro.read() { "#2c2c2c" } else { "#f9f9f9" };
    let card_bg = if *escuro.read() { "#333333" } else { "#ffffff" };
    let card_border = if *escuro.read() { "#555555" } else { "#ccc" };

    rsx! {
        div {
            style: "margin: 0px; display: flex; height: 100vh; font-family: monospace; background-color: {tema_fundo}; color: {tema_texto};",
            div {
                style: "width: 220px; background-color: {sidebar_bg}; padding: 20px; height: 100vh; display: flex; flex-direction: column; border-right: 1px solid #ddd;",
                h2 { "⚙️ Opções" }
                ul {
                    style: "list-style: none; padding: 0; margin: 0;",
                    li {
                        style: "padding: 8px 0; cursor: pointer; font-size: 18px; background-color: #0984ff; color: white; border-radius: 6px; text-align: center; margin-bottom: 16px;",
                        onclick: move |_| {
                            let atual = *escuro.read();
                            escuro.set(!atual);
                        },
                        if *escuro.read() { "☀️ Tema Claro" } else { "🌙 Tema Escuro" }
                    }
                    li {
                        style: "padding: 8px 0; cursor: pointer; font-size: 16px; background-color: #6c5ce7; color: white; border-radius: 6px; text-align: center; margin-bottom: 8px;",
                        onclick: move |_| {
                            let atual = *uppercase.read();
                            uppercase.set(!atual);
                        },
                        if *uppercase.read() { "🔠 Letras Maiúsculas: ON" } else { "🔡 Letras Maiúsculas: OFF" }
                    }
                    li {
                        style: "padding: 8px 0; cursor: pointer; font-size: 16px; background-color: #fdcb6e; color: black; border-radius: 6px; text-align: center; margin-bottom: 8px;",
                        onclick: move |_| {
                            let atual = *numbers.read();
                            numbers.set(!atual);
                        },
                        if *numbers.read() { "🔢 Números: ON" } else { "🚫 Números: OFF" }
                    }
                    li {
                        style: "padding: 8px 0; cursor: pointer; font-size: 16px; background-color: #00cec9; color: black; border-radius: 6px; text-align: center; margin-bottom: 16px;",
                        onclick: move |_| {
                            let atual = *special.read();
                            special.set(!atual);
                        },
                        if *special.read() { "✨ Especiais: ON" } else { "🚫 Especiais: OFF" }
                    }
                    li {
                        style: "padding: 8px 0;",
                        label {
                            style: "font-size: 14px;",
                            "Comprimento da senha: {quantidade.read()}"
                        }
                        input {
                            r#type: "range",
                            min: "1",
                            max: "100",
                            value: quantidade.read().to_string(),
                            oninput: move |evt| {
                                if let Ok(val) = evt.value().parse::<i32>() {
                                    quantidade.set(val);
                                }
                            }
                        }
                    }
                    li {
                        style: "padding: 8px 0; cursor: pointer; font-size: 18px; background-color: #00b894; color: white; border-radius: 6px; text-align: center;",
                        onclick: move |_| {
                            let mut charset: Vec<u8> = Vec::new();

                            if *uppercase.read() {
                                charset.extend(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                charset.extend(b"abcdefghijklmnopqrstuvwxyz");
                            } else {
                                charset.extend(b"abcdefghijklmnopqrstuvwxyz");
                            }

                            if *numbers.read() {
                                charset.extend(b"0123456789");
                            }

                            if *special.read() {
                                charset.extend(b"!@#$%&*()_+-=[]{}|;:,.<>?");
                            }

                            if charset.is_empty() {
                                charset.extend(b"abcdefghijklmnopqrstuvwxyz");
                            }

                            let mut rng = SmallRng::from_entropy();
                            let len = quantidade.read().max(1);

                            let senhas: Vec<String> = (0..10).map(|_| {
                                (0..len).map(|_| {
                                    let idx = rng.gen_range(0..charset.len());
                                    charset[idx] as char
                                }).collect()
                            }).collect();

                            let conteudo = senhas.join("\n");

                            let array = Array::new();
                            array.push(&wasm_bindgen::JsValue::from_str(&conteudo));

                            let blob = Blob::new_with_str_sequence(&array).unwrap();
                            let url = Url::create_object_url_with_blob(&blob).unwrap();

                            let document = window().unwrap().document().unwrap();
                            let a = document.create_element("a").unwrap();
                            a.set_attribute("href", &url).unwrap();
                            a.set_attribute("download", "senhas.txt").unwrap();
                            a.set_attribute("style", "display: none").unwrap();

                            document.body().unwrap().append_child(&a).unwrap();
                            let a_html = a.dyn_ref::<web_sys::HtmlElement>().unwrap();
                            a_html.click();
                            a.remove();
                        },
                        "📥 Baixar 10 Senhas"
                    }
                }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 16px; padding: 20px;",
                h1 {
                    style: "font-size: 32px; margin-bottom: 8px;",
                    "🔐 Gerador de Senhas Fortes"
                }
                button {
                    style: "background-color: #0984ff; color: white; border: none; padding: 10px 20px; font-size: 16px; border-radius: 8px; cursor: pointer; transition: background-color 0.3s ease;",
                    onclick: move |_| {
                        let mut charset: Vec<u8> = Vec::new();

                        if *uppercase.read() {
                            charset.extend(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                            charset.extend(b"abcdefghijklmnopqrstuvwxyz");
                        } else {
                            charset.extend(b"abcdefghijklmnopqrstuvwxyz");
                        }

                        if *numbers.read() {
                            charset.extend(b"0123456789");
                        }

                        if *special.read() {
                            charset.extend(b"!@#$%&*()_+-=[]{}|;:,.<>?");
                        }

                        if charset.is_empty() {
                            charset.extend(b"abcdefghijklmnopqrstuvwxyz");
                        }

                        let mut rng = SmallRng::from_entropy();
                        let len = quantidade.read().max(1);
                        let password: String = (0..len).map(|_| {
                            let idx = rng.gen_range(0..charset.len());
                            charset[idx] as char
                        }).collect();
                        senha.set(password);
                    },
                    "🔁 Gerar Nova Senha"
                }
                h2 { "Senha Gerada:" }
                div {
                    style: "background: {card_bg}; padding: 10px 16px; border-radius: 6px; border: 1px solid {card_border}; font-size: 18px; max-width: 100%; word-break: break-word;",
                    "{senha.read()}"
                }
            }
        }
    }
}

fn main() {
    launch(App);
}

