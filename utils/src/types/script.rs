use super::*;
use crate::buffer::Buffer;
use crate::decode::*;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default)]
pub struct Script(AVector<Method>);

impl RawDecoder for Script {
    fn step_size(&self) -> usize {
        self.0.step_size()
    }

    fn decode<'a>(
        &mut self,
        buffer: &mut Buffer<'a>,
        stage: &DecodeStage,
    ) -> DecodeResult<DecodeStage> {
        match stage.step {
            step if step < self.step_size() => self.0.decode(buffer, stage),
            _ => Err(DecodeError::InternalError),
        }
    }
}
