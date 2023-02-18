use windows::Win32::{
    Devices::Display::SetDisplayConfig,
    Graphics::Gdi::{
        SDC_APPLY, SDC_TOPOLOGY_CLONE, SDC_TOPOLOGY_EXTEND, SDC_TOPOLOGY_EXTERNAL,
        SDC_TOPOLOGY_INTERNAL,
    },
};

static ERROR_MESSAGE: &str =
    "Please use one of the following arguments : /Clone, /Extend, /External, /Internal .";

fn main() {
    let binding = std::env::args().nth(1).unwrap_or_default().to_lowercase();
    let arg = binding.as_ref();
    unsafe {
        match arg {
            "/clone" => SetDisplayConfig(None, None, SDC_APPLY | SDC_TOPOLOGY_CLONE),
            "/extend" => SetDisplayConfig(None, None, SDC_APPLY | SDC_TOPOLOGY_EXTEND),
            "/external" => SetDisplayConfig(None, None, SDC_APPLY | SDC_TOPOLOGY_EXTERNAL),
            "/internal" => SetDisplayConfig(None, None, SDC_APPLY | SDC_TOPOLOGY_INTERNAL),
            _ => {
                println!("{ERROR_MESSAGE}");
                0
            }
        };
    }
}
