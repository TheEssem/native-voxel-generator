#[macro_use]
extern crate neon;
extern crate neon_runtime;
extern crate voxel_worldgen;
extern crate rand;
extern crate nalgebra;
extern crate noise;

use voxel_worldgen::generators;
use voxel_worldgen::rnd::RngBuilder;

use nalgebra::Point2;

//use neon_runtime::call;
//use neon_runtime::primitive::integer;
use neon::result::JsResult;
use neon::types::JsObject;
use neon::prelude::*;
use neon::handle::Handle;
use neon::object::Object;

fn js_generate_chunk(call: Call) -> JsResult<JsBuffer> {
  let scope = call.scope;

  let seed = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
  let mut seed_rng = RngBuilder::init().mix(seed as u64).build();
  let world_gen_state = generators::vanilla::WorldGeneratorState::new(&mut seed_rng);

  let x = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>());
  let y = try!(try!(call.arguments.require(scope, 2)).check::<JsInteger>());
  let chunk_pos = Point2::new(x.value() as i32, y.value() as i32);

  let chunk = generators::vanilla::generate_chunk(&world_gen_state, chunk_pos);

  let data: Handle<JsBuffer> = try!(JsBuffer::new(scope, chunk.data.len() as u32));

  for (i, v) in chunk.data.iter().enumerate() {
      try!(data.set(i as u32, JsInteger::new(scope, *v as i32)));
  }

  Ok(data)
}

register_module!(m, {
    m.export("nativeGenerateChunk", js_generate_chunk)
});
