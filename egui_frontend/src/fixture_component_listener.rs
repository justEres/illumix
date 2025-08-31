use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use fixture_lib::{
    fixture::FixtureComponent,
    networking::{FixtureComponentUpdated, Packet, PacketType},
    universe::Universe,
};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebSocket;

use crate::websocket::send_packet;

type Callback = Box<dyn Fn(&FixtureComponent) -> ()>;

pub struct ListenerDatabase {
    callbacks: HashMap<(u8, u8), Vec<Callback>>,
}

impl ListenerDatabase {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
        }
    }

    pub fn add_listener(&mut self, fixture_id: u8, component_index: u8, callback: Callback) {
        let identifier = (fixture_id, component_index);
        if let Some(callbacks) = self.callbacks.get_mut(&identifier) {
            callbacks.push(callback);
        } else {
            self.callbacks.insert(identifier, vec![callback]);
        }
    }

    pub fn notify(&self, fixture_id: u8, component_index: u8, component: FixtureComponent) {
        let identifier = (fixture_id, component_index);
        if let Some(callbacks) = self.callbacks.get(&identifier) {
            for callback in callbacks {
                callback(&component);
            }
        }
    }

    pub fn notify_all(&self, universe: SharedState<Universe>) {
        let uni = universe.borrow();
        for fixture in uni.fixtures.iter() {
            for (index, fixture_component) in fixture.components.iter().enumerate() {
                self.notify(fixture.id, index as u8, fixture_component.clone());
            }
        }
    }
}

pub struct SharedState<T>(Rc<RefCell<T>>);

impl<T> SharedState<T> {
    pub fn new(value: T) -> Self {
        return SharedState(Rc::new(RefCell::new(value)));
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        return self.0.borrow();
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        return self.0.borrow_mut();
    }
}

impl<T> Clone for SharedState<T> {
    fn clone(&self) -> Self {
        SharedState(self.0.clone())
    }
}

pub struct ChangeEventManager {
    updates: HashMap<(u8, u8), FixtureComponent>,
}

impl ChangeEventManager {
    pub fn new() -> Self {
        Self {
            updates: HashMap::new(),
        }
    }

    pub fn create_event(
        &mut self,
        fixture_id: u8,
        component_index: u8,
        new_fixture_component: FixtureComponent,
    ) {
        self.updates
            .insert((fixture_id, component_index), new_fixture_component);
    }

    pub fn send_updates(&mut self, ws: WebSocket) {
        for ((fixture_id, component_index), component) in self.updates.drain() {
            //alert(&format!("fixture: {} component_index: {}, component: {:?}",fixture_id,component_index,component));
            let packet = Packet {
                packet_type: PacketType::FixtureComponentUpdated(FixtureComponentUpdated {
                    component,
                    fixture_id,
                    component_index,
                }),
            };
            send_packet(ws.clone(), packet);
            //web_sys::console::log_1(&"sent update".to_string().into());
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
