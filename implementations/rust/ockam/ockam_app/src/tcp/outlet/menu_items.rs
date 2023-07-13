use crate::tcp::outlet::create::tcp_outlet_create;
use crate::{AppHandle, Result};
use ockam_command::{CommandGlobalOpts, GlobalArgs};
use tauri::CustomMenuItem;

pub const TCP_OUTLET_HEADER_MENU_ID: &str = "tcp_outlet_header";
pub const TCP_OUTLET_CREATE_MENU_ID: &str = "tcp_outlet_create";

#[derive(Clone)]
pub struct TcpOutletActions {
    pub options: CommandGlobalOpts,
    pub(crate) menu_items: Vec<CustomMenuItem>,
}

impl TcpOutletActions {
    pub fn new() -> TcpOutletActions {
        let header = CustomMenuItem::new(TCP_OUTLET_HEADER_MENU_ID, "TCP Outlets").disabled();
        let create = CustomMenuItem::new(TCP_OUTLET_CREATE_MENU_ID, "Create...");
        let menu_items = vec![header, create];
        let opts = CommandGlobalOpts::new(GlobalArgs::default());
        TcpOutletActions {
            options: opts,
            menu_items,
        }
    }

    pub fn full() -> Result<TcpOutletActions> {
        let mut s = TcpOutletActions::new();
        let mut tcp_outlets = super::tcp_outlet_list()?
            .iter()
            .map(|outlet| {
                let outlet_info = format!(
                    "{} to {}",
                    outlet.worker_address().unwrap(),
                    outlet.tcp_addr
                );
                CustomMenuItem::new(outlet_info.clone(), outlet_info)
            })
            .collect::<Vec<CustomMenuItem>>();
        s.menu_items.append(&mut tcp_outlets);
        Ok(s)
    }
}

/// Event listener for the "Create..." menu item
pub fn on_create(app_handle: AppHandle) -> tauri::Result<()> {
    tcp_outlet_create(app_handle);
    Ok(())
}