use std::{
    sync::{Arc, Mutex},
    thread,
};

use eframe::egui;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Text {
    text: String,
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self { text: value }
    }
}

#[derive(Debug, Deserialize)]
struct PromptResponse {
    response: String,
}

struct State {
    text: String,
    client: Client,
    response_message: String,
    egui_ctx: egui::Context,
}

pub struct App {
    state: Arc<Mutex<State>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = State {
            text: String::with_capacity(10_000),
            client: Client::new(),
            response_message: String::new(),
            egui_ctx: cc.egui_ctx.clone(),
        };

        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    fn on_update(&mut self, ui: &mut egui::Ui) {
        let mut state = self.state.lock().unwrap();

        egui::TextEdit::multiline(&mut state.text)
            .hint_text("Prompt")
            .show(ui);

        ui.label(format!("Length: {}", state.text.len()));
        ui.label(format!(
            "Word count: {}",
            state.text.split_whitespace().count()
        ));

        drop(state);

        if ui.button("Prompt").clicked() {
            let state = self.state.clone();
            let text_is_empty = state.lock().unwrap().text.is_empty();
            if !text_is_empty {
                send_request_on_thread(state);
            }
        }

        let state = self.state.lock().unwrap();

        ui.separator();
        ui.label("Response:");
        ui.label(&*state.response_message);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.on_update(ui));
    }
}

fn send_request_on_thread(state: Arc<Mutex<State>>) {
    thread::spawn(move || send_request(state));
}

fn send_request(state: Arc<Mutex<State>>) {
    let state_mutex = state.lock().unwrap();
    let request_body = Text::from(state_mutex.text.clone());

    let request = state_mutex
        .client
        .post("http://127.0.0.1:8000/prompt")
        .json(&request_body);

    drop(state_mutex);

    let result = request.send();
    let msg = match result {
        Ok(response) => response.json::<PromptResponse>().unwrap().response,
        Err(error) => format!("Error: {}", error),
    };

    let mut state_mutex = state.lock().unwrap();
    state_mutex.response_message = msg;
    state_mutex.egui_ctx.request_repaint();
}
