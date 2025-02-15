use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_meta::*;

#[component]
pub fn Metadata() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Link rel="icon" type_="image/svg+xml" href="/static/favicon.svg?v=0" />
        <Link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png?v=0" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/static/favicon-32x32.png?v=0" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/static/favicon-16x16.png?v=0" />
        <Link rel="manifest" href="/static/site.webmanifest?v=0" />
        <Link rel="mask-icon" href="/static/safari-pinned-tab.svg?v=0" attr:color="#4f46e5" />
        <Link rel="shortcut icon" href="/static/favicon.ico?v=0" />
        <Meta name="msapplication-TileColor" content="#4f46e5" />
        <Meta name="msapplication-config" content="/static/browserconfig.xml?v=0" />
        <Meta name="theme-color" content="#ffffff" />
    }
}

#[component]
pub fn TitleAndDescription(
    #[prop(into)] title: TextProp,
    #[prop(into)] desc: TextProp,
) -> impl IntoView {
    view! {
        <Title text=title />
        <Meta name="description" content=desc />
    }
}
