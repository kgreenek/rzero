#[no_mangle]
pub extern fn new_public_function() -> i64 {
  2
}

#[no_mangle]
pub extern fn print_array_len(input: &[f32]) {
  println!("Array len: {}", input.len());
}

#[no_mangle]
pub extern fn something(input_ptr: *mut f32, length: i32) -> f32 {
  unsafe {
    let input = std::slice::from_raw_parts_mut(input_ptr, length as usize);
    println!("input[0] {} intput[1] {}", input[0], input[1]);
    return input[0] + input[1];
  }
}

#[no_mangle]
pub extern fn extract_pitch_raw(input_ptr: *mut f32, output_ptr: *mut f32, length: i32,
                                input_channels: i32, output_channels: i32,
                                sample_rate: f64) -> f32 {
  unsafe {
    let input = std::slice::from_raw_parts_mut(input_ptr, (input_channels * length) as usize);
    let mut output = std::slice::from_raw_parts_mut(output_ptr,
        (output_channels * length) as usize);
    return extract_pitch(input, output, length, input_channels, output_channels, sample_rate);
  }
}

#[no_mangle]
pub extern fn extract_pitch(input: &[f32], output: &mut [f32], length: i32, input_channels: i32,
                            output_channels: i32, sample_rate: f64) -> f32 {
  println!("Audio sample_rate: {} length: {}", sample_rate, length);
  println!("Audio input: len: {} channels: {}", input.len(), input_channels);
  println!("Audio output: len: {} channels: {}", output.len(), output_channels);
  let mut sum = 0.0;
  for value in input {
    sum += value.abs();
  }
  return sum;
}
