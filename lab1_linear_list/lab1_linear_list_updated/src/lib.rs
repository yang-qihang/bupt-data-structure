// shared helpers for terminal output (clear screen, page titles)
pub mod ui;

// the sequential and linked list implementation of the city list
mod city_list;

// the sequential and linked list implementation of the josephus problem
mod josephus;

// Here for the sake of cleaner api,
// we re-export the four modules in a different hierarchy.
pub mod city_list_linked {
    pub use crate::city_list::linked::*;
}

pub mod city_list_sequential {
    pub use crate::city_list::sequential::*;
}

pub mod josephus_linked {
    pub use crate::josephus::linked::*;
}

pub mod josephus_sequential {
    pub use crate::josephus::sequential::*;
}
