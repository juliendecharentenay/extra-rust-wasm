use super::*;


/// Bind the values of an array to the context using the key provided.
/// Assumes that the program has already been set.
/// Note: makes a copy of the array in place of the unsafe view
/// into the array...
pub fn bind(
    context: &web_sys::WebGl2RenderingContext,
    program: &web_sys::WebGlProgram,
    key: &str,
    array: &Vec<f32>
  ) -> Result<(), Error> {
    let buffer = context.create_buffer().ok_or("Unable to create buffer")?;
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    /*
    unsafe {
      let view = js_sys::Float32Array::view(array.as_slice());
      context.buffer_data_with_array_buffer_view(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &view,
        web_sys::WebGl2RenderingContext::STATIC_DRAW);
    }
    */
    let view = js_sys::Float32Array::new_with_length(array.len().try_into()?);
    view.copy_from(array.as_slice());
    context.buffer_data_with_array_buffer_view(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &view,
        web_sys::WebGl2RenderingContext::STATIC_DRAW);

    let position = context.get_attrib_location(program, key);
    context.vertex_attrib_pointer_with_i32(position.try_into()?, 3, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position as u32);
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    Ok(())
}

// Convert an array of `nalgebra::Point3<f32>` into a `Vec<f32>`
pub fn point3_to_vecf32(array: &Vec<&nalgebra::Point3<f32>>) -> Vec<f32> {
    array.iter()
    .map(|p| vec!(p.x, p.y, p.z))
    .fold(Vec::new(), |mut r, mut a| {r.append(&mut a); r})
}

// Convert an array of `nalgebra::Vector3<f32>` into a `Vec<f32>`
pub fn vector3_to_vecf32(array: &Vec<&nalgebra::Vector3<f32>>) -> Vec<f32> {
    array.iter()
    .map(|p| vec!(p.x, p.y, p.z))
    .fold(Vec::new(), |mut r, mut a| {r.append(&mut a); r})
}
