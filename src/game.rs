use crate::board::*;
use std::{
    collections::HashMap
};
use ethnum::*;
use nohash_hasher::*;

pub struct Game {
    pub current_board: Board,
    pub halfmove: u8,
    pub threefold_store: IntMap<BoardMove, u8>
}