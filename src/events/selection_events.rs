use tui::backend::Backend;

use crate::{
    audio::init::init_audio_handler,
    instance::{queue::Queue, selection::Selection, Instance},
};

use super::{
    handler::{Event, EventHandler},
    queue_events::QueueEvent,
};

pub enum SelectionEvent {
    PlayQueue,
    MoveUp,
    MoveDown,
    MoveToTop,
    MoveToBottom,
    AddToTopOfQueue,
    AddToBottomOfQueue,
    RemoveFromQueue,
}

trait SelectionActions {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to_top(&mut self);
    fn move_to_bottom(&mut self);
    fn add_to_top_of_queue(&mut self);
    fn add_to_bottom_of_queue(&mut self);
    fn remove_from_queue(&mut self);
}

impl<B: Backend> SelectionActions for EventHandler<B, Selection> {
    fn move_up(&mut self) {
        if let Some(selected) = self.instance.state.selected() {
            let amount = self.instance.content.len();
            if selected > 0 {
                self.instance.state.select(Some(selected - 1));
            } else {
                self.instance.state.select(Some(amount - 1));
            }
        }
    }

    fn move_down(&mut self) {
        if let Some(selected) = self.instance.state.selected() {
            let amount = self.instance.content.len();
            if selected >= amount - 1 {
                self.instance.state.select(Some(0));
            } else {
                self.instance.state.select(Some(selected + 1));
            }
        }
    }

    fn move_to_top(&mut self) {
        self.instance.state.select(Some(0));
    }
    fn move_to_bottom(&mut self) {
        let amount = self.instance.content.len();
        self.instance.state.select(Some(amount - 1));
    }

    fn add_to_top_of_queue(&mut self) {
        if let Some(selected) = self.instance.state.selected() {
            if let Some(item) = self.instance.content.get(selected) {
                if !self.instance.queue.contains(item) {
                    self.instance.queue.splice(0..0, vec![item.to_owned()]);
                }
            };
        }
    }

    fn add_to_bottom_of_queue(&mut self) {
        if let Some(selected) = self.instance.state.selected() {
            if let Some(item) = self.instance.content.get(selected) {
                if !self.instance.queue.contains(item) {
                    self.instance.queue.push(item.to_owned())
                }
            };
        }
    }

    fn remove_from_queue(&mut self) {
        if let Some(selected) = self.instance.state.selected() {
            if let Some(item) = self.instance.content.get(selected) {
                if let Some(index) = self
                    .instance
                    .queue
                    .iter()
                    .position(|element| element == item)
                {
                    self.instance.queue.remove(index);
                }
            }
        }
    }
}

impl Event for SelectionEvent {
    fn trigger_selection<B: Backend>(&self, handler: &mut EventHandler<B, Selection>) {
        match self {
            SelectionEvent::PlayQueue => {
                // let (sink, _stream) = init_audio_handler().unwrap();
                // let queue = Queue::new(handler.instance.queue, sink);
                // handler.instance = queue;
                // handler.instance.run(*handler);
            }
            SelectionEvent::MoveUp => handler.move_up(),
            SelectionEvent::MoveDown => handler.move_down(),
            SelectionEvent::MoveToTop => handler.move_to_top(),
            SelectionEvent::MoveToBottom => handler.move_to_bottom(),
            SelectionEvent::AddToTopOfQueue => handler.add_to_top_of_queue(),
            SelectionEvent::AddToBottomOfQueue => handler.add_to_bottom_of_queue(),
            SelectionEvent::RemoveFromQueue => handler.remove_from_queue(),
        }
    }

    fn trigger_queue<B: Backend>(&self, _handler: &mut EventHandler<B, Queue>) {}
}
