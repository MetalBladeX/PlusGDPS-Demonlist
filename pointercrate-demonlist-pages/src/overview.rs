use std::borrow::Cow;


use maud::{html, Markup, PreEscaped};
use url::Url;

use pointercrate_core_pages::{config as page_config, PageFragment, Script};
use pointercrate_demonlist::{config as list_config};


use crate::{
    components::{
        submitter::{submit_panel, RecordSubmitter},
        time_machine::TimeMachine,
    },
    statsviewer::stats_viewer_panel,
    DemonlistData,
};

pub struct OverviewPage {
    pub data: DemonlistData,
    pub time_machine: TimeMachine,
    pub submitter: RecordSubmitter<'static>,
}

impl PageFragment for OverviewPage {
    fn title(&self) -> String {
        "Geometry Dash Demonlist".to_string()
    }

    fn description(&self) -> String {
        "The official pointercrate Demonlist!".to_string()
    }

    fn additional_scripts(&self) -> Vec<Script> {
        vec![
            Script::module("/static/js/modules/formv2.js"),
            Script::module("/static/js/modules/demonlistv2.js"),
            Script::module("/static/js/demonlist.v2.2.js"),
        ]
    }

    fn additional_stylesheets(&self) -> Vec<String> {
        vec!["/static/css/demonlist.v2.1.css".to_string(), "/static/css/sidebar.css".to_string()]
    }

    fn head_fragment(&self) -> Markup {
        html! {
            (PreEscaped(r#"
                <script type="application/ld+json">
                {
                    "@context": "http://schema.org",
                    "@type": "WebPage",
                    "breadcrumb": {
                        "@type": "BreadcrumbList",
                        "itemListElement": [
                            {
                                "@type": "ListItem",
                                "position": 1,
                                "item": {
                                    "@id": "https://pointercrate.com/",
                                    "name": "pointercrate"
                                }
                            },
                            {
                                "@type": "ListItem",
                                "position": 2,
                                "item": {
                                    "@id": "https://pointercrate.com/demonlist/",
                                    "name": "demonlist"
                                }
                            }
                        ]
                    },
                    "name": "Geometry Dash Demonlist",
                    "description": "The official pointercrate Demonlist!",
                    "url": "https://pointercrate.com/demonlist/"
                }
                </script>
            "#))
            (PreEscaped(format!("
                <script>
                    window.list_length = {0};
                    window.extended_list_length = {1}
                </script>", list_config::list_size(), list_config::extended_list_size())
            ))
            // FIXME: abstract away
            link ref = "canonical" href = "https://pointercrate.com/demonlist/";
        }
    }

    fn body_fragment(&self) -> Markup {
        let dropdowns = super::dropdowns(&self.data.demon_overview, None);
        html! {
            (super::besides_sidebar_ad())
            (dropdowns)

            div.flex.m-center.container {
                main.left {
                    (self.time_machine)
                    (self.submitter)

                    @for demon in &self.data.demon_overview {
                        @if demon.position <= list_config::extended_list_size() {
                            section.panel.fade style="overflow:hidden" {
                                @if let Some(ref video) = demon.video {
                                    div.flex style = "align-items: center" {
                                        div.thumb."ratio-16-9"."js-delay-css" style = "position: relative" data-property = "background-image" data-property-value = {"url('" (thumbnail(video)) "')"} {
                                            a.play href = (video) {}
                                        }
                                        div style = "padding-left: 15px" {
                                            h2 style = "text-align: left; margin-bottom: 0px" {
                                                a href = {"/demonlist/permalink/" (demon.id) "/"} {
                                                    "#" (demon.position) (PreEscaped(" &#8211; ")) (demon.name)
                                                }
                                            }
                                            h3 style = "text-align: left" {
                                                i {
                                                    (demon.publisher)
                                                }
                                                @if let Some(current_position) = demon.current_position {
                                                    br;
                                                    @if current_position > list_config::extended_list_size() {
                                                        "Currently Legacy"
                                                    }
                                                    @else {
                                                        "Currently #"(current_position)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                @else {
                                    div.flex.col style = "align-items: center" {
                                        h2 style = "margin-bottom: 0px"{
                                            a href = {"/demonlist/permalink/" (demon.id) "/"} {
                                                "#" (demon.position) (PreEscaped(" &#8211; ")) (demon.name)
                                            }
                                        }
                                        h3 {
                                            i {
                                                (demon.publisher)
                                            }
                                            @if let Some(current_position) = demon.current_position {
                                                br;
                                                @if current_position > list_config::extended_list_size() {
                                                    "Currently Legacy"
                                                }
                                                @else {
                                                    "Currently #"(current_position)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            @if demon.position == 1 {
                                section.panel.fade style = "padding: 0px; height: 90px"{
                                (PreEscaped(format!(r#"
                                    <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={0}"
     crossorigin="anonymous"></script>
<!-- Demonlist Responsive Feed Ad -->
<ins class="adsbygoogle"
     style="display:inline-block;width:728px;height:90px"
     data-ad-client="{0}"
     data-ad-slot="2819150519"></ins>
<script>
     (adsbygoogle = window.adsbygoogle || []).push({{}});
</script>
                                    "#, page_config::adsense_publisher_id())))
                                }
                            }
                            // Place ad every 20th demon
                            @if demon.position % 20 == 0 {
                                section.panel.fade {
                                (PreEscaped(format!(r#"
                                    <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={0}"
     crossorigin="anonymous"></script>
<ins class="adsbygoogle"
     style="display:block"
     data-ad-format="fluid"
     data-ad-layout-key="-h1+40+4u-93+n"
     data-ad-client="{0}"
     data-ad-slot="5157884729"></ins>
<script>
     (adsbygoogle = window.adsbygoogle || []).push({{}});
</script>
                                    "#, page_config::adsense_publisher_id())))
                                }
                            }
                        }
                    }
                }

                aside.right {
                    (self.data.team_panel())
                    (super::sidebar_ad())
                    (super::rules_panel())
                    (submit_panel())
                    (stats_viewer_panel())
                    (super::discord_panel())
                }
            }
        }
    }
}

fn thumbnail(video: &str) -> String {
    // Videos need to be well formed once we get here!
    let url = Url::parse(video).unwrap();
    let video_id = url
        .query_pairs()
        .find_map(|(key, value)| if key == "v" { Some(value) } else { None })
        .unwrap_or(Cow::Borrowed("dQw4w9WgXcQ"));

    format!("https://i.ytimg.com/vi/{}/mqdefault.jpg", video_id)
}
