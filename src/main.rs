use dioxus::document::eval;
use dioxus::prelude::*;
use serde::de::IgnoredAny;
use serde_saphyr::{from_multiple_with_options, from_str_with_options, Options};
use serde_saphyr::budget::BudgetReport;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut input_text = use_signal(|| "".to_string());
    let mut output_text = use_signal(|| "".to_string());
    let mut budget_report_signal = use_signal(|| None::<serde_saphyr::budget::BudgetReport>);

    use_effect(move || {
        spawn(async move {
            let mut eval = eval(
                r#"
                let parent = document.getElementById("editor-container");
                let initial_value = await dioxus.recv();
                window.view = window.initCodeMirror(parent, initial_value, (val) => {
                    dioxus.send(val);
                });
                "#,
            );

            eval.send(input_text.read().clone()).unwrap();

            while let Ok(val) = eval.recv::<serde_json::Value>().await {
                if let Some(s) = val.as_str() {
                    input_text.set(s.to_string());
                }
            }
        });
    });

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; font-family: sans-serif;",
            div {
                style: "padding: 10px; border-bottom: 1px solid #ccc; display: flex; justify-content: space-between; align-items: center;",
                button {
                    onclick: move |_| {
                        let options = Options {
                            with_snippet: false, // Using miette for snippets
                            ..Options::default()
                        }.with_budget_report({
                            let mut signal = budget_report_signal;
                            move |report: BudgetReport| {
                                signal.set(Some(report));
                            }
                        });

                        let content = input_text.read();
                        // We can validate multiple documents
                        let result: Result<Vec<IgnoredAny>, _> = from_multiple_with_options(&content, options);
                        
                        let mut final_output = String::new();
                        match result {
                            Ok(_) => {
                                final_output.push_str("YAML is valid");
                            }
                            Err(err) => {
                                budget_report_signal.set(None);
                                let report = serde_saphyr::miette::to_miette_report(&err, &content, "input.yaml");
                                
                                let mut ansi_output = String::new();
                                let handler = miette::GraphicalReportHandler::new_themed(miette::GraphicalTheme::unicode())
                                    .with_links(true);
                                let _ = handler.render_report(&mut ansi_output, report.as_ref());
                                
                                let converter = ansi_to_html::Converter::default();
                                final_output = converter.convert(&ansi_output).unwrap_or_else(|_| ansi_output);
                            }
                        }
                        output_text.set(final_output);
                    },
                    "Validate"
                }
                span {
                    style: "font-size: 0.9em; color: #666;",
                    "Your YAML will be validated within this browser, without sending it to our server"
                }
            }
            div {
                style: "display: flex; flex: 1; overflow: hidden;",
                div {
                    id: "editor-container",
                    style: "flex: 1; border-right: 1px solid #ccc; overflow: hidden; background: #fff;",
                }
                div {
                    style: "flex: 1; padding: 10px; margin: 0; overflow: auto; background-color: #ffffff; color: #000000; font-family: monospace;",
                    if let Some(report) = budget_report_signal() {
                        div {
                            p { style: "font-weight: bold; color: green;", "{output_text}" }
                            h3 { "Budget Report" }
                            table {
                                style: "border-collapse: collapse; width: 100%; max-width: 400px; margin-top: 10px;",
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Events" } td { style: "padding: 4px; text-align: right;", "{report.events}" } }
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Aliases" } td { style: "padding: 4px; text-align: right;", "{report.aliases}" } }
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Anchors" } td { style: "padding: 4px; text-align: right;", "{report.anchors}" } }
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Documents" } td { style: "padding: 4px; text-align: right;", "{report.documents}" } }
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Nodes" } td { style: "padding: 4px; text-align: right;", "{report.nodes}" } }
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Max Depth" } td { style: "padding: 4px; text-align: right;", "{report.max_depth}" } }
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Total Scalar Bytes" } td { style: "padding: 4px; text-align: right;", "{report.total_scalar_bytes}" } }
                                tr { style: "border-bottom: 1px solid #eee;", td { style: "padding: 4px;", "Merge Keys" } td { style: "padding: 4px; text-align: right;", "{report.merge_keys}" } }
                                if let Some(breach) = report.breached {
                                    tr { style: "color: red; font-weight: bold;", td { style: "padding: 4px;", "BREACHED" } td { style: "padding: 4px; text-align: right;", "{breach:?}" } }
                                }
                            }
                        }
                    } else {
                        pre {
                            style: "margin: 0; white-space: pre-wrap;",
                            dangerous_inner_html: "{output_text}"
                        }
                    }
                }
            }
        }
    }
}
