use std::fmt::Debug;

use crate::TerminalMenu;

pub struct ClickCallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, T) -> Option<String>,
{
    pub callback: F,
    pub state: T,
}

pub struct InputCallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, String, T) -> Option<String>,
{
    pub callback: F,
    pub state: T,
}

pub trait IClickCallback: Debug {
    fn call(&self, context: &mut TerminalMenu) -> Option<String>;
}

pub trait IInputCallback: Debug {
    fn call(&self, context: &mut TerminalMenu, input: String) -> Option<String>;
}

impl<F, T> IClickCallback for ClickCallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, T) -> Option<String> + Clone,
    T: Clone,
{
    fn call(&self, context: &mut TerminalMenu) -> Option<String> {
        (self.callback.to_owned())(context, self.state.clone())
    }
}

impl<F, T> IInputCallback for InputCallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, String, T) -> Option<String> + Clone,
    T: Clone,
{
    fn call(&self, context: &mut TerminalMenu, input: String) -> Option<String> {
        (self.callback.to_owned())(context, input, self.state.clone())
    }
}

impl<F, T> Debug for ClickCallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, T) -> Option<String>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClickCallbackItem")
            .field("callback", &())
            .field("state", &())
            .finish()
    }
}

impl<F, T> Debug for InputCallbackItem<F, T>
where
    F: FnOnce(&mut TerminalMenu, String, T) -> Option<String>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputCallbackItem")
            .field("callback", &())
            .field("state", &())
            .finish()
    }
}
