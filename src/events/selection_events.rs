use tui::backend::Backend;

use crate::{
    instance::{queue::Queue, selection::Selection, InstanceRunable},
    state::handler::StateHandler,
};

use super::event::Event;

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
    fn play_queue(&mut self, instance: &mut Selection);
    fn move_up(instance: &mut Selection);
    fn move_down(instance: &mut Selection);
    fn move_to_top(instance: &mut Selection);
    fn move_to_bottom(instance: &mut Selection);
    fn add_to_top_of_queue(instance: &mut Selection);
    fn add_to_bottom_of_queue(instance: &mut Selection);
    fn remove_from_queue(instance: &mut Selection);
}

impl<B: Backend> SelectionActions for StateHandler<B> {
    fn play_queue(&mut self, instance: &mut Selection) {
        let mut queue = Queue::new(instance.queue.clone());
        queue.run(self);
    }

    fn move_up(instance: &mut Selection) {
        if let Some(selected) = instance.state.selected() {
            let amount = instance.content.len();
            if selected > 0 {
                instance.state.select(Some(selected - 1));
            } else {
                instance.state.select(Some(amount - 1));
            }
        }
    }

    fn move_down(instance: &mut Selection) {
        if let Some(selected) = instance.state.selected() {
            let amount = instance.content.len();
            if selected >= amount - 1 {
                instance.state.select(Some(0));
            } else {
                instance.state.select(Some(selected + 1));
            }
        }
    }

    fn move_to_top(instance: &mut Selection) {
        instance.state.select(Some(0));
    }
    fn move_to_bottom(instance: &mut Selection) {
        let amount = instance.content.len();
        instance.state.select(Some(amount - 1));
    }

    fn add_to_top_of_queue(instance: &mut Selection) {
        if let Some(selected) = instance.state.selected() {
            if let Some(item) = instance.content.get(selected) {
                if !instance.queue.contains(item) {
                    instance.queue.splice(0..0, vec![item.to_owned()]);
                }
            };
        }
    }

    fn add_to_bottom_of_queue(instance: &mut Selection) {
        if let Some(selected) = instance.state.selected() {
            if let Some(item) = instance.content.get(selected) {
                if !instance.queue.contains(item) {
                    instance.queue.push(item.to_owned())
                }
            };
        }
    }

    fn remove_from_queue(instance: &mut Selection) {
        if let Some(selected) = instance.state.selected() {
            if let Some(item) = instance.content.get(selected) {
                if let Some(index) = instance.queue.iter().position(|element| element == item) {
                    instance.queue.remove(index);
                }
            }
        }
    }
}

impl Event<Selection> for SelectionEvent {
    fn trigger<B: Backend>(&self, handler: &mut StateHandler<B>, instance: &mut Selection) {
        match self {
            SelectionEvent::PlayQueue => handler.play_queue(instance),
            SelectionEvent::MoveUp => StateHandler::<B>::move_up(instance),
            SelectionEvent::MoveDown => StateHandler::<B>::move_down(instance),
            SelectionEvent::MoveToTop => StateHandler::<B>::move_to_top(instance),
            SelectionEvent::MoveToBottom => StateHandler::<B>::move_to_bottom(instance),
            SelectionEvent::AddToTopOfQueue => StateHandler::<B>::add_to_top_of_queue(instance),
            SelectionEvent::AddToBottomOfQueue => {
                StateHandler::<B>::add_to_bottom_of_queue(instance)
            }
            SelectionEvent::RemoveFromQueue => StateHandler::<B>::remove_from_queue(instance),
        }
    }
}
