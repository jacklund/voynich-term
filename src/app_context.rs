use crate::theme::THEME;
use rand::{self, seq::SliceRandom};
use ratatui::prelude::*;
use std::collections::HashMap;
use tor_client_lib::key::TorServiceId;
use voynich::chat::{Chat, ChatList};

#[derive(Debug)]
pub struct ConnectionContext {
    pub connection_address: TorServiceId,
    pub accept_selected: bool,
}

impl ConnectionContext {
    pub fn new(address: &TorServiceId) -> Self {
        Self {
            connection_address: address.clone(),
            accept_selected: true,
        }
    }
}

#[derive(Debug)]
pub struct AppContext {
    pub id: TorServiceId,
    pub onion_service_address: String,
    pub should_quit: bool,
    pub chat_list: ChatList,
    pub chats: HashMap<TorServiceId, Chat>,
    pub show_command_popup: bool,
    pub system_messages_scroll: usize,
    pub cursor_location: Option<(u16, u16)>,
    pub show_welcome_popup: bool,
    pub connection_context: Option<ConnectionContext>,
    pub message_colors: HashMap<TorServiceId, Color>,
}

impl AppContext {
    pub fn new(id: TorServiceId, onion_service_address: String) -> Self {
        Self {
            id,
            onion_service_address,
            should_quit: false,
            chat_list: ChatList::default(),
            chats: HashMap::default(),
            show_command_popup: false,
            system_messages_scroll: 0,
            cursor_location: None,
            show_welcome_popup: false,
            connection_context: None,
            message_colors: HashMap::new(),
        }
    }

    pub fn toggle_command_popup(&mut self) {
        self.show_command_popup = !self.show_command_popup;
    }

    pub fn toggle_welcome_popup(&mut self) {
        self.show_welcome_popup = !self.show_welcome_popup;
    }

    pub fn add_new_chat(&mut self, id: &TorServiceId) {
        self.chat_list.add(id);
        self.chats.insert(id.clone(), Chat::new(id));
        self.add_id(id.clone());
    }

    pub fn add_id(&mut self, id: TorServiceId) {
        let color = THEME
            .chat_message
            .message_id_colors
            .choose(&mut rand::thread_rng());
        self.message_colors.insert(id, *color.unwrap());
    }

    pub fn get_color(&self, id: &TorServiceId) -> Option<&Color> {
        self.message_colors.get(id)
    }

    pub fn remove_id(&mut self, id: &TorServiceId) {
        self.message_colors.remove(id);
    }
}
