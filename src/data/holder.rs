use crate::data::types::qpigs::QPIGS;
use std::sync::{RwLock};
use crate::data::types::qpiws::QPIWS;


//Going to add a singleton golder for each piece of data
lazy_static! {
    static ref QPIGS_HOLDER: RwLock<Option<QPIGS>> = {
        RwLock::new(None)
    };

    static ref QPIWS_HOLDER: RwLock<Option<QPIWS>> = {
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
            Ok(mut qpigs_writer) => {
                *qpigs_writer = Some(input);
            },
            Err(e) => {
                error!("Unable to acquire read lock to QPIGS_HOLDER: {}", e);
            }
        }
    }

    pub fn get_qpiws() -> Option<QPIWS> {
        let qpiws_reader = match QPIWS_HOLDER.read() {
            Ok(reader) => reader,
            Err(e) => {
                error!("Unable to acquire write lock to QPIWS_HOLDER: {}", e);
                return None
            }
        };
        qpiws_reader.clone()
    }

    pub fn set_qpiws(input: QPIWS) {
        match QPIWS_HOLDER.write() {
            Ok(mut qpiws_writer) => {
                *qpiws_writer = Some(input);
            },
            Err(e) => {
                error!("Unable to acquire read lock to QPIWS_HOLDER: {}", e);
            }
        }
    }


}




