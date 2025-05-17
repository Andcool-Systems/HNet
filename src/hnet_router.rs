use crate::hnet_addr::HNetAddr;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub struct HNetRouter {
    pub self_address: HNetAddr,
    pub routing_table: HashMap<String, String>,
    pub interface_table: HashMap<String, Weak<RefCell<HNetRouter>>>,
}

impl HNetRouter {
    pub fn new(self_address: HNetAddr) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(HNetRouter {
            self_address,
            routing_table: HashMap::new(),
            interface_table: HashMap::new(),
        }))
    }

    pub fn add_route(&mut self, prefix: HNetAddr, next_hop: &str) {
        self.routing_table
            .insert(prefix.to_string(), next_hop.to_string());
    }

    pub fn connect(&mut self, interface: &str, target_router: &Rc<RefCell<HNetRouter>>) {
        self.interface_table
            .insert(interface.to_string(), Rc::downgrade(target_router));
    }

    pub fn get_router(&self, hop: &str) -> Option<Rc<RefCell<HNetRouter>>> {
        if let Some(next_router) = self.interface_table.get(hop) {
            next_router.upgrade()
        } else {
            eprint!(
                "[{}] has no connected router for {}",
                self.self_address.to_string(),
                hop
            );
            None
        }
    }

    pub fn route_to(&self, destination: &HNetAddr) {
        let prefixes = destination.generate_prefixes();
        if self.self_address.to_string().eq(prefixes.get(1).unwrap()) {
            println!(
                "[{}] ROUTE FINISHED! {} in my network!",
                self.self_address.to_string(),
                destination.to_string()
            );
            return;
        }

        for prefix in prefixes {
            if let Some(hop) = self.routing_table.get(&prefix) {
                if let Some(next_router) = self.get_router(hop) {
                    println!(
                        "{} -> {}",
                        self.self_address.to_string(),
                        next_router.borrow_mut().self_address.to_string()
                    );
                    next_router.borrow_mut().route_to(destination);
                    return;
                }
            }
        }
        if let Some(wan) = self.routing_table.get("*:*:*:*:*") {
            if let Some(next_router) = self.get_router(&wan) {
                println!(
                    "{} -> {}",
                    self.self_address.to_string(),
                    next_router
                        .try_borrow_mut()
                        .unwrap_or_else(|_| panic!("Loop detected!"))
                        .self_address
                        .to_string()
                );
                next_router.borrow_mut().route_to(destination);
                return;
            }
        }

        eprintln!(
            "[{}] has no matching route for {}!",
            self.self_address.to_string(),
            destination.to_string()
        );
    }
}
