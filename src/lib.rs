extern crate sdl2;

#[cfg(feature = "hotload")]
extern crate dynamic_reload;
#[cfg(target_os="android")]
extern crate jni;
extern crate rusttype;


#[cfg(target_os="android")]
use jni::objects::JObject;
#[cfg(target_os="android")]
use jni::objects::JClass;
#[cfg(target_os="android")]
use jni::JNIEnv;
#[cfg(target_os="android")]
use jni::sys::jint;
#[cfg(target_os="android")]
use sdl2::libc::c_char;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod text;
mod game_tile;
mod clickable;
mod drawable;

use clickable::Clickable;
use drawable::Drawable;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SDL_main() -> i32 {
    rust_main();
    0
}

#[cfg(target_os="android")]
extern "C" {
    fn SDL_Android_Init(env: JNIEnv, cls: JClass);
    fn SDL_SetMainReady();
}

#[cfg(target_os="android")]
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_org_libsdl_app_SDLActivity_nativeInit(env: JNIEnv, cls: JClass, array: JObject) -> jint
{
    let mut i: i32;
    let mut argc: i32;
    let mut status: i32;
    let mut len: i32;
    let mut argv: *const *const c_char;

    /* This interface could expand with ABI negotiation, callbacks, etc. */
    SDL_Android_Init(env, cls);

    SDL_SetMainReady();

    /* Prepare the arguments. */
    status = SDL_main(/*argc, argv*/);

    return status;
}


pub fn rust_main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    video_subsys.text_input().start();

    let window = video_subsys.window("SDL2_TTF Example", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    canvas.clear();

    let tile = game_tile::GameTile::new(0, 0, 100, 100, "A");

    // Note: Defeats the purpose of having impl drawable and impl clickables
    let mut entities: Vec<game_tile::GameTile> = Vec::new();
    entities.push(tile);

    {
        let mut clickables = Vec::new();
        let mut drawables = Vec::new();

        clickables.push(entities.len() - 1);
        drawables.push(entities.len() - 1);

        canvas.present();

        'mainloop: loop {
            for event in sdl_context.event_pump().unwrap().poll_iter() {
                match event {
                    Event::MouseButtonUp {x, y, ..} => {
                        for clickable_index in &clickables {
                            let clickable = entities.get(*clickable_index).unwrap();
                            clickable.respond(x, y);
                        }
                    },
                    Event::KeyDown {keycode: Some(Keycode::Escape), ..} |
                    Event::Quit {..} => break 'mainloop,
                    Event::TextInput {text, ..} | Event::TextEditing {text, ..} => {
                        println!("text: {}", text);

                        if text == "q" {
                            break 'mainloop
                        }

                        let tile = game_tile::GameTile::new(150 * (drawables.len() as i32 % 4), 150 * (drawables.len() as i32 / 4), 100, 100, &text);

                        entities.push(tile);
                        clickables.push(entities.len() - 1);
                        drawables.push(entities.len() - 1);
                    },
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
            canvas.clear();
            for drawable_index in &drawables {
                let drawable = entities.get(*drawable_index).unwrap();
                canvas = drawable.draw(canvas);
            }
            canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}


static SCREEN_WIDTH : u32 = 800;
static SCREEN_HEIGHT : u32 = 600;
