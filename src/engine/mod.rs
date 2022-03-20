use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;

pub use photon::Photon;

pub trait Engine {
  // 对engine 按照 specs 进行一系列有序的处理
  fn apply(&mut self, specs: &[Spec]);

  // 从engine中生成目标图片，注意这里使用的是self，而非self的引用
  fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}


pub trait SpecTransform<T> {
  fn transform(&mut self, op: T);
}
