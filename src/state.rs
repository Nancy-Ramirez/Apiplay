use std::sync::{Mutex, Arc};
use crate::modules::music::domain::Playlist;

pub struct State{
    pub playlist: Arc<Mutex<Vec<Playlist>>>
}