use sdl2::{self, EventPump, event::WindowEvent};

#[derive(Debug, PartialEq)]
pub enum EventType {
    ExitEvent,
    MouseEvent,
}

pub struct Event {
    pub event_type: EventType,
    pub sdl_event : sdl2::event::Event,
}

pub struct EventSystem {
    pub event_pump: sdl2::EventPump,
    event_queue: Vec<Event>,
}

impl EventSystem {
    pub fn new(event_pump: EventPump) -> EventSystem {
        EventSystem {
            event_pump,
            event_queue: vec![],
        }
    }

    pub fn create_event_from_sdl_event(sdl_event: sdl2::event::Event) -> Option<Event> {
        use sdl2::event::Event as SdlEvent;
        match sdl_event {
            SdlEvent::Window {win_event, .. } if win_event==WindowEvent::Close => Some(Event {
                event_type: EventType::ExitEvent,
                sdl_event,
            }),
            SdlEvent::Quit { .. } => Some(Event {
                event_type: EventType::ExitEvent,
                sdl_event,
            }),
            SdlEvent::MouseMotion { .. } => Some(Event {
                event_type: EventType::MouseEvent,
                sdl_event,
            }),
            SdlEvent::MouseButtonDown { .. } => Some(Event {
                event_type: EventType::MouseEvent,
                sdl_event,
            }),
            SdlEvent::MouseButtonUp { .. } => Some(Event {
                event_type: EventType::MouseEvent,
                sdl_event,
            }),
            _ => None,
        }
    }

    pub fn update(&mut self) {
        self.event_queue.clear();
        for sdl_event in self.event_pump.poll_iter() {
            let event = Self::create_event_from_sdl_event(sdl_event);
            if event.is_some() {
                self.event_queue.push(event.unwrap());
            }
        }
    }
    pub fn iter(&self) -> EventSystemIter {
        EventSystemIter {
            inner_iter: self.event_queue.iter(),
        }
    }

    ///Mainly for imgui
    pub fn get_mouse_state(&self) -> sdl2::mouse::MouseState {
        sdl2::mouse::MouseState::new(&self.event_pump)
    }
}
pub struct EventSystemIter<'a> {
    inner_iter: std::slice::Iter<'a, Event>,
}

impl<'a> Iterator for EventSystemIter<'a> {
    type Item = &'a Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next()
    }
}
