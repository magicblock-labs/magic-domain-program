use crate::{consts::tags::FieldTag, state::field::Field};

pub struct FieldSerializer<'a> {
    index: usize,
    slice: &'a mut [u8],
}

impl<'a> FieldSerializer<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self { index: 0, slice }
    }

    pub fn write_field<F: Field>(&mut self, field: &F) {
        self.slice[self.index] = F::TAG;
        self.index += 1;
        field.serialize(&mut self.slice[self.index..]);
        self.index += field.size();
    }
}

pub struct FieldDeserializer<'a> {
    index: usize,
    slice: &'a [u8],
}

impl<'a> FieldDeserializer<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { index: 0, slice }
    }

    pub fn read_tag(&mut self) -> Option<FieldTag> {
        let index = self.index;
        self.index += 1;
        self.slice.get(index).copied()
    }

    pub fn read_field<F: Field>(&mut self) -> Option<F> {
        let slice = self.slice.get(self.index..)?;
        let field = F::deserialize(slice)?;
        self.index += field.size();
        Some(field)
    }
}
