use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::fd::{AsRawFd, BorrowedFd, FromRawFd};

use memmap2::MmapMut;
use wayland_client::protocol::{
    wl_buffer, wl_compositor, wl_registry, wl_shm, wl_shm_pool, wl_surface,
};
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

struct State {
    compositor: Option<wl_compositor::WlCompositor>,
    shm: Option<wl_shm::WlShm>,
    wm_base: Option<xdg_wm_base::XdgWmBase>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == "wl_compositor" {
                let compositor =
                    proxy.bind::<wl_compositor::WlCompositor, _, _>(name, version, qhandle, ());

                state.compositor = Some(compositor);
                println!("Bound wl_compositor");
            }
            if interface == "wl_shm" {
                let shm = proxy.bind::<wl_shm::WlShm, _, _>(name, version, qhandle, ());
                state.shm = Some(shm);
                println!("Bound wl_shm");
            }
            if interface == "xdg_wm_base" {
                let wm_base =
                    proxy.bind::<xdg_wm_base::XdgWmBase, _, _>(name, version, qhandle, ());
                state.wm_base = Some(wm_base);
                println!("Bound wl_shm");
            }
        }
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: <wl_compositor::WlCompositor as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // No events to handle
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_surface::WlSurface,
        _: <wl_surface::WlSurface as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // No events to handle
    }
}

impl Dispatch<wl_shm::WlShm, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_shm::WlShm,
        _: <wl_shm::WlShm as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // No events to handle
    }
}

impl Dispatch<wl_buffer::WlBuffer, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_buffer::WlBuffer,
        event: <wl_buffer::WlBuffer as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wl_buffer::Event::Release = event {
            println!("Buffer released by compositor");
        }
    }
}

impl Dispatch<wl_shm_pool::WlShmPool, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_shm_pool::WlShmPool,
        _: <wl_shm_pool::WlShmPool as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        // No events to handle
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for State {
    fn event(
        _: &mut Self,
        wm: &xdg_wm_base::XdgWmBase,
        event: <xdg_wm_base::XdgWmBase as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm.pong(serial);
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for State {
    fn event(
        _: &mut Self,
        _: &xdg_surface::XdgSurface,
        event: <xdg_surface::XdgSurface as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = event {
            println!("Ack configure");
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for State {
    fn event(
        _: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        event: <xdg_toplevel::XdgToplevel as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_toplevel::Event::Close = event {
            println!("Window close requested");
            std::process::exit(0);
        }
    }
}

fn create_shm_file(size: usize) -> File {
    let fd = unsafe { libc::memfd_create(b"wayland-shm\0".as_ptr() as *const i8, 0) };
    assert!(fd >= 0, "memfd_create failed");

    let file = unsafe { File::from_raw_fd(fd) };
    file.set_len(size as u64)
        .expect("Failed to resize shm file");

    file
}

fn main() {
    // 1. Connect to the Wayland display (the compositor).
    let conn = Connection::connect_to_env().expect("Failed to connect to compositor");
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let display = conn.display();
    let mut state = State {
        compositor: None,
        shm: None,
        wm_base: None,
    };

    display.get_registry(&qh, ());
    event_queue.roundtrip(&mut state).unwrap();

    let compositor = state.compositor.as_ref().expect("No compositor found");
    let shm = state.shm.as_ref().expect("No wl_shm found");
    let wm_base = state.wm_base.as_ref().expect("No wm_base found");

    let surface = compositor.create_surface(&qh, ());
    println!("Created a Wayland surface!");

    let xdg_surface = wm_base.get_xdg_surface(&surface, &qh, ());
    let toplevel = xdg_surface.get_toplevel(&qh, ());
    toplevel.set_title("Rust wayland example".to_string());
    surface.commit();

    let width: u32 = 200;
    let height: u32 = 200;
    let stride = width * 4;
    let size = (stride * height) as usize;

    let file = create_shm_file(size);
    let mut mmap = unsafe { MmapMut::map_mut(&file).expect("mmap failed") };

    for y in 0..height {
        for x in 0..width {
            let offset = (y * stride + x * 4) as usize;
            mmap[offset..offset + 4].copy_from_slice(&[0x00, 0x00, 0xFF, 0xFF]);
        }
    }

    let _mmap = ManuallyDrop::new(mmap);

    let pool = shm.create_pool(
        unsafe { BorrowedFd::borrow_raw(file.as_raw_fd()) },
        size as i32,
        &qh,
        (),
    );
    let buffer = pool.create_buffer(
        0,
        width as i32,
        height as i32,
        stride as i32,
        wl_shm::Format::Argb8888,
        &qh,
        (),
    );

    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    println!("Displayed a red box");

    // 5. Enter the event loop
    loop {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}
