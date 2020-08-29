use crate::data::types::qpigs::QPIGS;
use std::sync::{Arc, LockResult, RwLock};


//Going to add a singleton golder for each piece of data
lazy_static! {
    static ref QPIGS_HOLDER: RwLock<Option<QPIGS>> = {
        RwLock::new(None)
    };
}

pub struct Holder;

impl Holder {
    pub fn get_qpigs() -> Option<QPIGS> {
        let qpigs_reader = match QPIGS_HOLDER.read() {
            Ok(reader) => reader,
            Err(e) => {
                error!("Unable to acquire write lock to QPIGS_HOLDER: {}", e);
                return None
            }
        };
        qpigs_reader.clone()
    }

    pub fn set_qpigs(input: QPIGS) {
        match QPIGS_HOLDER.write() {
            Ok(qpigs_writer) => {
                *qpigs_writer = Some(input);
            },
            Err(e) => {
                error!("Unable to acquire read lock to QPIGS_HOLDER: {}", e);
            }
        }
    }


}




