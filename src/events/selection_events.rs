use super::handler::{Event, EventHandler};

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
    fn play_queue(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_to_top(&mut self);
    fn move_to_bottom(&mut self);
    fn add_to_top_of_queue(&mut self);
    fn add_to_bottom_of_queue(&mut self);
    fn remove_from_queue(&mut self);
}

impl SelectionActions for EventHandler<'_> {
    fn play_queue(&mut self) {}

    fn move_up(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            if let Some(selected) = instance.state.selected() {
                let amount = instance.content.len();
                if selected > 0 {
                    instance.state.select(Some(selected - 1));
                } else {
                    instance.state.select(Some(amount - 1));
                }
            }
        }
    }

    fn move_down(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            if let Some(selected) = instance.state.selected() {
                let amount = instance.content.len();
                if selected >= amount - 1 {
                    instance.state.select(Some(0));
                } else {
                    instance.state.select(Some(selected + 1));
                }
            }
        }
    }

    fn move_to_top(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            instance.state.select(Some(0));
        }
    }
    fn move_to_bottom(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            let amount = instance.content.len();
            instance.state.select(Some(amount - 1));
        }
    }

    fn add_to_top_of_queue(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            if let Some(selected) = instance.state.selected() {
                if let Some(item) = instance.content.get(selected) {
                    if !instance.queue.contains(item) {
                        instance.queue.splice(0..0, vec![item.to_owned()]);
                    }
                };
            }
        }
    }

    fn add_to_bottom_of_queue(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            if let Some(selected) = instance.state.selected() {
                if let Some(item) = instance.content.get(selected) {
                    if !instance.queue.contains(item) {
                        instance.queue.push(item.to_owned())
                    }
                };
            }
        }
    }

    fn remove_from_queue(&mut self) {
        if let Some(instance) = self.selection_instance.as_mut() {
            if let Some(selected) = instance.state.selected() {
                if let Some(item) = instance.content.get(selected) {
                    if let Some(index) = instance.queue.iter().position(|element| element == item) {
                        instance.queue.remove(index);
                    }
                }
            }
        }
    }
}
impl Event for SelectionEvent {
    fn trigger(&self, handler: &mut EventHandler) {
        match self {
            SelectionEvent::PlayQueue => handler.play_queue(),
            SelectionEvent::MoveUp => handler.move_up(),
            SelectionEvent::MoveDown => handler.move_down(),
            SelectionEvent::MoveToTop => handler.move_to_top(),
            SelectionEvent::MoveToBottom => handler.move_to_bottom(),
            SelectionEvent::AddToTopOfQueue => handler.add_to_top_of_queue(),
            SelectionEvent::AddToBottomOfQueue => handler.add_to_bottom_of_queue(),
            SelectionEvent::RemoveFromQueue => handler.remove_from_queue(),
        }
    }
}
