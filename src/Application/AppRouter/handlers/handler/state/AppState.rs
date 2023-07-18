//Std
use std::sync::{Arc, Mutex};
//Handlers State

//Contains the App scoped state of the whole application
#[derive(Clone)]
pub struct State
{
}

impl State
{
    pub fn new() -> State
    {
	State
	{
	}
    }
}
