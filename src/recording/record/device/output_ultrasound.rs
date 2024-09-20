use std::time::Duration;

use polars::prelude::DataFrame;

use crate::{error::EmulatorError, recording::record::transducer};

use super::DeviceRecord;
pub struct OutputUltrasound<'a> {
    pub(crate) inner: Vec<transducer::output_ultrasound::OutputUltrasound<'a>>,
}

impl<'a> OutputUltrasound<'a> {
    pub fn next(&mut self, duration: Duration) -> Result<DataFrame, EmulatorError> {
        let mut df = self.inner[0].next(duration)?;
        df.rename("p[a.u.]", "p_0[a.u.]".into()).unwrap();
        self.inner
            .iter_mut()
            .enumerate()
            .skip(1)
            .for_each(|(i, tr)| {
                let mut d = tr.next(duration).unwrap();
                d.rename("p[a.u.]", format!("p_{}[a.u.]", i).into())
                    .unwrap();
                let mut d = d.take_columns();
                let v = d.pop().unwrap();
                df.hstack_mut(&[v]).unwrap();
            });
        Ok(df)
    }
}

impl<'a> DeviceRecord<'a> {
    pub fn output_ultrasound(&'a self) -> OutputUltrasound<'a> {
        OutputUltrasound {
            inner: self.iter().map(|tr| tr.output_ultrasound()).collect(),
        }
    }
}
