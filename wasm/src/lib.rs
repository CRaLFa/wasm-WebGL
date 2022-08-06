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
    let context = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vertex_shader = create_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es

        in vec3 position;
        in vec4 color;

        out vec4 vertexColor;

        void main() {
            gl_Position = vec4(position, 1.0);
            vertexColor = color;
        }
        "#,
    )?;
    let fragment_shader = create_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es

        precision highp float;

        in vec4 vertexColor;
        out vec4 fragmentColor;

        void main() {
            fragmentColor = vertexColor;
        }
        "#,
    )?;

    let program = create_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    let pos_location = context.get_attrib_location(&program, "position") as u32;
    let color_location = context.get_attrib_location(&program, "color") as u32;

    let pos_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    let color_buffer = context.create_buffer().ok_or("Failed to create buffer")?;

    // let vao = context.create_vertex_array().ok_or("Failed to create vertex array object")?;
    // context.bind_vertex_array(Some(&vao));

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
    context.enable_vertex_attrib_array(pos_location);
    context.vertex_attrib_pointer_with_i32(pos_location, VERTEX_SIZE as i32, WebGl2RenderingContext::FLOAT, false, 0, 0);

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    context.enable_vertex_attrib_array(color_location);
    context.vertex_attrib_pointer_with_i32(color_location, COLOR_SIZE as i32, WebGl2RenderingContext::FLOAT, false, 0, 0);

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

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
    unsafe {
        let pos_arr_buf_view = js_sys::Float32Array::view(&vertices);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &pos_arr_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
    unsafe {
        let color_arr_buf_view = js_sys::Float32Array::view(&colors);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_arr_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    // context.bind_vertex_array(Some(&vao));
    // context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    draw(&context, VERTEX_COUNT as i32);

    Ok(())
}

fn create_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str
) -> Result<WebGlShader, String> {
    let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Failed to create shader object"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Failed to compile shader")))
    }
}

fn create_program(
    context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context.create_program().ok_or_else(|| String::from("Failed to create program object"))?;
    context.attach_shader(&program, &vertex_shader);
    context.attach_shader(&program, &fragment_shader);
    context.link_program(&program);

    if context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Failed to link program")))
    }
}

fn draw(context: &WebGl2RenderingContext, vertex_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertex_count);
    context.flush();
}
