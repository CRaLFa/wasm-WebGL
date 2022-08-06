use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ WebGl2RenderingContext, WebGlShader, WebGlProgram };

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap()
        .document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vertex_shader = create_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, include_str!("shader/vertex_shader.glsl"))?;
    let fragment_shader = create_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, include_str!("shader/fragment_shader.glsl"))?;

    let program = create_program(&gl, &vertex_shader, &fragment_shader)?;
    gl.use_program(Some(&program));

    let pos_location = gl.get_attrib_location(&program, "position") as u32;
    let color_location = gl.get_attrib_location(&program, "color") as u32;

    let pos_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    let color_buffer = gl.create_buffer().ok_or("Failed to create buffer")?;

    // let vao = context.create_vertex_array().ok_or("Failed to create vertex array object")?;
    // context.bind_vertex_array(Some(&vao));

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
    gl.enable_vertex_attrib_array(pos_location);
    gl.vertex_attrib_pointer_with_i32(pos_location, VERTEX_SIZE as i32, WebGl2RenderingContext::FLOAT, false, 0, 0);

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    gl.enable_vertex_attrib_array(color_location);
    gl.vertex_attrib_pointer_with_i32(color_location, COLOR_SIZE as i32, WebGl2RenderingContext::FLOAT, false, 0, 0);

    const VERTEX_SIZE: usize = 3;
    const COLOR_SIZE: usize = 4;
    const VERTEX_COUNT: usize = 6;

    let vertices: [f32; VERTEX_SIZE * VERTEX_COUNT] = [
        -0.5, 0.5,  0.0,
        -0.5, -0.5, 0.0,
        0.5,  0.5,  0.0,
        -0.5, -0.5, 0.0,
        0.5,  -0.5, 0.0,
        0.5,  0.5,  0.0
    ];
    let colors: [f32; COLOR_SIZE * VERTEX_COUNT] = [
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0,
    ];

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
    unsafe {
        let pos_arr_buf_view = js_sys::Float32Array::view(&vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &pos_arr_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    unsafe {
        let color_arr_buf_view = js_sys::Float32Array::view(&colors);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_arr_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    // context.bind_vertex_array(Some(&vao));
    // context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    draw(&gl, VERTEX_COUNT as i32);

    Ok(())
}

fn create_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str
) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or_else(|| String::from("Failed to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Failed to compile shader")))
    }
}

fn create_program(
    gl: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or_else(|| String::from("Failed to create program object"))?;
    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_else(|| String::from("Failed to link program")))
    }
}

fn draw(gl: &WebGl2RenderingContext, vertex_count: i32) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertex_count);
    gl.flush();
}
