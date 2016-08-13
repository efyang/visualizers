// used by both audio_updater.rs and instance.rs

pub enum UpdateMessage {
    // id, index
    Destroy(usize, usize),
    // id, old index, new index
    ChangeMapping(usize, usize, usize),
}


