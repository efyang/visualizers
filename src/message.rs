// used by both audio_input/updater.rs and instance.rs

pub enum UpdateMessage {
    // id, index
    Destroy(usize, usize),
    // id, old index, new index
    ChangeMapping(usize, usize, usize),
    // id, index
    Add(usize, usize),
}
