use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::landing_page::LandingPage;
use crate::dashboard::MainDashboard;
use crate::stream_page::StreamPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                
                <meta name="description" content="An alternative to the default TorBox UI. Built with Rust because why not? Manage downloads, streaming, and more."/>
                <meta name="keywords" content="torbox, torrent, download manager, usenet, streaming, rust, web app, p2p, file sharing, dark theme, api"/>
                <meta name="author" content="Torbox Companion"/>
                <meta name="robots" content="index, follow"/>
                <meta name="language" content="en"/>
                <meta name="revisit-after" content="7 days"/>
                
                <meta property="og:title" content="Torbox Companion"/>
                <meta property="og:description" content="An alternative to the default TorBox UI. Built with Rust because why not? Manage downloads, streaming, and more."/>
                <meta property="og:type" content="website"/>
                <meta property="og:url" content="https://torbox-companion.app"/>
                <meta property="og:image" content="/favicon-192x192.png"/>
                <meta property="og:image:width" content="192"/>
                <meta property="og:image:height" content="192"/>
                <meta property="og:site_name" content="Torbox Companion"/>
                <meta property="og:locale" content="en_US"/>
                
                <meta name="twitter:card" content="summary"/>
                <meta name="twitter:title" content="Torbox Companion"/>
                <meta name="twitter:description" content="An alternative to the default TorBox UI. Built with Rust because why not? Manage downloads, streaming, and more."/>
                <meta name="twitter:image" content="/favicon-192x192.png"/>
                
                <meta name="theme-color" content="#1e293b"/>
                <meta name="msapplication-TileColor" content="#1e293b"/>
                <meta name="msapplication-TileImage" content="/favicon-192x192.png"/>
                <meta name="apple-mobile-web-app-capable" content="yes"/>
                <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent"/>
                <meta name="apple-mobile-web-app-title" content="Torbox Companion"/>
                
                <link rel="icon" type="image/x-icon" href="/favicon.ico"/>
                <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png"/>
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png"/>
                <link rel="icon" type="image/png" sizes="48x48" href="/favicon-48x48.png"/>
                <link rel="icon" type="image/png" sizes="64x64" href="/favicon-64x64.png"/>
                <link rel="icon" type="image/png" sizes="96x96" href="/favicon-96x96.png"/>
                <link rel="icon" type="image/png" sizes="128x128" href="/favicon-128x128.png"/>
                <link rel="icon" type="image/png" sizes="192x192" href="/favicon-192x192.png"/>
                <link rel="icon" type="image/png" sizes="256x256" href="/favicon-256x256.png"/>
                <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
                <link rel="manifest" href="/site.webmanifest"/>
                
                <link rel="canonical" href="https://torbox-companion.app"/>
                
                <script type="application/ld+json">
                {r#"
                {
                    "@context": "https://schema.org",
                    "@type": "WebApplication",
                    "name": "Torbox Companion",
                    "description": "An alternative to the default TorBox UI. Built with Rust because why not? Manage downloads, streaming, and more.",
                    "url": "https://torbox-companion.app",
                    "applicationCategory": "UtilityApplication",
                    "operatingSystem": "Web Browser",
                    "offers": {
                        "@type": "Offer",
                        "price": "0",
                        "priceCurrency": "USD"
                    },
                    "author": {
                        "@type": "Organization",
                        "name": "Torbox Companion"
                    },
                    "datePublished": "2024-10-25",
                    "dateModified": "2024-10-25",
                    "inLanguage": "en",
                    "isAccessibleForFree": true,
                    "browserRequirements": "Requires JavaScript. Requires HTML5.",
                    "softwareVersion": "1.0.0",
                    "screenshot": "https://torbox-companion.app/favicon-192x192.png"
                }
                "#}
                </script>
                
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/torbox-companion.css"/>
        <Title text="Torbox Companion"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=LandingPage/>
                    <Route path=StaticSegment("dashboard") view=MainDashboard/>
                    <Route path=StaticSegment("stream") view=StreamPage/>
                </Routes>
            </main>
        </Router>
    }
}
