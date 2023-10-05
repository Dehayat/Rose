use sdl2;

pub enum EventType{
    EmptyEvent,
    ExitEvent,
}

pub(crate) struct Event{
    pub event_type:EventType,
}

pub(crate) struct EventSystem{
    pub event_pump: sdl2::EventPump
}

impl EventSystem{
    pub fn get_event(&mut self) ->Option<Event>{
        if let Some(sdl_event) = self.event_pump.poll_event(){
            Some(Self::create_event_from_sdl_event(sdl_event))
        }
        else{
            None
        }
    }
    pub fn create_event_from_sdl_event(sdl_event: sdl2::event::Event)->Event{
        use sdl2::event::Event as SdlEvent;
        match sdl_event {
            SdlEvent::Quit { .. } => Event { event_type: EventType::ExitEvent },
            _ => Event { event_type: EventType::EmptyEvent }
        }
    }
}