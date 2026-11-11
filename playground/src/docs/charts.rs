use dioxus::prelude::*;
use g3_charts::geom::{DAY, days_from_civil};
use g3_charts::prelude::*;
use g3_ui::prelude::*;

#[component]
pub fn DonutExample() -> Element {
    let picked = use_signal(|| None::<String>);
    let slices = vec![
        Slice::new("us", "US stocks", 52_400.0),
        Slice::new("intl", "International", 21_800.0),
        Slice::new("bonds", "Bonds", 18_300.0),
        // Neutral keeps cash out of the series palette.
        Slice::new("cash", "Cash", 4_500.0).with_color(ChartColor::Neutral),
    ];
    let picked_label = picked
        .read()
        .as_ref()
        .and_then(|id| slices.iter().find(|slice| &slice.id == id))
        .map(|slice| slice.label.clone());

    rsx! {
        AppWrapper {
            Header { title: "Portfolio" }
            Content { footer_space: false,
                Card { title: "Holdings",
                    Donut {
                        slices,
                        title: "Holdings by asset class",
                        description: "US stocks are just over half of the portfolio.",
                        format: ValueFormat::Currency { symbol: "$", decimals: 0 },
                        center_label: "Invested",
                        selected: picked,
                    }
                    p { role: "status",
                        match picked_label {
                            Some(label) => format!("Selected: {label}"),
                            None => "Tap a slice, or tab to the chart and use the arrow keys.".to_string(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SunburstExample() -> Element {
    let target = Node::group(
        "root",
        "Portfolio",
        vec![
            Node::group(
                "us",
                "US stocks",
                vec![Node::leaf("vti", "VTI", 0.45), Node::leaf("vb", "VB", 0.10)],
            ),
            Node::group(
                "intl",
                "International",
                vec![
                    Node::leaf("vea", "VEA", 0.20),
                    Node::leaf("vwo", "VWO", 0.05),
                ],
            ),
            Node::leaf("bnd", "BND", 0.20),
        ],
    );
    // Optional: what is held, drawn as thin bands inside each target wedge.
    let held = Node::group(
        "root",
        "Portfolio",
        vec![
            Node::group(
                "us",
                "US stocks",
                vec![
                    Node::leaf("vti", "VTI", 48_100.0),
                    Node::leaf("vb", "VB", 8_900.0),
                ],
            ),
            Node::group(
                "intl",
                "International",
                vec![
                    Node::leaf("vea", "VEA", 17_200.0),
                    Node::leaf("vwo", "VWO", 4_600.0),
                ],
            ),
            Node::leaf("bnd", "BND", 21_200.0),
        ],
    );

    rsx! {
        AppWrapper {
            Header { title: "Allocation" }
            Content { footer_space: false,
                Card { title: "Target allocation",
                    Sunburst {
                        root: target,
                        actual: Some(held),
                        title: "Target allocation against holdings",
                        description: "Inner ring: asset classes. Outer ring: funds.",
                        format: ValueFormat::Percent { decimals: 0 },
                        center_label: "Target",
                    }
                }
            }
        }
    }
}

#[component]
pub fn TargetBarsExample() -> Element {
    let variant = use_signal(|| TargetBarVariant::Bullet);
    let rows = vec![
        TargetRow::new("vti", "VTI", 0.45, 0.48),
        TargetRow::new("vb", "VB", 0.10, 0.09),
        TargetRow::new("vea", "VEA", 0.20, 0.17),
        TargetRow::new("vwo", "VWO", 0.05, 0.05),
        TargetRow::new("bnd", "BND", 0.20, 0.21),
    ];

    rsx! {
        AppWrapper {
            // A segment in the header's toolbar row, as Ionic places one,
            // gets the full width a card would squeeze.
            Header { title: "Targets",
                toolbar: rsx! {
                    SegmentGroup { value: variant, aria_label: "Bar style",
                        SegmentButton { value: TargetBarVariant::Bullet, "Bullet" }
                        SegmentButton { value: TargetBarVariant::Grouped, "Grouped" }
                        SegmentButton { value: TargetBarVariant::Drift, "Drift" }
                    }
                },
            }
            Content { footer_space: false,
                Card { title: "Fund weights",
                    TargetBars {
                        rows,
                        title: "Fund weights against targets",
                        variant: variant(),
                        format: ValueFormat::Percent { decimals: 1 },
                    }
                }
            }
        }
    }
}

#[component]
pub fn LineChartExample() -> Element {
    // Points are milliseconds since 1970 and a value. Fixed data, so a
    // server render and the hydrating client draw the same chart.
    let start = days_from_civil(2025, 1, 6) * DAY;
    let weeks = 0..104_i64;
    let value = weeks
        .clone()
        .map(|w| {
            let trend = 40_000.0 + 260.0 * w as f64;
            let wobble = 1_800.0 * (w as f64 / 6.0).sin();
            Point::new(start + w * 7 * DAY, trend + wobble)
        })
        .collect();
    let paid_in = weeks
        .map(|w| Point::new(start + w * 7 * DAY, 40_000.0 + 200.0 * w as f64))
        .collect();

    rsx! {
        AppWrapper {
            Header { title: "Value" }
            Content { footer_space: false,
                Card { title: "Portfolio value",
                    LineChart {
                        series: vec![
                            Series::new("value", "Value", value).with_area(),
                            Series::new("paid", "Paid in", paid_in).with_color(ChartColor::Neutral),
                        ],
                        title: "Portfolio value and contributions",
                        description: "Weekly value over two years, with the total paid in.",
                        y_format: ValueFormat::Currency { symbol: "$", decimals: 0 },
                    }
                }
            }
        }
    }
}

#[component]
pub fn SparklineExample() -> Element {
    let funds = [("VTI", 0.9_f64), ("VEA", -0.4), ("BND", 0.1)];

    rsx! {
        AppWrapper {
            Header { title: "Watchlist" }
            Content { footer_space: false,
                Card { title: "Last 30 days",
                    List { lines: ListLines::Inset,
                        for (symbol, drift) in funds {
                            Item { key: "{symbol}", label: symbol,
                                end: rsx! {
                                    Sparkline {
                                        values: (0..30)
                                            .map(|d| 100.0 + drift * d as f64 + 1.5 * (d as f64 * 0.7).sin())
                                            .collect::<Vec<_>>(),
                                        label: "{symbol}, last 30 days",
                                        area: true,
                                        format: ValueFormat::Number { decimals: 2 },
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
