pub enum Factions {
    SCKK,
    TOW,
}

pub enum Permissions {
    SaesLogin,
    SaesMaintenance,
    SaesUcp(Factions),
    SaesUcpHome(Factions),
    SaesUcpHelp(Factions),
    SaesUcpLinks(Factions),
    SaesUcpViewAll(Factions),
    SaesUcpPostAll(Factions),
    SaesUcpViewSupplements(Factions),
    SaesUcpViewHails(Factions),
    SaesUcpViewBills(Factions),
    SaesUcpAll(Factions),
    SaesUcpPostSupplements(Factions),
    SaesUcpPostHails(Factions),
    SaesUcpPostBills(Factions),
    SaesSm(Factions),
    SaesSmHome(Factions),
    SaesSmAll(Factions),
    SaesSmStatsAll(Factions),
    SaesSmStatsCurrent(Factions),
    SaesSmStatsPrevious(Factions),
    SaesSmStatsOlder(Factions),
    SaesSmViewAll(Factions),
    SaesSmPostAll(Factions),
    SaesSmViewSupplements(Factions),
    SaesSmViewHails(Factions),
    SaesSmViewBills(Factions),
    SaesSmPostSupplements(Factions),
    SaesSmPostHails(Factions),
    SaesSmPostBills(Factions),
}

pub fn get_perm(perm: Permissions) -> String {
    match perm {
        Permissions::SaesLogin => "saes.login".to_string(),
        Permissions::SaesMaintenance => "saes.maintenance".to_string(),
        Permissions::SaesSm(fact) => format!(
            "saes.sm.{}",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmAll(fact) => format!(
            "saes.sm.{}.all",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmHome(fact) => format!(
            "saes.sm.{}.home",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmPostAll(fact) => format!(
            "saes.sm.{}.post.all",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmPostBills(fact) => format!(
            "saes.sm.{}.post.bills",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmPostHails(fact) => format!(
            "saes.sm.{}.post.hails",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmPostSupplements(fact) => format!(
            "saes.sm.{}.post.supplements",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmStatsAll(fact) => format!(
            "saes.sm.{}.stats.all",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmStatsCurrent(fact) => format!(
            "saes.sm.{}.stats.current",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmStatsOlder(fact) => format!(
            "saes.sm.{}.stats.older",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmStatsPrevious(fact) => format!(
            "saes.sm.{}.stats.previous",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmViewAll(fact) => format!(
            "saes.sm.{}.view.all",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmViewBills(fact) => format!(
            "saes.sm.{}.view.bill",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmViewHails(fact) => format!(
            "saes.sm.{}.view.hails",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesSmViewSupplements(fact) => format!(
            "saes.sm.{}.view.supplements",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcp(fact) => format!(
            "saes.ucp.{}",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpAll(fact) => format!(
            "saes.ucp.{}.all",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpHelp(fact) => format!(
            "saes.ucp.{}.help",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpHome(fact) => format!(
            "saes.ucp.{}.home",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpLinks(fact) => format!(
            "saes.ucp.{}.links",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpPostAll(fact) => format!(
            "saes.ucp.{}.post.all",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpPostBills(fact) => format!(
            "saes.ucp.{}.post.bills",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpPostHails(fact) => format!(
            "saes.ucp.{}.post.hails",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpPostSupplements(fact) => format!(
            "saes.ucp.{}.post.supplements",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpViewAll(fact) => format!(
            "saes.ucp.{}.view.all",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpViewBills(fact) => format!(
            "saes.ucp.{}.view.bills",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpViewHails(fact) => format!(
            "saes.ucp.{}.view.hails",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcpViewSupplements(fact) => format!(
            "saes.ucp.{}.view.supplements",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
    }
}
