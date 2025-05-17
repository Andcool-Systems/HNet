use hnet_addr::HNetAddr;
use hnet_router::HNetRouter;
mod hnet_addr;
mod hnet_router;

fn main() {
    // ------------------------ Create MGTS network -----------------------------
    let subnet_router_mgts_1 = HNetRouter::new(HNetAddr::parse("EU:RU:MGTS:045A:*").unwrap());
    let subnet_router_mgts_2 = HNetRouter::new(HNetAddr::parse("EU:RU:MGTS:1C2F:*").unwrap());

    // ------------------------- Create main MGTS router ------------------------
    let router_mgts_1 = HNetRouter::new(HNetAddr::parse("EU:RU:MGTS:*:*").unwrap());

    // Add wan route
    router_mgts_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // Create route table for root MGTS router
    router_mgts_1
        .borrow_mut()
        .add_route(HNetAddr::parse("EU:RU:MGTS:045A:*").unwrap(), "eth-1");

    router_mgts_1
        .borrow_mut()
        .add_route(HNetAddr::parse("EU:RU:MGTS:1C2F:*").unwrap(), "eth-2");

    // ------------------------- Physically connect MGTS subnets routers --------
    router_mgts_1
        .borrow_mut()
        .connect("eth-1", &subnet_router_mgts_1);
    router_mgts_1
        .borrow_mut()
        .connect("eth-2", &subnet_router_mgts_2);

    // ----------------- Reverse connect subnets to MGTS router -----------------
    subnet_router_mgts_1
        .borrow_mut()
        .connect("wan", &router_mgts_1);
    subnet_router_mgts_2
        .borrow_mut()
        .connect("wan", &router_mgts_1);

    // Create route table for MGTS subnets
    subnet_router_mgts_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    subnet_router_mgts_2
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // ------------------------- Create RTK network -----------------------------
    let subnet_router_rtk_1 = HNetRouter::new(HNetAddr::parse("EU:RU:RTK:01BF:*").unwrap());
    let subnet_router_rtk_2 = HNetRouter::new(HNetAddr::parse("EU:RU:RTK:6A0C:*").unwrap());

    let router_rtk_1 = HNetRouter::new(HNetAddr::parse("EU:RU:RTK:*:*").unwrap());

    // Add wan route
    router_rtk_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // ----------------- Physically connect RTK subnets routers ------------------
    router_rtk_1
        .borrow_mut()
        .connect("eth-1", &subnet_router_rtk_1);
    router_rtk_1
        .borrow_mut()
        .connect("eth-2", &subnet_router_rtk_2);

    // Create RTK route table
    router_rtk_1
        .borrow_mut()
        .add_route(HNetAddr::parse("EU:RU:RTK:01BF:*").unwrap(), "eth-1");

    router_rtk_1
        .borrow_mut()
        .add_route(HNetAddr::parse("EU:RU:RTK:6A0C:*").unwrap(), "eth-2");

    // --------------- Reverse connect subnets to RTK router ---------------------
    subnet_router_rtk_1
        .borrow_mut()
        .connect("wan", &router_rtk_1);

    subnet_router_rtk_2
        .borrow_mut()
        .connect("wan", &router_rtk_1);

    // Create RTK subnets routers route table
    subnet_router_rtk_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    subnet_router_rtk_2
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // ------------------------ CREATE MAIN RU ROUTER ----------------------------
    let router_ru = HNetRouter::new(HNetAddr::parse("EU:RU:*:*:*").unwrap());
    router_ru.borrow_mut().connect("eth-1", &router_mgts_1);
    router_ru.borrow_mut().connect("eth-2", &router_rtk_1);

    router_ru
        .borrow_mut()
        .add_route(HNetAddr::parse("EU:RU:MGTS:*:*").unwrap(), "eth-1");
    router_ru
        .borrow_mut()
        .add_route(HNetAddr::parse("EU:RU:RTK:*:*").unwrap(), "eth-2");

    // Reverse connect regional routers
    router_mgts_1.borrow_mut().connect("wan", &router_ru);
    router_rtk_1.borrow_mut().connect("wan", &router_ru);

    // ---------------------------------------------------------------------------------
    // -------------------------------- US network ------------------------------------

    let subnet_router_atnt_1 = HNetRouter::new(HNetAddr::parse("NA:US:NTNT:5723:*").unwrap());
    let subnet_router_atnt_2 = HNetRouter::new(HNetAddr::parse("NA:US:NTNT:8824:*").unwrap());

    // ------------------------- Create main ATnT router ------------------------
    let router_atnt_1 = HNetRouter::new(HNetAddr::parse("NA:US:NTNT:*:*").unwrap());

    // Add wan route
    router_atnt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // Create route table for root ATnT router
    router_atnt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("NA:US:NTNT:5723:*").unwrap(), "eth-1");

    router_atnt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("NA:US:NTNT:8824:*").unwrap(), "eth-2");

    // ------------------------- Physically connect ATnT subnets routers --------
    router_atnt_1
        .borrow_mut()
        .connect("eth-1", &subnet_router_atnt_1);
    router_atnt_1
        .borrow_mut()
        .connect("eth-2", &subnet_router_atnt_2);

    // ----------------- Reverse connect subnets to ATnT router -----------------
    subnet_router_atnt_1
        .borrow_mut()
        .connect("wan", &router_atnt_1);
    subnet_router_atnt_2
        .borrow_mut()
        .connect("wan", &router_atnt_1);

    // Create route table for ATnT subnets
    subnet_router_atnt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    subnet_router_atnt_2
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // ------------------------- Create ALT network -----------------------------
    let subnet_router_alt_1 = HNetRouter::new(HNetAddr::parse("NA:US:ALT:0C11:*").unwrap());
    let subnet_router_alt_2 = HNetRouter::new(HNetAddr::parse("NA:US:ALT:AB14:*").unwrap());

    let router_alt_1 = HNetRouter::new(HNetAddr::parse("NA:US:ALT:*:*").unwrap());

    // Add wan route
    router_alt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // ----------------- Physically connect ALT subnets routers ------------------
    router_alt_1
        .borrow_mut()
        .connect("eth-1", &subnet_router_alt_1);
    router_alt_1
        .borrow_mut()
        .connect("eth-2", &subnet_router_alt_2);

    // Create ALT route table
    router_alt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("NA:US:ALT:0C11:*").unwrap(), "eth-1");

    router_alt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("NA:US:ALT:AB14:*").unwrap(), "eth-2");

    // --------------- Reverse connect subnets to ALT router ---------------------
    subnet_router_alt_1
        .borrow_mut()
        .connect("wan", &router_alt_1);

    subnet_router_alt_2
        .borrow_mut()
        .connect("wan", &router_alt_1);

    // Create alt subnets routers route table
    subnet_router_alt_1
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    subnet_router_alt_2
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // ------------------------ CREATE MAIN US ROUTER ----------------------------
    let router_usa = HNetRouter::new(HNetAddr::parse("NA:US:*:*:*").unwrap());
    router_usa.borrow_mut().connect("eth-1", &router_atnt_1);
    router_usa.borrow_mut().connect("eth-2", &router_alt_1);

    router_usa
        .borrow_mut()
        .add_route(HNetAddr::parse("NA:US:NTNT:*:*").unwrap(), "eth-1");
    router_usa
        .borrow_mut()
        .add_route(HNetAddr::parse("NA:US:ALT:*:*").unwrap(), "eth-2");

    // Reverse connect regional routers
    router_atnt_1.borrow_mut().connect("wan", &router_usa);
    router_alt_1.borrow_mut().connect("wan", &router_usa);

    // ------------------------------- BACKBONE! -----------------------------------
    router_ru.borrow_mut().connect("wan", &router_usa);
    router_usa.borrow_mut().connect("wan", &router_ru);

    router_ru
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    router_usa
        .borrow_mut()
        .add_route(HNetAddr::parse("*:*:*:*:*").unwrap(), "wan");

    // Try to route
    subnet_router_mgts_1
        .borrow_mut()
        .route_to(&HNetAddr::parse("NA:US:NTNT:5723:0C8F").unwrap());
}
