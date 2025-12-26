

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
    IMDB,
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

    pub fn parsed_title(&self) -> String {
        match self {
            SearchResultItem::Torrent(t) => {
                t.title_parsed_data.as_ref()
                    .and_then(|tpd| tpd.title.clone())
                    .unwrap_or_else(|| t.title.clone())
            }
            SearchResultItem::Usenet(u) => {
                u.title_parsed_data.as_ref()
                    .and_then(|tpd| tpd.title.clone())
                    .unwrap_or_else(|| u.title.clone())
            }
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
                            "imdb" => SearchType::IMDB,
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
                        SearchType::IMDB => "imdb",
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
    let total_results_count = RwSignal::new(0);
    let is_searching = RwSignal::new(false);
    let search_error = RwSignal::new(Option::<String>::None);
    let downloading_items = RwSignal::new(std::collections::HashSet::<String>::new());
    let expanded_groups = RwSignal::new(std::collections::HashSet::<String>::new());
    
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
            expanded_groups.update(|set| set.clear());
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
                                let mut total_count = 0;
                                let mut has_error = false;
                                let mut error_msg = String::new();
                                
                                match search_type_value {
                                    SearchType::IMDB => {
                                        let imdb_id = query_clone.trim().to_string();
                                        if !imdb_id.starts_with("tt") {
                                            has_error = true;
                                            error_msg = "IMDB ID must start with 'tt' (e.g., tt5151761)".to_string();
                                        } else {
                                            match client.get_torrents_by_imdb(imdb_id.clone()).await {
                                                Ok(response) => {
                                                    if let Some(data) = response.data {
                                                        total_count += data.total_torrents;
                                                        for torrent in data.torrents {
                                                            all_results.push(SearchResultItem::Torrent(torrent));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    has_error = true;
                                                    error_msg = format!("IMDB torrent search failed: {}", e);
                                                }
                                            }
                                            
                                            if has_plan_2_value {
                                                match client.get_usenet_by_imdb(imdb_id).await {
                                                    Ok(response) => {
                                                        if let Some(data) = response.data {
                                                            total_count += data.total_nzbs;
                                                            for usenet in data.nzbs {
                                                                all_results.push(SearchResultItem::Usenet(usenet));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        if !has_error {
                                                            has_error = true;
                                                            error_msg = format!("IMDB usenet search failed: {}", e);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if use_custom_indexers_value && has_plan_2_value {
                                            // Custom indexers mode: search both torrents and usenet with custom engines
                                            match client.search_torrents(
                                                query_clone.clone(),
                                                Some(false),
                                                Some(true),
                                                Some(false),
                                                Some(true), // Use custom search engines
                                            ).await {
                                                Ok(response) => {
                                                    if let Some(data) = response.data {
                                                        total_count += data.total_torrents;
                                                        for torrent in data.torrents {
                                                            all_results.push(SearchResultItem::Torrent(torrent));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    has_error = true;
                                                    error_msg = format!("Custom indexer torrent search failed: {}", e);
                                                }
                                            }
                                            
                                            match client.search_usenet(
                                                query_clone.clone(),
                                                Some(false),
                                                None,
                                                None,
                                                Some(true), // Check cache
                                                Some(false),
                                                Some(true), // Use custom search engines
                                            ).await {
                                                Ok(response) => {
                                                    if let Some(data) = response.data {
                                                        total_count += data.total_nzbs;
                                                        for usenet in data.nzbs {
                                                            all_results.push(SearchResultItem::Usenet(usenet));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    if !has_error {
                                                        has_error = true;
                                                        error_msg = format!("Custom indexer usenet search failed: {}", e);
                                                    }
                                                }
                                            }
                                        } else {
                                            match search_type_value {
                                                // IMDB case is handled earlier in the outer match
                                                SearchType::IMDB => {}
                                                SearchType::Torrents => {
                                                    match client.search_torrents(
                                                        query_clone.clone(),
                                                        Some(false),
                                                        Some(true), // Check cache
                                                        Some(false),
                                                        None, // Don't use custom engines in regular mode
                                                    ).await {
                                                        Ok(response) => {
                                                            if let Some(data) = response.data {
                                                                total_count += data.total_torrents;
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
                                                        error_msg = "Usenet search requires Pro plan".to_string();
                                                    } else {
                                                        match client.search_usenet(
                                                            query_clone.clone(),
                                                            Some(false),
                                                            None,
                                                            None,
                                                            Some(true), // Check cache
                                                            Some(false),
                                                            None, // Don't use custom engines in regular mode
                                                        ).await {
                                                            Ok(response) => {
                                                                if let Some(data) = response.data {
                                                                    total_count += data.total_nzbs;
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
                                                        Some(true), // Check cache
                                                        Some(false),
                                                        None, // Don't use custom engines in regular mode
                                                    ).await {
                                                        Ok(response) => {
                                                            if let Some(data) = response.data {
                                                                total_count += data.total_torrents;
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
                                                            Some(true), // Check cache
                                                            Some(false),
                                                            None, // Don't use custom engines in regular mode
                                                        ).await {
                                                            Ok(response) => {
                                                                if let Some(data) = response.data {
                                                                    total_count += data.total_nzbs;
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
                                    }
                                }
                                
                                // Get displayed count before moving all_results
                                let displayed_count = all_results.len() as i32;
                                results_signal.set(all_results);
                                // Update total count - use displayed results count if API doesn't provide accurate total
                                total_results_count.set(if total_count > 0 { total_count } else { displayed_count });
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
    
    let handle_download = {
        let downloading_items_clone = downloading_items.clone();
        move |item: SearchResultItem| {
            let item_hash = item.hash();
            let downloading_items_local = downloading_items_clone.clone();
            
            downloading_items_local.update(|set| {
                set.insert(item_hash.clone());
            });
            
            spawn_local(async move {
                let cleanup = || {
                    downloading_items_local.update(|set| {
                        set.remove(&item_hash);
                    });
                };
                
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
                                    
                                    cleanup();
                                    return;
                                }
                            }
                        }
                    }
                }
                
                cleanup();
            });
        }
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
    
    let handle_copy_hash = move |item: SearchResultItem| {
        let hash = item.hash();
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    let clipboard = window.navigator().clipboard();
                    let promise = clipboard.write_text(&hash);
                    if let Ok(_) = JsFuture::from(promise).await {
                        web_sys::console::log_1(&"Hash copied to clipboard!".into());
                    }
                }
            }
        });
    };
    
    let handle_copy_nzb = move |item: SearchResultItem| {
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(nzb) = item.nzb() {
                    if let Some(window) = web_sys::window() {
                        let clipboard = window.navigator().clipboard();
                        let promise = clipboard.write_text(&nzb);
                        if let Ok(_) = JsFuture::from(promise).await {
                            web_sys::console::log_1(&"NZB link copied to clipboard!".into());
                        }
                    }
                }
            }
        });
    };
    
    let expanded_search_dropdowns = RwSignal::new(std::collections::HashSet::<String>::new());
    
    let toggle_search_dropdown = move |item_hash: String| {
        expanded_search_dropdowns.update(|set| {
            if set.contains(&item_hash) {
                set.remove(&item_hash);
            } else {
                set.insert(item_hash);
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
    
    let grouped_results = move || {
        let results = sorted_results();
        let mut groups: std::collections::HashMap<String, (String, Vec<SearchResultItem>)> = std::collections::HashMap::new();
        
        for item in results {
            let parsed_title = item.parsed_title();
            let group_key = parsed_title.to_lowercase();
            let entry = groups.entry(group_key).or_insert_with(|| (parsed_title.clone(), Vec::new()));
            entry.1.push(item);
        }
        
        let mut grouped: Vec<(String, Vec<SearchResultItem>)> = groups.into_iter()
            .map(|(_, (title, items))| (title, items))
            .collect();
        grouped.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        grouped
    };
    
    let toggle_group = move |group_title: String| {
        expanded_groups.update(|set| {
            let key = group_title.to_lowercase();
            if set.contains(&key) {
                set.remove(&key);
            } else {
                set.insert(key);
            }
        });
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
                    <div class="flex gap-3 items-stretch">
                        <div class="flex-1 relative">
                            <input
                                type="text"
                                placeholder=move || {
                                    if use_custom_indexers.get() && has_plan_2() {
                                        "Search custom indexers"
                                    } else {
                                        match search_type.get() {
                                            SearchType::Torrents => "Search torrents",
                                            SearchType::Usenet => "Search usenet",
                                            SearchType::Both => "Search torrents and usenet",
                                            SearchType::IMDB => "Enter IMDB ID (e.g., tt5151761)",
                                        }
                                    }
                                }
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
                                    total_results_count.set(0);
                                    search_error.set(None);
                                    expanded_groups.update(|set| set.clear());
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
                        <div class="flex items-center gap-2 sm:gap-3 w-full sm:w-auto">
                            <span class="text-xs sm:text-sm font-medium shrink-0" style="color: var(--text-secondary);">"Type:"</span>
                            <div class="flex gap-0.5 rounded-lg" style="background-color: var(--bg-secondary); border: 1px solid var(--border-secondary);">
                            <button
                                class="px-2.5 sm:px-4 py-1.5 sm:py-2 text-xs sm:text-sm font-medium transition-all min-h-[36px] sm:min-h-0"
                                style=move || {
                                    let is_active = search_type.get() == SearchType::Torrents;
                                    let classes = if is_active {
                                        "background-color: var(--accent-primary); color: white; border-radius: 0.5rem 0 0 0.5rem;"
                                    } else {
                                        "background-color: transparent; color: var(--text-secondary); border-radius: 0.5rem 0 0 0.5rem;"
                                    };
                                    classes
                                }
                                on:click=move |_| save_search_type(SearchType::Torrents)
                            >
                                "Torrents"
                            </button>
                            <button
                                class="px-2.5 sm:px-4 py-1.5 sm:py-2 text-xs sm:text-sm font-medium transition-all min-h-[36px] sm:min-h-0"
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
                                class="px-2.5 sm:px-4 py-1.5 sm:py-2 text-xs sm:text-sm font-medium transition-all min-h-[36px] sm:min-h-0"
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
                            <button
                                class="px-2.5 sm:px-4 py-1.5 sm:py-2 text-xs sm:text-sm font-medium transition-all min-h-[36px] sm:min-h-0"
                                style=move || {
                                    let is_active = search_type.get() == SearchType::IMDB;
                                    let classes = if is_active {
                                        "background-color: var(--accent-primary); color: white; border-radius: 0 0.5rem 0.5rem 0;"
                                    } else {
                                        "background-color: transparent; color: var(--text-secondary); border-radius: 0 0.5rem 0.5rem 0;"
                                    };
                                    classes
                                }
                                on:click=move |_| save_search_type(SearchType::IMDB)
                            >
                                "IMDB"
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
                    <div class="px-4 md:px-6 py-3 border-b" style="border-color: var(--border-secondary); background-color: var(--bg-tertiary);">
                        <div class="flex items-center justify-between">
                            <span class="text-sm font-medium" style="color: var(--text-primary);">
                                {move || {
                                    let displayed = search_results.get().len();
                                    let total = total_results_count.get();
                                    if total > displayed as i32 {
                                        format!("Showing {} of {} results", displayed, total)
                                    } else {
                                        format!("{} result{}", displayed, if displayed == 1 { "" } else { "s" })
                                    }
                                }}
                            </span>
                        </div>
                    </div>
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
                                grouped_results().into_iter().map(|(group_title, items)| {
                                    let group_title_clone = group_title.clone();
                                    let group_title_for_toggle = group_title.clone();
                                    let group_title_lower = group_title.to_lowercase();
                                    let group_title_lower2 = group_title_lower.clone();
                                    let expanded_groups_clone = expanded_groups.clone();
                                    let expanded_groups_clone2 = expanded_groups.clone();
                                    let is_expanded = move || expanded_groups_clone.get().contains(&group_title_lower);
                                    let is_expanded2 = move || expanded_groups_clone2.get().contains(&group_title_lower2);
                                    let items_count = items.len();
                                    let downloading_items_clone = downloading_items.clone();
                                    let handle_download_clone = handle_download.clone();
                                    let handle_copy_magnet_clone = handle_copy_magnet.clone();
                                    
                                    view! {
                                        <>
                                            <tr
                                                class="border-b transition-colors cursor-pointer hover:bg-opacity-10"
                                                style="border-color: var(--border-secondary); background-color: var(--bg-secondary);"
                                                on:click=move |_| toggle_group(group_title_for_toggle.clone())
                                            >
                                                <td class="px-4 md:px-6 py-3" colspan="5" style="color: var(--text-primary);">
                                                    <div class="flex items-center gap-3">
                                                        <svg
                                                            class="w-5 h-5 transition-transform"
                                                            style={move || if is_expanded() { "transform: rotate(90deg);" } else { "transform: rotate(0deg);" }}
                                                            fill="none"
                                                            stroke="currentColor"
                                                            viewBox="0 0 24 24"
                                                        >
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                        </svg>
                                                        <span class="font-semibold text-base" style="color: var(--text-primary);">
                                                            {group_title_clone.clone()}
                                                        </span>
                                                        <span class="text-sm" style="color: var(--text-secondary);">
                                                            {format!("({} result{})", items_count, if items_count == 1 { "" } else { "s" })}
                                                        </span>
                                                    </div>
                                                </td>
                                            </tr>
                                            {move || {
                                                if is_expanded2() {
                                                    let items_clone = items.clone();
                                                    items_clone.into_iter().map(|item| {
                                                        let item_download = item.clone();
                                                        let item_magnet_check = item.clone();
                                                        let item_copy_magnet = item.clone();
                                                        let item_title = item.clone();
                                                        let item_title_for_raw = item.clone();
                                                        let item_size = item.clone();
                                                        let item_seeders = item.clone();
                                                        let item_type = item.clone();
                                                        
                                                        let (details, is_cached): (Vec<String>, bool) = match &item {
                                                            SearchResultItem::Torrent(t) => {
                                                                let cached = t.cached.unwrap_or(false);
                                                                let d = if let Some(tpd) = &t.title_parsed_data {
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
                                                                };
                                                                (d, cached)
                                                            }
                                                            SearchResultItem::Usenet(u) => {
                                                                let cached = u.cached.unwrap_or(false);
                                                                let d = if let Some(tpd) = &u.title_parsed_data {
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
                                                                };
                                                                (d, cached)
                                                            }
                                                        };
                                                        let has_details = !details.is_empty();
                                                        let details_for_view = details.clone();
                                                        let cached_for_view = is_cached;
                                                        let has_magnet = item_magnet_check.magnet().is_some();
                                                        let item_copy_for_magnet = item_copy_magnet.clone();
                                                        
                                                        view! {
                                                            <tr
                                                                class="border-b transition-colors hover:bg-opacity-5"
                                                                style="border-color: var(--border-secondary);"
                                                            >
                                                                <td class="px-4 md:px-6 py-4" style="color: var(--text-primary);">
                                                                    <div class="flex flex-col gap-2">
                                                                        <div class="flex items-center gap-2 flex-wrap">
                                                                            <span class="font-semibold text-sm sm:text-base leading-tight" style="color: var(--text-primary);">{item_title.title()}</span>
                                                                            {move || {
                                                                                match &item_title {
                                                                                    SearchResultItem::Torrent(t) => {
                                                                                        if t.private.unwrap_or(false) {
                                                                                            view! {
                                                                                                <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" title="Private Tracker" style="color: #f97316;">
                                                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                                                                                                </svg>
                                                                                            }.into_any()
                                                                                        } else {
                                                                                            view! {}.into_any()
                                                                                        }
                                                                                    }
                                                                                    SearchResultItem::Usenet(_) => view! {}.into_any(),
                                                                                }
                                                                            }}
                                                                        </div>
                                                                        <div class="flex flex-col gap-1.5">
                                                                            <span class="text-xs sm:text-sm font-mono leading-relaxed" style="color: var(--text-secondary); opacity: 0.85; word-break: break-all;">
                                                                                {move || {
                                                                                    match &item_title_for_raw {
                                                                                        SearchResultItem::Torrent(t) => t.raw_title.clone(),
                                                                                        SearchResultItem::Usenet(u) => u.raw_title.clone(),
                                                                                    }
                                                                                }}
                                                                            </span>
                                                                            <Show when=move || has_details || cached_for_view>
                                                                                <div class="flex gap-1.5 flex-wrap">
                                                                                    {move || {
                                                                                        if cached_for_view {
                                                                                            view! {
                                                                                                <div class="flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(34, 197, 94, 0.15); color: #4ade80; border: 1px solid rgba(34, 197, 94, 0.3);" title="Cached - Ready to download instantly">
                                                                                                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                                                                                                    </svg>
                                                                                                    <span>"Cached"</span>
                                                                                                </div>
                                                                                            }.into_any()
                                                                                        } else {
                                                                                            view! {}.into_any()
                                                                                        }
                                                                                    }}
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
                                                                        SearchResultItem::Torrent(_) => {
                                                                            view! {
                                                                                <span class="px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(59, 130, 246, 0.1); color: #60a5fa; border: 1px solid rgba(59, 130, 246, 0.3);">
                                                                                    "Torrent"
                                                                                </span>
                                                                            }.into_any()
                                                                        }
                                                                        SearchResultItem::Usenet(_) => {
                                                                            view! {
                                                                                <span class="px-2 py-1 rounded-md text-xs font-medium" style="background-color: rgba(168, 85, 247, 0.1); color: #c084fc; border: 1px solid rgba(168, 85, 247, 0.3);">
                                                                                    "Usenet"
                                                                                </span>
                                                                            }.into_any()
                                                                        }
                                                                    }}
                                                                </td>
                                                                <td class="px-4 md:px-6 py-4">
                                                                    <div class="flex items-center gap-2">
                                                                        {move || {
                                                                            let item_hash_for_check = item_download.hash();
                                                                            let is_downloading = downloading_items_clone.get().contains(&item_hash_for_check);
                                                                            let item_download_clone = item_download.clone();
                                                                            
                                                                            if is_downloading {
                                                                                view! {
                                                                                    <button
                                                                                        class="p-2 rounded-lg transition-all flex items-center justify-center"
                                                                                        style="background-color: transparent; color: var(--accent-secondary); opacity: 0.6; cursor: not-allowed;"
                                                                                        disabled=true
                                                                                        title="Downloading..."
                                                                                    >
                                                                                        <LoadingSpinner size=SpinnerSize::Small variant=SpinnerVariant::Default/>
                                                                                    </button>
                                                                                }.into_any()
                                                                            } else {
                                                                                view! {
                                                                                    <button
                                                                                        class="p-2 rounded-lg transition-all flex items-center justify-center hover:opacity-80 hover:scale-105"
                                                                                        style="background-color: transparent; color: var(--accent-secondary);"
                                                                                        on:click=move |_| handle_download_clone(item_download_clone.clone())
                                                                                        title="Download"
                                                                                    >
                                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                                                                                        </svg>
                                                                                    </button>
                                                                                }.into_any()
                                                                            }
                                                                        }}
                                                                        {
                                                                            let item_hash_dropdown = item_copy_for_magnet.hash();
                                                                            let item_for_dropdown = item_copy_for_magnet.clone();
                                                                            let item_hash_for_toggle = item_hash_dropdown.clone();
                                                                            let item_copy_hash = item_for_dropdown.clone();
                                                                            let item_copy_nzb = item_for_dropdown.clone();
                                                                            let item_copy_magnet_dropdown = item_for_dropdown.clone();
                                                                            let has_magnet_dropdown = item_for_dropdown.magnet().is_some();
                                                                            let has_nzb_dropdown = item_for_dropdown.nzb().is_some();
                                                                            let expanded_dropdowns_clone = expanded_search_dropdowns.clone();
                                                                            
                                                                            view! {
                                                                                <div class="relative inline-block text-left" on:click=|ev| ev.stop_propagation()>
                                                                                    <button
                                                                                        class="p-2 rounded-lg transition-all flex items-center justify-center hover:opacity-80 hover:scale-105"
                                                                                        style="background-color: transparent; color: var(--text-secondary);"
                                                                                        title="More options"
                                                                                        on:click={
                                                                                            let item_hash_toggle = item_hash_for_toggle.clone();
                                                                                            let expanded_dropdowns_toggle = expanded_search_dropdowns.clone();
                                                                                            move |_| {
                                                                                                let item_hash_toggle_clone = item_hash_toggle.clone();
                                                                                                toggle_search_dropdown(item_hash_toggle_clone)
                                                                                            }
                                                                                        }
                                                                                    >
                                                                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"></path>
                                                                                        </svg>
                                                                                    </button>
                                                                                    <Show when=move || expanded_dropdowns_clone.get().contains(&item_hash_dropdown)>
                                                                                        <div class="absolute z-50 mt-2 w-48 rounded-md shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" style="background: var(--bg-tertiary); border: 1px solid var(--border-secondary); left: 0; transform: translateX(-100%);" on:click=|ev| ev.stop_propagation()>
                                                                                            <div class="py-1">
                                                                                                <button
                                                                                                    class="block w-full px-4 py-2 text-sm transition-colors hover:opacity-80 text-left"
                                                                                                    style="color: var(--text-secondary);"
                                                                                                    on:click={
                                                                                                        let item_hash_for_close = item_hash_for_toggle.clone();
                                                                                                        let expanded_dropdowns_close = expanded_search_dropdowns.clone();
                                                                                                        let item_copy_hash_btn = item_copy_hash.clone();
                                                                                                        move |_| {
                                                                                                            handle_copy_hash(item_copy_hash_btn.clone());
                                                                                                            expanded_dropdowns_close.update(|set| {
                                                                                                                set.remove(&item_hash_for_close);
                                                                                                            });
                                                                                                        }
                                                                                                    }
                                                                                                >
                                                                                                    "Copy Hash"
                                                                                                </button>
                                                                                                {if has_magnet_dropdown {
                                                                    let item_copy_magnet_btn = item_copy_magnet_dropdown.clone();
                                                                    let item_hash_for_close_magnet = item_hash_for_toggle.clone();
                                                                    let expanded_dropdowns_close_magnet = expanded_search_dropdowns.clone();
                                                                    view! {
                                                                        <button
                                                                            class="block w-full px-4 py-2 text-sm transition-colors hover:opacity-80 text-left"
                                                                            style="color: var(--text-secondary);"
                                                                            on:click=move |_| {
                                                                                handle_copy_magnet_clone(item_copy_magnet_btn.clone());
                                                                                expanded_dropdowns_close_magnet.update(|set| {
                                                                                    set.remove(&item_hash_for_close_magnet);
                                                                                });
                                                                            }
                                                                        >
                                                                            "Copy Magnet Link"
                                                                        </button>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {}.into_any()
                                                                }}
                                                                {if has_nzb_dropdown {
                                                                    let item_copy_nzb_btn = item_copy_nzb.clone();
                                                                    let item_hash_for_close_nzb = item_hash_for_toggle.clone();
                                                                    let expanded_dropdowns_close_nzb = expanded_search_dropdowns.clone();
                                                                    view! {
                                                                        <button
                                                                            class="block w-full px-4 py-2 text-sm transition-colors hover:opacity-80 text-left"
                                                                            style="color: var(--text-secondary);"
                                                                            on:click=move |_| {
                                                                                handle_copy_nzb(item_copy_nzb_btn.clone());
                                                                                expanded_dropdowns_close_nzb.update(|set| {
                                                                                    set.remove(&item_hash_for_close_nzb);
                                                                                });
                                                                            }
                                                                        >
                                                                            "Copy NZB Link"
                                                                        </button>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {}.into_any()
                                                                }}
                                                                                            </div>
                                                                                        </div>
                                                                                    </Show>
                                                                                </div>
                                                                            }.into_any()
                                                                        }
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect::<Vec<_>>()
                                                } else {
                                                    Vec::new()
                                                }
                                            }}
                                        </>
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

