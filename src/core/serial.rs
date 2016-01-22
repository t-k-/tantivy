use core::global::*;
use core::schema::*;
use core::error::{Result, Error};
use std::fmt;


pub trait SegmentSerializer<Output> {
    fn new_term(&mut self, term: &Term, doc_freq: DocId) -> Result<()>;
    fn add_doc(&mut self, doc_id: DocId) -> Result<()>;
    fn close(self,) -> Result<Output>;
}

pub trait SerializableSegment {
    fn write<Output, SegSer: SegmentSerializer<Output>>(&self, serializer: SegSer) -> Result<Output>;
}


// change the API to remove the lifetime, by
// "pushing" the data to a SegmentSerializer.

pub struct DebugSegmentSerializer {
    text: String,
}

impl fmt::Debug for DebugSegmentSerializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl DebugSegmentSerializer {

    pub fn debug_string<S: SerializableSegment>(index: &S) -> String {
        let serializer = DebugSegmentSerializer::new();
        index.write(serializer).unwrap()
    }

    pub fn new() -> DebugSegmentSerializer {
        DebugSegmentSerializer {
            text: String::new(),
        }
    }
}

impl SegmentSerializer<String> for DebugSegmentSerializer {
    fn new_term(&mut self, term: &Term, doc_freq: DocId) -> Result<()> {
        self.text.push_str(&format!("{:?}\n", term));
        Ok(())
    }

    fn add_doc(&mut self, doc_id: DocId) -> Result<()> {
        self.text.push_str(&format!("   - Doc {:?}\n", doc_id));
        Ok(())
    }

    fn close(self,) -> Result<String> {
        Ok(self.text)
    }
}

pub fn serialize_eq<L: SerializableSegment, R: SerializableSegment>(left: &L, right: &R) -> bool{
    let str_left = DebugSegmentSerializer::debug_string(left);
    let str_right = DebugSegmentSerializer::debug_string(right);
    str_left == str_right
}


// TODO make iteration over Fields somehow sorted
