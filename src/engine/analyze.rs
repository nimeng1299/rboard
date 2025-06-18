#[derive(Default)]
pub struct Analyzes {
    pub datas: Vec<Analyze>,
}

impl Analyzes {
    pub fn from_string(str: &String) -> Self {
        let v = str.trim().split_whitespace().collect::<Vec<&str>>();

        let mut datas = Vec::new();
        let mut strs = Vec::new();
        for str in v {
            match str {
                "info" => {
                    if strs.len() > 0 {
                        datas.push(Analyze::from_strs(strs.clone()));
                        strs.clear();
                    }
                }
                _ => {
                    strs.push(str);
                }
            }
        }
        if strs.len() > 0 {
            datas.push(Analyze::from_strs(strs.clone()));
            strs.clear();
        }
        Self { datas }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Analyze {
    pub move_: String,
    pub visits: u64,
    pub utility: f64,
    pub winrate: f64,
    pub score_mean: f64,
    pub score_stdev: f64,
    pub score_lead: f64,
    pub score_selfplay: f64,
    pub prior: f64,
    pub lcb: f64,
    pub utility_lcb: f64,
    pub order: u64,
    pub pv: Vec<String>,
    pub pv_visits: u64,
}

impl Analyze {
    //info move K9 visits 1 utility 0.963494 winrate 0.981747
    //scoreMean 1.56209 scoreStdev 13.7542 scoreLead 1.56209
    //scoreSelfplay 1.56209 prior 0.0407438 lcb -0.0182531
    //utilityLcb -2.8 order 0 pv K9 K8 pvVisits 1
    fn from_strs(data: Vec<&str>) -> Self {
        let mut analyze = Analyze::default();
        let keys = vec![
            "move",
            "visits",
            "utility",
            "winrate",
            "scoreMean",
            "scoreStdev",
            "scoreLead",
            "scoreSelfplay",
            "prior",
            "lcb",
            "utilityLcb",
            "order",
            "pv",
            "pvVisits",
        ];
        let mut iter = data.iter();
        'main: while let Some(key) = iter.next() {
            if keys.contains(key) {
                if *key != "pv" {
                    if let Some(value) = iter.next() {
                        match *key {
                            "move" => analyze.move_ = value.to_string(),
                            "visits" => analyze.visits = value.parse().unwrap_or_default(),
                            "utility" => analyze.utility = value.parse().unwrap_or_default(),
                            "winrate" => analyze.winrate = value.parse().unwrap_or_default(),
                            "scoreMean" => analyze.score_mean = value.parse().unwrap_or_default(),
                            "scoreStdev" => analyze.score_stdev = value.parse().unwrap_or_default(),
                            "scoreLead" => analyze.score_lead = value.parse().unwrap_or_default(),
                            "scoreSelfplay" => {
                                analyze.score_selfplay = value.parse().unwrap_or_default()
                            }
                            "prior" => analyze.prior = value.parse().unwrap_or_default(),
                            "lcb" => analyze.lcb = value.parse().unwrap_or_default(),
                            "utilityLcb" => analyze.utility_lcb = value.parse().unwrap_or_default(),
                            "order" => analyze.order = value.parse().unwrap_or_default(),
                            _ => {}
                        }
                    }
                } else {
                    //pv and pvVisits
                    while let Some(value) = iter.next() {
                        match *value {
                            "pvVisits" => {
                                if let Some(value) = iter.next() {
                                    analyze.pv_visits = value.parse().unwrap_or_default()
                                }
                                break 'main;
                            }
                            _ => {
                                analyze.pv.push(value.to_string());
                            }
                        }
                    }
                }
            }
        }

        analyze
    }
}
