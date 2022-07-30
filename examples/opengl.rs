#![allow(non_snake_case)]

//! Example: Basic Tailwind usage
//!
//! This example shows how an app might be styled with TailwindCSS.
//!
//! To minify your tailwind bundle, currently you need to use npm. Follow these instructions:
//!
//!     https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9

use std::ffi::CStr;
use std::rc::Rc;
use std::time::Duration;
use futures_util::StreamExt;
use glutin::{Context, ContextBuilder, ContextWrapper, NotCurrent, PossiblyCurrent, RawContext};

use dioxus::prelude::*;
use dioxus_desktop::{DesktopContext, use_window};
use dioxus_desktop::raw_window_handle::RawWindowHandle;
use dioxus_desktop::tao::dpi::{PhysicalPosition, PhysicalSize};
use dioxus_desktop::tao::event::Rectangle;

pub mod gl {
    #![allow(
    non_camel_case_types
    )]

    // Generated in build script
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
    0.0,  0.5,  0.0,  1.0,  0.0,
    0.5, -0.5,  0.0,  0.0,  1.0,
];

const VS_SRC: &[u8] = b"
    #version 330 core

    in vec2 position;
    in vec3 color;

    out vec3 v_color;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_color = color;
    }
\0";

const FS_SRC: &[u8] = b"
    #version 330 core

    in vec3 v_color;

    void main() {
        gl_FragColor = vec4(v_color, 1.0);
    }
\0";


struct State {
    context: ContextWrapper<PossiblyCurrent, ()>,
    gl: gl::Gl,
}

impl State {

    pub fn new(desktop_context: &DesktopContext) -> State {
        let raw_context = State::create_context(desktop_context);
        let context = unsafe { raw_context.make_current() }.unwrap();

        let gl = State::load_opengl_functions(&*context);

        let version = unsafe {
            let data = CStr::from_ptr(gl.GetString(gl::VERSION) as * const _)
                .to_bytes().to_vec();

            String::from_utf8(data).unwrap()
        };

        println!("Using OpenGL {}", version);

        unsafe {
            let vs = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl.CompileShader(vs);
            println!("Compiled vertex shader");

            let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl.CompileShader(fs);
            println!("Compiled fragment shader");

            let program = gl.CreateProgram();
            gl.AttachShader(program, vs);
            gl.AttachShader(program, fs);
            gl.LinkProgram(program);
            gl.UseProgram(program);
            println!("Linked shader program");

            let mut vb = std::mem::zeroed();
            gl.GenBuffers(1, &mut vb);
            gl.BindBuffer(gl::ARRAY_BUFFER, vb);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            println!("Created vertex buffer");

            if gl.BindVertexArray.is_loaded() {
                let mut vao = std::mem::zeroed();
                gl.GenVertexArrays(1, &mut vao);
                gl.BindVertexArray(vao);
            }
            println!("Created vertex array");

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
            println!("Configured vertex attributes");
        }

        State {
            context, gl
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.ClearColor(0.0, 0.0, 0.0, 0.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
            self.context.swap_buffers().unwrap();

            println!("Draw");
        }
    }

    #[cfg(target_os = "windows")]
    fn create_context(desktop_context: &DesktopContext) -> RawContext<NotCurrent> {
        use glutin::platform::windows::RawContextExt;

        let hwnd;
        match desktop_context.window.0 {
            RawWindowHandle::Win32(handle) => {
                hwnd = handle.hwnd;
            },
            _ => unreachable!()
        }

        unsafe {
            ContextBuilder::new()
                .build_raw_context(hwnd)
                .unwrap()
        }
    }

    #[cfg(target_os = "linux")]
    fn create_context(desktop_context: &DesktopContext) -> AtlasResult<RawContext<NotCurrent>> {
        unimplemented!()
    }

    fn load_opengl_functions(context: &Context<PossiblyCurrent>) -> gl::Gl {
        gl::Gl::load_with(|ptr| context.get_proc_address(ptr) as *const _)
    }
}

fn main() {
    dioxus_desktop::launch_cfg(app, |c| {
        c
            .with_window(|cfg| {
                cfg
                    .with_inner_size(PhysicalSize::new(1024.0, 768.0))
                    .with_decorations(true)
                    .with_title("OpenGL example")
            })
            .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string())
    });
}

fn app(cx: Scope) -> Element {
    /// Must be outside the hook because of borrowing issues
    let context = use_window(&cx);
    context.set_bounds(Rectangle {
        position: PhysicalPosition::new(600.0, 0.0),
        size: PhysicalSize::new(1024.0, 768.0)
    });

    cx.use_hook(|| {
        cx.provide_context(Rc::new(State::new(&context)))
    });

    cx.render(rsx! {
        Child {}
    })
}

fn Child(cx: Scope) -> Element {
    let renderer = use_coroutine(&cx, |mut rx: UnboundedReceiver<Rc<State>>| async move {
        while let Some(state) = rx.next().await {
            state.draw();
        }
    });

    println!("Rendered GUI");
    cx.render(rsx! {
        div {
            class: "flex h-screen items-center justify-center px-4 bg-transparent",
            div {
                class: "max-w-sm overflow-hidden rounded-xl bg-white shadow-md duration-200 hover:scale-105 hover:shadow-xl",
                img {
                    class: "h-auto w-full",
                    alt: "plant",
                    src: "https://i.imgur.com/5dmBrx6.jpg"
                }

                div {
                    class: "p-5",
                    p {
                        class: "text-medium mb-5 text-gray-700",
                        "Lorem ipsum dolor sit, amet consectetur adipisicing elit. In autem ipsa, nulla laboriosam dolores, repellendus perferendis libero suscipit nam temporibus molestiae."
                    }

                    button {
                        onclick: move |_| renderer.send(cx.consume_context::<Rc<State>>().unwrap().clone()),
                        class: "w-full rounded-md bg-indigo-600 py-2 text-indigo-100 hover:bg-indigo-500 hover:shadow-md duration-75",
                        "Click Me"
                    }
                }
            }
        }
    })
}
