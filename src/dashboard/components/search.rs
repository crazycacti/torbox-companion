use leptos::prelude::*;
use leptos::task::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
use crate::api::{SearchTorrent, SearchUsenet, CreateTorrentRequest, CreateUsenetDownloadRequest};
use crate::api::TorboxClient;
use crate::dashboard::DashboardContext;
use crate::dashboard::components::loading_spinner::{LoadingSpinner, SpinnerSize, SpinnerVariant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SearchType {
    Torrents,
    Usenet,
    Both,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SearchResultItem {
    Torrent(SearchTorrent),
    Usenet(SearchUsenet),
}

impl SearchResultItem {
    pub fn title(&self) -> String {
        match self {
            SearchResultItem::Torrent(t) => t.title.clone(),
            SearchResultItem::Usenet(u) => u.title.clone(),
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            SearchResultItem::Torrent(t) => t.size,
            SearchResultItem::Usenet(u) => u.size,
        }
    }

    pub fn hash(&self) -> String {
        match self {
            SearchResultItem::Torrent(t) => t.hash.clone(),
            SearchResultItem::Usenet(u) => u.hash.clone(),
        }
    }

    pub fn is_torrent(&self) -> bool {
        matches!(self, SearchResultItem::Torrent(_))
    }

    pub fn is_usenet(&self) -> bool {
        matches!(self, SearchResultItem::Usenet(_))
    }

    pub fn magnet(&self) -> Option<String> {
        match self {
            SearchResultItem::Torrent(t) => t.magnet.clone(),
            SearchResultItem::Usenet(u) => u.magnet.clone(),
        }
    }

    pub fn nzb(&self) -> Option<String> {
        match self {
            SearchResultItem::Torrent(_) => None,
            SearchResultItem::Usenet(u) => u.nzb.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortField {
    Title,
    Size,
    Seeders,
    Date,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[component]
pub fn SearchComponent() -> impl IntoView {
    let context = use_context::<DashboardContext>()
        .expect("DashboardContext should be provided");
    
    let user_data = context.user_data;
    let has_plan_2 = move || {
        user_data.get()
            .map(|u| u.plan == 2)
            .unwrap_or(false)
    };
    
    let search_query = RwSignal::new(String::new());
    let search_type = RwSignal::new(SearchType::Torrents);
    
    #[cfg(target_arch = "wasm32")]
    {
        let search_type_load = search_type.clone();
        let user_data_load = user_data.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved_type)) = storage.get_item("search_type_preference") {
                        let loaded_type = match saved_type.as_str() {
                            "torrents" => SearchType::Torrents,
                            "usenet" => SearchType::Usenet,
                            "both" => SearchType::Both,
                            _ => SearchType::Torrents,
                        };
                        
                        let final_type = if let Some(user) = user_data_load.get() {
                            let has_plan = user.plan == 2;
                            if !has_plan && (loaded_type == SearchType::Usenet || loaded_type == SearchType::Both) {
                                let _ = storage.set_item("search_type_preference", "torrents");
                                SearchType::Torrents
                            } else {
                                loaded_type
                            }
                        } else {
                            loaded_type
                        };
                        
                        search_type_load.set(final_type);
                    }
                }
            }
        });
    }
    
    {
        let user_data_effect = user_data.clone();
        let search_type_effect = search_type.clone();
        Effect::new(move |_| {
            if let Some(user) = user_data_effect.get() {
                let has_plan = user.plan == 2;
                let current_type = search_type_effect.get();
                
                if !has_plan && (current_type == SearchType::Usenet || current_type == SearchType::Both) {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("search_type_preference", "torrents");
                            }
                        }
                    }
                    search_type_effect.set(SearchType::Torrents);
                }
            }
        });
    }
    
    let save_search_type = move |new_type: SearchType| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let type_str = match new_type {
                        SearchType::Torrents => "torrents",
                        SearchType::Usenet => "usenet",
                        SearchType::Both => "both",
                    };
                    let _ = storage.set_item("search_type_preference", type_str);
                }
            }
        }
        search_type.set(new_type);
    };
    
    let use_custom_indexers = RwSignal::new(false);
    
    #[cfg(target_arch = "wasm32")]
    {
        let use_custom_indexers_load = use_custom_indexers.clone();
        let user_data_load = user_data.clone();
        spawn_local(async move {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved_pref)) = storage.get_item("use_custom_indexers") {
                        let loaded_pref = saved_pref == "true";
                        
                        let final_pref = if let Some(user) = user_data_load.get() {
                            let has_plan = user.plan == 2;
                            if !has_plan && loaded_pref {
                                let _ = storage.set_item("use_custom_indexers", "false");
                                false
                            } else {
                                loaded_pref
                            }
                        } else {
                            loaded_pref
                        };
                        
                        use_custom_indexers_load.set(final_pref);
                    }
                }
            }
        });
    }
    
    {
        let user_data_effect = user_data.clone();
        let use_custom_indexers_effect = use_custom_indexers.clone();
        Effect::new(move |_| {
            if let Some(user) = user_data_effect.get() {
                let has_plan = user.plan == 2;
                let current_pref = use_custom_indexers_effect.get();
                
                if !has_plan && current_pref {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("use_custom_indexers", "false");
                            }
                        }
                    }
                    use_custom_indexers_effect.set(false);
                }
            }
        });
    }
    
    let save_custom_indexers_pref = move |new_pref: bool| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("use_custom_indexers", if new_pref { "true" } else { "false" });
                }
            }
        }
        use_custom_indexers.set(new_pref);
    };
    
    let search_results = RwSignal::new(Vec::<SearchResultItem>::new());
    let is_searching = RwSignal::new(false);
    let search_error = RwSignal::new(Option::<String>::None);
    
    let sort_field = RwSignal::new(SortField::Title);
    let sort_direction = RwSignal::new(SortDirection::Asc);
    
    let format_size = |size: u64| -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.2} {}", size, UNITS[unit_index])
    };
    
    let perform_search = move || {
        let query = search_query.get();
        if query.trim().is_empty() {
            search_results.set(Vec::new());
            return;
        }
        
        is_searching.set(true);
        search_error.set(None);
        
        let search_type_value = search_type.get();
        let use_custom_indexers_value = use_custom_indexers.get();
        let query_clone = query.clone();
        let results_signal = search_results.clone();
        let error_signal = search_error.clone();
        let is_searching_signal = is_searching.clone();
        
        let has_plan_2_value = has_plan_2();
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                let mut all_results = Vec::<SearchResultItem>::new();
                                let mut has_error = false;
                                let mut error_msg = String::new();
                                
                                if use_custom_indexers_value && has_plan_2_value {
                                    match client.search_usenet(
                                        query_clone.clone(),
                                        Some(false),
                                        None,
                                        None,
                                        Some(false),
                                        Some(false),
                                        Some(true),
                                    ).await {
                                        Ok(response) => {
                                            if let Some(data) = response.data {
                                                for usenet in data.nzbs {
                                                    all_results.push(SearchResultItem::Usenet(usenet));
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            has_error = true;
                                            error_msg = format!("Custom indexer search failed: {}", e);
                                        }
                                    }
                                } else {
                                    match search_type_value {
                                        SearchType::Torrents => {
                                            match client.search_torrents(
                                                query_clone.clone(),
                                                Some(false),
                                                Some(true),
                                                Some(false),
                                                None,
                                            ).await {
                                            Ok(response) => {
                                                if let Some(data) = response.data {
                                                    for torrent in data.torrents {
                                                        all_results.push(SearchResultItem::Torrent(torrent));
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                has_error = true;
                                                error_msg = format!("Torrent search failed: {}", e);
                                            }
                                        }
                                    }
                                    SearchType::Usenet => {
                                        if !has_plan_2_value {
                                            has_error = true;
                                            error_msg = "Usenet search requires Plan 2".to_string();
                                        } else {
                                            match client.search_usenet(
                                                query_clone.clone(),
                                                Some(false),
                                                None,
                                                None,
                                                Some(true),
                                                Some(false),
                                                None,
                                            ).await {
                                                Ok(response) => {
                                                    if let Some(data) = response.data {
                                                        for usenet in data.nzbs {
                                                            all_results.push(SearchResultItem::Usenet(usenet));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    has_error = true;
                                                    error_msg = format!("Usenet search failed: {}", e);
                                                }
                                            }
                                        }
                                    }
                                    SearchType::Both => {
                                        match client.search_torrents(
                                            query_clone.clone(),
                                            Some(false),
                                            Some(true),
                                            Some(false),
                                            None,
                                        ).await {
                                            Ok(response) => {
                                                if let Some(data) = response.data {
                                                    for torrent in data.torrents {
                                                        all_results.push(SearchResultItem::Torrent(torrent));
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                has_error = true;
                                                error_msg = format!("Torrent search failed: {}", e);
                                            }
                                        }
                                        
                                        if has_plan_2_value {
                                            match client.search_usenet(
                                                query_clone.clone(),
                                                Some(false),
                                                None,
                                                None,
                                                Some(true),
                                                Some(false),
                                                None,
                                            ).await {
                                                Ok(response) => {
                                                    if let Some(data) = response.data {
                                                        for usenet in data.nzbs {
                                                            all_results.push(SearchResultItem::Usenet(usenet));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    if !has_error {
                                                        has_error = true;
                                                        error_msg = format!("Usenet search failed: {}", e);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                                
                                results_signal.set(all_results);
                                if has_error {
                                    error_signal.set(Some(error_msg));
                                }
                            }
                        }
                    }
                }
            }
            is_searching_signal.set(false);
        });
    };
    
    let handle_download = move |item: SearchResultItem| {
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                
                                match item {
                                    SearchResultItem::Torrent(t) => {
                                        if let Some(magnet) = t.magnet {
                                            let request = CreateTorrentRequest {
                                                magnet: Some(magnet),
                                                file: None,
                                                seed: None,
                                                allow_zip: None,
                                                name: Some(t.title),
                                                as_queued: None,
                                                add_only_if_cached: None,
                                            };
                                            let _ = client.create_torrent(request).await;
                                        }
                                    }
                                    SearchResultItem::Usenet(u) => {
                                        if let Some(nzb_url) = u.nzb {
                                            let request = CreateUsenetDownloadRequest {
                                                link: Some(nzb_url),
                                                file: None,
                                                name: Some(u.title),
                                                password: None,
                                                post_processing: None,
                                                as_queued: None,
                                                add_only_if_cached: None,
                                            };
                                            let _ = client.create_usenet_download(request).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    };
    
    let handle_copy_magnet = move |item: SearchResultItem| {
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(magnet) = item.magnet() {
                    if let Some(window) = web_sys::window() {
                        let clipboard = window.navigator().clipboard();
                        let promise = clipboard.write_text(&magnet);
                        if let Ok(_) = JsFuture::from(promise).await {
                            web_sys::console::log_1(&"Magnet link copied to clipboard!".into());
                        }
                    }
                }
            }
        });
    };
    
    let sorted_results = move || {
        let mut results = search_results.get();
        let field = sort_field.get();
        let direction = sort_direction.get();
        
        results.sort_by(|a, b| {
            let comparison = match field {
                SortField::Title => a.title().cmp(&b.title()),
                SortField::Size => a.size().cmp(&b.size()),
                SortField::Seeders => {
                    let a_seeders = match a {
                        SearchResultItem::Torrent(t) => t.last_known_seeders.unwrap_or(-1),
                        SearchResultItem::Usenet(_) => -1,
                    };
                    let b_seeders = match b {
                        SearchResultItem::Torrent(t) => t.last_known_seeders.unwrap_or(-1),
                        SearchResultItem::Usenet(_) => -1,
                    };
                    a_seeders.cmp(&b_seeders)
                }
                SortField::Date => {
                    String::cmp(&a.title(), &b.title())
                }
            };
            
            match direction {
                SortDirection::Asc => comparison,
                SortDirection::Desc => comparison.reverse(),
            }
        });
        
        results
    };
    
    let toggle_sort = move |field: SortField| {
        if sort_field.get() == field {
            sort_direction.update(|d| *d = match *d {
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::Asc,
            });
        } else {
            sort_field.set(field);
            sort_direction.set(SortDirection::Asc);
        }
    };
    
    view! {
        <div class="w-full mb-4 sm:mb-6" style="position: relative; z-index: 10; pointer-events: auto;" on:click=move |ev| {
            ev.stop_propagation();
        }>
            <div class="rounded-xl border p-4 sm:p-5 md:p-6" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                <div class="flex flex-col gap-4">
                    <div class="flex gap-2 items-stretch">
                        <div class="flex-1 relative">
                            <input
                                type="text"
                                placeholder="Search torrents, usenet..."
                                class="w-full px-4 py-3 pr-10 rounded-lg border transition-all text-sm sm:text-base"
                                style="background-color: var(--bg-secondary); border-color: var(--border-primary); color: var(--text-primary);"
                                prop:value=move || search_query.get()
                                on:input=move |ev| {
                                    ev.stop_propagation();
                                    let value = event_target_value(&ev);
                                    search_query.set(value);
                                }
                                on:keypress=move |ev| {
                                    ev.stop_propagation();
                                    if ev.key_code() == 13 {
                                        perform_search();
                                    }
                                }
                                on:click=move |ev| {
                                    ev.stop_propagation();
                                }
                                on:focus=move |ev| {
                                    ev.stop_propagation();
                                }
                            />
                            <Show when=move || !search_query.get().is_empty()>
                                <button
                                    type="button"
                                    class="absolute top-1/2 -translate-y-1/2 right-3 p-1.5 rounded transition-all"
                                    style="color: var(--text-secondary);"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        search_query.set(String::new());
                                        search_results.set(Vec::new());
                                        search_error.set(None);
                                    }
                                    title="Clear search"
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                    </svg>
                                </button>
                            </Show>
                        </div>
                        <button
                            class="px-5 sm:px-6 py-3 rounded-lg transition-all font-medium flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed shrink-0"
                            style="background-color: var(--accent-primary); color: white; min-width: 100px;"
                            on:click=move |_| perform_search()
                            disabled=move || is_searching.get()
                        >
                            <Show when=move || is_searching.get()>
                                <LoadingSpinner size=SpinnerSize::Small variant=SpinnerVariant::Default/>
                            </Show>
                            {move || if is_searching.get() { "Searching..." } else { "Search" }.to_string()}
                        </button>
                    </div>
                    
                    <div class="flex flex-col sm:flex-row gap-3 items-start sm:items-center">
                        <div class="flex items-center gap-3 w-full sm:w-auto">
                            <span class="text-sm font-medium shrink-0" style="color: var(--text-secondary);">"Type:"</span>
                            <div class="flex gap-0.5 rounded-lg" style="background-color: var(--bg-secondary); border: 1px solid var(--border-secondary);">
                            <button
                                class="px-4 py-2 text-sm font-medium transition-all rounded-l-lg"
                                style=move || {
                                    let is_active = search_type.get() == SearchType::Torrents;
                                    if is_active {
                                        "background-color: var(--accent-primary); color: white;"
                                    } else {
                                        "background-color: transparent; color: var(--text-secondary);"
                                    }
                                }
                                on:click=move |_| save_search_type(SearchType::Torrents)
                            >
                                "Torrents"
                            </button>
                            <button
                                class="px-4 py-2 text-sm font-medium transition-all"
                                style=move || {
                                    let has_plan = has_plan_2();
                                    let is_active = search_type.get() == SearchType::Usenet;
                                    if !has_plan {
                                        "background-color: transparent; color: var(--text-secondary); opacity: 0.4; cursor: not-allowed;"
                                    } else if is_active {
                                        "background-color: var(--accent-primary); color: white;"
                                    } else {
                                        "background-color: transparent; color: var(--text-secondary);"
                                    }
                                }
                                disabled=move || !has_plan_2()
                                on:click=move |_| {
                                    if has_plan_2() {
                                        save_search_type(SearchType::Usenet);
                                    }
                                }
                            >
                                "Usenet"
                            </button>
                            <button
                                class="px-4 py-2 text-sm font-medium transition-all rounded-r-lg"
                                style=move || {
                                    let has_plan = has_plan_2();
                                    let is_active = search_type.get() == SearchType::Both;
                                    if !has_plan {
                                        "background-color: transparent; color: var(--text-secondary); opacity: 0.4; cursor: not-allowed;"
                                    } else if is_active {
                                        "background-color: var(--accent-primary); color: white;"
                                    } else {
                                        "background-color: transparent; color: var(--text-secondary);"
                                    }
                                }
                                disabled=move || !has_plan_2()
                                on:click=move |_| {
                                    if has_plan_2() {
                                        save_search_type(SearchType::Both);
                                    }
                                }
                            >
                                "Both"
                            </button>
                            </div>
                        </div>
                        
                        <div class="flex items-center gap-3 w-full sm:w-auto sm:ml-auto">
                            <span class="text-sm font-medium shrink-0" style="color: var(--text-secondary);">"Custom:"</span>
                            <button
                                class="px-4 py-1.5 sm:py-2 rounded-full text-xs sm:text-sm font-medium transition-all"
                                style=move || {
                                    let has_plan = has_plan_2();
                                    let is_enabled = use_custom_indexers.get();
                                    if !has_plan {
                                        "background-color: var(--bg-secondary); color: var(--text-secondary); opacity: 0.4; cursor: not-allowed; border: 1px solid var(--border-secondary);"
                                    } else if is_enabled {
                                        "background-color: var(--accent-primary); color: white; border: none;"
                                    } else {
                                        "background-color: var(--bg-secondary); color: var(--text-secondary); border: 1px solid var(--border-secondary);"
                                    }
                                }
                                disabled=move || !has_plan_2()
                                on:click=move |_| {
                                    if has_plan_2() {
                                        save_custom_indexers_pref(!use_custom_indexers.get());
                                    }
                                }
                            >
                                {move || if use_custom_indexers.get() { "ON" } else { "OFF" }}
                            </button>
                        </div>
                    </div>
                    
                    <Show when=move || search_error.get().is_some()>
                        <div class="px-4 py-3 rounded-lg flex items-center gap-2" style="background-color: rgba(239, 68, 68, 0.1); border: 1px solid rgba(239, 68, 68, 0.3); color: #fca5a5;">
                            <svg class="w-5 h-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <span class="text-sm">{move || search_error.get().unwrap_or_default()}</span>
                        </div>
                    </Show>
                    
                    <Show when=move || is_searching.get()>
                        <div class="flex justify-center py-4">
                            <LoadingSpinner 
                                size=SpinnerSize::Medium 
                                variant=SpinnerVariant::Accent 
                                text="Searching...".to_string()
                                centered=true
                            />
                        </div>
                    </Show>
                </div>
            </div>
            
            <Show when=move || !search_results.get().is_empty()>
                <div class="mt-4 rounded-xl border overflow-hidden" style="background-color: var(--bg-card); border-color: var(--border-secondary);">
                    <div class="overflow-x-auto" style="max-height: 500px; overflow-y: auto;">
                        <table class="w-full border-collapse min-w-[600px]">
                            <thead style="position: sticky; top: 0; z-index: 10;">
                                <tr style="background-color: var(--bg-tertiary); border-bottom: 1px solid var(--border-primary);">
                                    <th class="px-4 md:px-6 py-3 text-left">
                                        <button
                                            class="flex items-center gap-2 font-medium hover:opacity-80 transition-opacity text-sm"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| toggle_sort(SortField::Title)
                                        >
                                            "Title"
                                            {move || if sort_field.get() == SortField::Title {
                                                if sort_direction.get() == SortDirection::Asc {
                                                    "↑"
                                                } else {
                                                    "↓"
                                                }
                                            } else {
                                                ""
                                            }}
                                        </button>
                                    </th>
                                    <th class="px-4 md:px-6 py-3 text-left">
                                        <button
                                            class="flex items-center gap-2 font-medium hover:opacity-80 transition-opacity text-sm"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| toggle_sort(SortField::Size)
                                        >
                                            "Size"
                                            {move || if sort_field.get() == SortField::Size {
                                                if sort_direction.get() == SortDirection::Asc {
                                                    "↑"
                                                } else {
                                                    "↓"
                                                }
                                            } else {
                                                ""
                                            }}
                                        </button>
                                    </th>
                                    <th class="px-4 md:px-6 py-3 text-left hidden sm:table-cell">
                                        <button
                                            class="flex items-center gap-2 font-medium hover:opacity-80 transition-opacity text-sm"
                                            style="color: var(--text-primary);"
                                            on:click=move |_| toggle_sort(SortField::Seeders)
                                        >
                                            "Seeders"
                                            {move || if sort_field.get() == SortField::Seeders {
                                                if sort_direction.get() == SortDirection::Asc {
                                                    "↑"
                                                } else {
                                                    "↓"
                                                }
                                            } else {
                                                ""
                                            }}
                                        </button>
                                    </th>
                                    <th class="px-4 md:px-6 py-3 text-left font-medium text-sm" style="color: var(--text-primary);">
                                        "Type"
                                    </th>
                                    <th class="px-4 md:px-6 py-3 text-left font-medium text-sm" style="color: var(--text-primary); width: 100px;">
                                        "Actions"
                                    </th>
                                </tr>
                            </thead>
                        <tbody>
                            {move || {
                                sorted_results().into_iter().map(|item| {
                                    let item_download = item.clone();
                                    let item_magnet_check = item.clone();
                                    let item_copy_magnet = item.clone();
                                    let item_title = item.clone();
                                    let item_title_for_raw = item.clone();
                                    let item_size = item.clone();
                                    let item_seeders = item.clone();
                                    let item_type = item.clone();
                                    
                                    let details: Vec<String> = match &item {
                                        SearchResultItem::Torrent(t) => {
                                            if let Some(tpd) = &t.title_parsed_data {
                                                let mut d = Vec::new();
                                                if let Some(res) = &tpd.resolution {
                                                    d.push(res.clone());
                                                }
                                                if let Some(qual) = &tpd.quality {
                                                    d.push(qual.clone());
                                                }
                                                if let Some(codec) = &tpd.codec {
                                                    d.push(codec.clone());
                                                }
                                                if let Some(season) = tpd.season {
                                                    d.push(format!("S{:02}", season));
                                                }
                                                d
                                            } else {
                                                Vec::new()
                                            }
                                        }
                                        SearchResultItem::Usenet(u) => {
                                            if let Some(tpd) = &u.title_parsed_data {
                                                let mut d = Vec::new();
                                                if let Some(res) = &tpd.resolution {
                                                    d.push(res.clone());
                                                }
                                                if let Some(qual) = &tpd.quality {
                                                    d.push(qual.clone());
                                                }
                                                if let Some(codec) = &tpd.codec {
                                                    d.push(codec.clone());
                                                }
                                                if let Some(season) = tpd.season {
                                                    d.push(format!("S{:02}", season));
                                                }
                                                d
                                            } else {
                                                Vec::new()
                                            }
                                        }
                                    };
                                    let has_details = !details.is_empty();
                                    let details_for_view = details.clone();
                                    let has_magnet = item_magnet_check.magnet().is_some();
                                    let item_copy_for_magnet = item_copy_magnet.clone();
                                    
                                    view! {
                                        <tr
                                            class="border-b transition-colors hover:bg-opacity-5"
                                            style="border-color: var(--border-secondary);"
                                        >
                                            <td class="px-4 md:px-6 py-4" style="color: var(--text-primary);">
                                                <div class="flex flex-col gap-2">
                                                    <span class="font-semibold text-sm sm:text-base leading-tight" style="color: var(--text-primary);">{item_title.title()}</span>
                                                    <div class="flex flex-col gap-1.5">
                                                        <span class="text-xs sm:text-sm font-mono leading-relaxed" style="color: var(--text-secondary); opacity: 0.85; word-break: break-all;">
                                                            {move || {
                                                                match &item_title_for_raw {
                                                                    SearchResultItem::Torrent(t) => t.raw_title.clone(),
                                                                    SearchResultItem::Usenet(u) => u.raw_title.clone(),
                                                                }
                                                            }}
                                                        </span>
                                                        <Show when=move || has_details>
                                                            <div class="flex gap-1.5 flex-wrap">
                                                                {details_for_view.iter().map(|d| {
                                                                    let d_clone = d.clone();
                                                                    view! {
                                                                        <span class="px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(59, 130, 246, 0.15); color: #93c5fd; border: 1px solid rgba(59, 130, 246, 0.3);">
                                                                            {d_clone}
                                                                        </span>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </div>
                                                        </Show>
                                                    </div>
                                                </div>
                                            </td>
                                            <td class="px-4 md:px-6 py-4 font-medium text-sm" style="color: var(--text-secondary);">
                                                {format_size(item_size.size())}
                                            </td>
                                            <td class="px-4 md:px-6 py-4 font-medium text-sm hidden sm:table-cell" style="color: var(--text-secondary);">
                                                {move || {
                                                    match &item_seeders {
                                                        SearchResultItem::Torrent(t) => {
                                                            t.last_known_seeders.map(|s| s.to_string()).unwrap_or_else(|| "-".to_string())
                                                        }
                                                        SearchResultItem::Usenet(_) => "-".to_string()
                                                    }
                                                }}
                                            </td>
                                            <td class="px-4 md:px-6 py-4">
                                                {move || match &item_type {
                                                    SearchResultItem::Torrent(t) => {
                                                        let cached = t.cached.unwrap_or(false);
                                                        view! {
                                                            <div class="flex items-center gap-2">
                                                                <span class="px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(59, 130, 246, 0.1); color: #60a5fa; border: 1px solid rgba(59, 130, 246, 0.3);">
                                                                    "Torrent"
                                                                </span>
                                                                <Show when=move || cached>
                                                                    <span class="px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(34, 197, 94, 0.1); color: #86efac; border: 1px solid rgba(34, 197, 94, 0.3);">
                                                                        "Cached"
                                                                    </span>
                                                                </Show>
                                                            </div>
                                                        }.into_any()
                                                    }
                                                    SearchResultItem::Usenet(u) => {
                                                        let cached = u.cached.unwrap_or(false);
                                                        view! {
                                                            <div class="flex items-center gap-2">
                                                                <span class="px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(168, 85, 247, 0.1); color: #c084fc; border: 1px solid rgba(168, 85, 247, 0.3);">
                                                                    "Usenet"
                                                                </span>
                                                                <Show when=move || cached>
                                                                    <span class="px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(34, 197, 94, 0.1); color: #86efac; border: 1px solid rgba(34, 197, 94, 0.3);">
                                                                        "Cached"
                                                                    </span>
                                                                </Show>
                                                            </div>
                                                        }.into_any()
                                                    }
                                                }}
                                            </td>
                                            <td class="px-4 md:px-6 py-4">
                                                <div class="flex items-center gap-2">
                                                    <button
                                                        class="p-2 rounded-lg transition-all flex items-center justify-center hover:opacity-80 hover:scale-105"
                                                        style="background-color: transparent; color: var(--accent-secondary);"
                                                        on:click=move |_| handle_download(item_download.clone())
                                                        title="Download"
                                                    >
                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                                                        </svg>
                                                    </button>
                                                    {move || {
                                                        if has_magnet {
                                                            let item_copy_btn = item_copy_for_magnet.clone();
                                                            view! {
                                                                <button
                                                                    class="p-2 rounded-lg transition-all flex items-center justify-center hover:opacity-80 hover:scale-105"
                                                                    style="background-color: transparent; color: var(--accent-secondary);"
                                                                    on:click=move |_| {
                                                                        handle_copy_magnet(item_copy_btn.clone());
                                                                    }
                                                                    title="Copy Magnet Link"
                                                                >
                                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                                                    </svg>
                                                                </button>
                                                            }.into_any()
                                                        } else {
                                                            view! {}.into_any()
                                                        }
                                                    }}
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                            </tbody>
                        </table>
                    </div>
                </div>
            </Show>
        </div>
    }
}

