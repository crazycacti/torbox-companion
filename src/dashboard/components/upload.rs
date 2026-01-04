use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::logging::log;
#[cfg(target_arch = "wasm32")]
use web_sys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use crate::api::{TorboxClient, CreateTorrentRequest, CreateUsenetDownloadRequest, CreateWebDownloadRequest};
use crate::dashboard::components::loading_spinner::{LoadingSpinner, SpinnerSize, SpinnerVariant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UploadType {
    Torrents,
    NZB,
    WebLinks,
}

#[component]
pub fn UploadComponent() -> impl IntoView {
    let is_collapsed = RwSignal::new(true);
    let upload_type = RwSignal::new(UploadType::Torrents);
    let torrent_links = RwSignal::new(String::new());
    let nzb_links = RwSignal::new(String::new());
    let web_links = RwSignal::new(String::new());
    
    let is_uploading = RwSignal::new(false);
    let upload_progress = RwSignal::new(0);
    let upload_total = RwSignal::new(0);
    
    let seed_duration = RwSignal::new(1i32);
    let allow_zip = RwSignal::new(false);
    let as_queued = RwSignal::new(false);
    let add_only_if_cached = RwSignal::new(false);
    
    let is_dragging = RwSignal::new(false);
    let upload_errors = RwSignal::new(Vec::<(String, String)>::new());
    
    let toggle_collapse = move |_| {
        is_collapsed.update(|c| *c = !*c);
    };
    
    #[cfg(target_arch = "wasm32")]
    let handle_drop = {
        let is_dragging_clone = is_dragging.clone();
        let torrent_links_clone = torrent_links.clone();
        let nzb_links_clone = nzb_links.clone();
        let web_links_clone = web_links.clone();
        move |ev: web_sys::DragEvent, upload_type_val: UploadType| {
            ev.prevent_default();
            is_dragging_clone.set(false);
            
            if let Some(data_transfer) = ev.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    let mut file_names = Vec::new();
                    for i in 0..files.length() {
                        if let Some(file) = files.get(i) {
                            let file: web_sys::File = file.dyn_into().unwrap();
                            let file_name = file.name();
                            
                            // Check file extension matches upload type
                            let matches = match upload_type_val {
                                UploadType::Torrents => file_name.ends_with(".torrent"),
                                UploadType::NZB => file_name.ends_with(".nzb"),
                                UploadType::WebLinks => false,
                            };
                            
                            if matches {
                                file_names.push(file_name);
                            }
                        }
                    }
                    
                    if !file_names.is_empty() {
                        let files_signal = match upload_type_val {
                            UploadType::Torrents => torrent_links_clone.clone(),
                            UploadType::NZB => nzb_links_clone.clone(),
                            UploadType::WebLinks => web_links_clone.clone(),
                        };
                        files_signal.set(file_names.join("\n"));
                    }
                }
            }
        }
    };
    
    #[cfg(not(target_arch = "wasm32"))]
    let handle_drop = move |_ev: (), _upload_type_val: UploadType| {};
    
    #[cfg(target_arch = "wasm32")]
    let handle_drag_over = {
        let is_dragging_clone = is_dragging.clone();
        move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            is_dragging_clone.set(true);
        }
    };
    
    #[cfg(not(target_arch = "wasm32"))]
    let handle_drag_over = move |_ev: ()| {};
    
    #[cfg(target_arch = "wasm32")]
    let handle_drag_leave = {
        let is_dragging_clone = is_dragging.clone();
        move |_: ()| {
            is_dragging_clone.set(false);
        }
    };
    
    #[cfg(not(target_arch = "wasm32"))]
    let handle_drag_leave = move |_: ()| {};
    
    let parse_links = move |text: String| -> Vec<String> {
        text.lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    };
    
    let handle_file_select = move |upload_type_val: UploadType| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let input = document.create_element("input").ok();
                    if let Some(input) = input {
                        let input: web_sys::HtmlInputElement = input.dyn_into().ok().unwrap();
                        input.set_type("file");
                        input.set_multiple(true);
                        
                        match upload_type_val {
                            UploadType::Torrents => {
                                input.set_accept(".torrent");
                            }
                            UploadType::NZB => {
                                input.set_accept(".nzb");
                            }
                            UploadType::WebLinks => { 
                                return;
                            }
                        }
                        
                        let files_signal = match upload_type_val {
                            UploadType::Torrents => torrent_links.clone(),
                            UploadType::NZB => nzb_links.clone(),
                            UploadType::WebLinks => web_links.clone(),
                        };
                        
                        let on_change = wasm_bindgen::closure::Closure::wrap(Box::new(move |ev: web_sys::Event| {
                            let target = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
                            if let Some(input_elem) = target {
                                if let Some(files) = input_elem.files() {
                                    let mut file_names = Vec::new();
                                    for i in 0..files.length() {
                                        if let Some(file) = files.get(i) {
                                            let file: web_sys::File = file.dyn_into().unwrap();
                                            file_names.push(file.name());
                                        }
                                    }
                                    if !file_names.is_empty() {
                                        files_signal.set(file_names.join("\n"));
                                    }
                                }
                            }
                        }) as Box<dyn FnMut(web_sys::Event)>);
                        
                        input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
                        on_change.forget();
                        
                        let _ = input.click();
                    }
                }
            }
        }
    };
    
    let process_uploads = move || {
        if is_uploading.get() {
            return;
        }
        
        is_uploading.set(true);
        upload_progress.set(0);
        upload_errors.set(Vec::new());
        
        let links_to_process = match upload_type.get() {
            UploadType::Torrents => parse_links(torrent_links.get()),
            UploadType::NZB => parse_links(nzb_links.get()),
            UploadType::WebLinks => parse_links(web_links.get()),
        };
        
        let total_items = links_to_process.len();
        upload_total.set(total_items as i32);
        
        if total_items == 0 {
            is_uploading.set(false);
            return;
        }
        
        let upload_type_val = upload_type.get();
        let seed_duration_val = seed_duration.get();
        let allow_zip_val = allow_zip.get();
        let as_queued_val = as_queued.get();
        let add_only_if_cached_val = add_only_if_cached.get();
        
        let progress_signal = upload_progress.clone();
        let is_uploading_signal = is_uploading.clone();
        let errors_signal = upload_errors.clone();
        
        spawn_local(async move {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if let Ok(Some(api_key)) = storage.get_item("api_key") {
                            if !api_key.is_empty() {
                                let client = TorboxClient::new(api_key);
                                let mut processed = 0;
                            

                                for link in links_to_process.iter() {
                                    match upload_type_val {
                                        UploadType::Torrents => {
                                            let request = CreateTorrentRequest {
                                                file: None,
                                                magnet: Some(link.clone()),
                                                seed: Some(seed_duration_val),
                                                allow_zip: Some(allow_zip_val),
                                                name: None,
                                                as_queued: Some(as_queued_val),
                                                add_only_if_cached: Some(add_only_if_cached_val),
                                            };
                                            
                                            match client.async_create_torrent(request).await {
                                                Ok(_) => {
                                                    processed += 1;
                                                    progress_signal.set(processed);
                                                }
                                                Err(e) => {
                                                    log!("Failed to upload torrent link: {:?}", e);
                                                    let error_msg = format!("{}", e);
                                                    let status_code = match &e {
                                                        crate::api::ApiError::HttpError { status_code, .. } => format!("Status: {}", status_code),
                                                        crate::api::ApiError::RateLimitError => "Status: 429".to_string(),
                                                        crate::api::ApiError::AuthenticationError => "Status: 401".to_string(),
                                                        crate::api::ApiError::ValidationError => "Status: 400".to_string(),
                                                        crate::api::ApiError::NotFoundError => "Status: 404".to_string(),
                                                        crate::api::ApiError::ServerError => "Status: 500".to_string(),
                                                        _ => "Unknown error".to_string(),
                                                    };
                                                    errors_signal.update(|errors| {
                                                        errors.push((link.clone(), format!("{} ({})", error_msg, status_code)));
                                                    });
                                                    processed += 1;
                                                    progress_signal.set(processed);
                                                }
                                            }
                                        }
                                        UploadType::NZB => {
                                            let request = CreateUsenetDownloadRequest {
                                                file: None,
                                                link: Some(link.clone()),
                                                name: None,
                                                password: None,
                                                post_processing: None,
                                                as_queued: Some(as_queued_val),
                                                add_only_if_cached: Some(add_only_if_cached_val),
                                            };
                                            
                                            match client.async_create_usenet_download(request).await {
                                                Ok(_) => {
                                                    processed += 1;
                                                    progress_signal.set(processed);
                                                }
                                                Err(e) => {
                                                    log!("Failed to upload NZB link: {:?}", e);
                                                    let error_msg = format!("{}", e);
                                                    let status_code = match &e {
                                                        crate::api::ApiError::HttpError { status_code, .. } => format!("Status: {}", status_code),
                                                        crate::api::ApiError::RateLimitError => "Status: 429".to_string(),
                                                        crate::api::ApiError::AuthenticationError => "Status: 401".to_string(),
                                                        crate::api::ApiError::ValidationError => "Status: 400".to_string(),
                                                        crate::api::ApiError::NotFoundError => "Status: 404".to_string(),
                                                        crate::api::ApiError::ServerError => "Status: 500".to_string(),
                                                        _ => "Unknown error".to_string(),
                                                    };
                                                    errors_signal.update(|errors| {
                                                        errors.push((link.clone(), format!("{} ({})", error_msg, status_code)));
                                                    });
                                                    processed += 1;
                                                    progress_signal.set(processed);
                                                }
                                            }
                                        }
                                        UploadType::WebLinks => {
                                            let request = CreateWebDownloadRequest {
                                                link: link.clone(),
                                                password: None,
                                                name: None,
                                                as_queued: Some(as_queued_val),
                                                add_only_if_cached: Some(add_only_if_cached_val),
                                            };
                                            
                                            match client.async_create_web_download(request).await {
                                                Ok(_) => {
                                                    processed += 1;
                                                    progress_signal.set(processed);
                                                }
                                                Err(e) => {
                                                    log!("Failed to upload web link: {:?}", e);
                                                    let error_msg = format!("{}", e);
                                                    let status_code = match &e {
                                                        crate::api::ApiError::HttpError { status_code, .. } => format!("Status: {}", status_code),
                                                        crate::api::ApiError::RateLimitError => "Status: 429".to_string(),
                                                        crate::api::ApiError::AuthenticationError => "Status: 401".to_string(),
                                                        crate::api::ApiError::ValidationError => "Status: 400".to_string(),
                                                        crate::api::ApiError::NotFoundError => "Status: 404".to_string(),
                                                        crate::api::ApiError::ServerError => "Status: 500".to_string(),
                                                        _ => "Unknown error".to_string(),
                                                    };
                                                    errors_signal.update(|errors| {
                                                        errors.push((link.clone(), format!("{} ({})", error_msg, status_code)));
                                                    });
                                                    processed += 1;
                                                    progress_signal.set(processed);
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                if errors_signal.get().is_empty() {
                                    match upload_type_val {
                                        UploadType::Torrents => {
                                            torrent_links.set(String::new());
                                        }
                                        UploadType::NZB => {
                                            nzb_links.set(String::new());
                                        }
                                        UploadType::WebLinks => {
                                            web_links.set(String::new());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            is_uploading_signal.set(false);
        });
    };
    
    view! {
        <div class="w-full mb-4 sm:mb-6" style="position: relative; z-index: 5;">
            <div class="rounded-xl border" style="background-color: var(--bg-card); border-color: var(--border-secondary); box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);">
                <div class="px-4 py-4">
                    <div class="flex items-center justify-between gap-4 cursor-pointer" on:click=toggle_collapse style="user-select: none;">
                        <h3 class="text-base lg:text-lg font-semibold" style="color: var(--text-primary); pointer-events: none;">
                            "Upload"
                        </h3>
                        <div class="text-lg transition-transform" style={move || if is_collapsed.get() { "transform: rotate(0deg); color: var(--text-secondary); pointer-events: none;" } else { "transform: rotate(180deg); color: var(--text-secondary); pointer-events: none;" }}>
                            "▼"
                        </div>
                    </div>
                </div>
                
                <Show when=move || !is_collapsed.get()>
                    <div class="px-4 pb-4 border-t" style="border-color: var(--border-secondary);">
                        <div class="pt-4 flex flex-col gap-4">
                            <div class="flex items-center gap-2 sm:gap-3 w-full flex-wrap">
                                <span class="text-xs sm:text-sm font-medium shrink-0" style="color: var(--text-secondary);">"Type:"</span>
                                <div class="flex gap-0.5 rounded-lg" style="background-color: var(--bg-secondary); border: 1px solid var(--border-secondary);">
                                    <button
                                        class="px-2.5 sm:px-4 py-1.5 sm:py-2 text-xs sm:text-sm font-medium transition-all min-h-[36px] sm:min-h-0"
                                        style=move || {
                                            let is_active = upload_type.get() == UploadType::Torrents;
                                            if is_active { "background-color: var(--accent-primary); color: white; border-radius: 0.5rem 0 0 0.5rem;" } else { "background-color: transparent; color: var(--text-secondary); border-radius: 0.5rem 0 0 0.5rem;" }
                                        }
                                        on:click=move |_| {
                                            upload_type.set(UploadType::Torrents);
                                            upload_errors.set(Vec::new());
                                        }
                                    >
                                        "Torrents"
                                    </button>
                                    <button
                                        class="px-2.5 sm:px-4 py-1.5 sm:py-2 text-xs sm:text-sm font-medium transition-all min-h-[36px] sm:min-h-0"
                                        style=move || {
                                            let is_active = upload_type.get() == UploadType::NZB;
                                            if is_active { "background-color: var(--accent-primary); color: white;" } else { "background-color: transparent; color: var(--text-secondary);" }
                                        }
                                        on:click=move |_| {
                                            upload_type.set(UploadType::NZB);
                                            upload_errors.set(Vec::new());
                                        }
                                    >
                                        "NZB"
                                    </button>
                                    <button
                                        class="px-2.5 sm:px-4 py-1.5 sm:py-2 text-xs sm:text-sm font-medium transition-all min-h-[36px] sm:min-h-0"
                                        style=move || {
                                            let is_active = upload_type.get() == UploadType::WebLinks;
                                            if is_active { "background-color: var(--accent-primary); color: white; border-radius: 0 0.5rem 0.5rem 0;" } else { "background-color: transparent; color: var(--text-secondary); border-radius: 0 0.5rem 0.5rem 0;" }
                                        }
                                        on:click=move |_| {
                                            upload_type.set(UploadType::WebLinks);
                                            upload_errors.set(Vec::new());
                                        }
                                    >
                                        "Web Links"
                                    </button>
                                </div>
                            </div>
                            
                            {move || {
                                let upload_type_val = upload_type.get();
                                let is_dragging_val = is_dragging.get();
                                match upload_type_val {
                                    UploadType::Torrents => view! {
                                        <div class="flex flex-col gap-4">
                                            <div 
                                                class="border-2 border-dashed rounded-lg p-6 transition-all cursor-pointer"
                                                style=move || {
                                                    if is_dragging_val {
                                                        "border-color: var(--accent-primary); background-color: rgba(var(--accent-primary-rgb), 0.1);"
                                                    } else {
                                                        "border-color: var(--border-primary); background-color: var(--bg-secondary);"
                                                    }
                                                }
                                                on:dragover=move |ev| {
                                                    #[cfg(target_arch = "wasm32")]
                                                    {
                                                        if let Some(drag_ev) = ev.dyn_ref::<web_sys::DragEvent>() {
                                                            drag_ev.prevent_default();
                                                            is_dragging.set(true);
                                                        }
                                                    }
                                                }
                                                on:dragleave=move |_| is_dragging.set(false)
                                                on:drop=move |ev| {
                                                    #[cfg(target_arch = "wasm32")]
                                                    {
                                                        if let Some(drag_ev) = ev.dyn_ref::<web_sys::DragEvent>() {
                                                            drag_ev.prevent_default();
                                                            is_dragging.set(false);
                                                            if let Some(data_transfer) = drag_ev.data_transfer() {
                                                                if let Some(files) = data_transfer.files() {
                                                                    let mut file_names = Vec::new();
                                                                    for i in 0..files.length() {
                                                                        if let Some(file) = files.get(i) {
                                                                            let file: web_sys::File = file.dyn_into().unwrap();
                                                                            let file_name = file.name();
                                                                            if file_name.ends_with(".torrent") {
                                                                                file_names.push(file_name);
                                                                            }
                                                                        }
                                                                    }
                                                                    if !file_names.is_empty() {
                                                                        torrent_links.set(file_names.join("\n"));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                on:click=move |_| handle_file_select(UploadType::Torrents)
                                            >
                                                <div class="flex flex-col items-center justify-center gap-3 text-center">
                                                    <svg class="w-8 h-8 sm:w-10 sm:h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--text-secondary);">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                                                    </svg>
                                                    <div class="flex flex-col gap-1">
                                                        <span class="text-sm font-medium" style="color: var(--text-primary);">
                                                            "Drop .torrent files here or click to browse"
                                                        </span>
                                                        <span class="text-xs" style="color: var(--text-secondary);">
                                                            "Supports multiple files"
                                                        </span>
                                                    </div>
                                                    <button
                                                        class="px-4 py-2 text-sm font-medium rounded-lg transition-colors"
                                                        style="background-color: var(--accent-primary); color: white;"
                                                        on:click=move |ev| {
                                                            ev.stop_propagation();
                                                            handle_file_select(UploadType::Torrents);
                                                        }
                                                    >
                                                        "Browse Files"
                                                    </button>
                                                </div>
                                            </div>
                                            
                                            <div class="flex flex-col gap-2">
                                                <label class="text-sm font-medium" style="color: var(--text-primary);">"Or paste magnet links (one per line):"</label>
                                                <textarea
                                                    class="w-full px-4 py-3 rounded-lg border transition-all text-sm resize-y min-h-[100px]"
                                                    style="background-color: var(--bg-secondary); border-color: var(--border-primary); color: var(--text-primary);"
                                                    prop:value=move || torrent_links.get()
                                                    on:input=move |ev| torrent_links.set(event_target_value(&ev))
                                                    placeholder="magnet:?xt=urn:btih:..."
                                                ></textarea>
                                            </div>
                                            
                                            <div class="flex flex-col gap-2 mt-4">
                                                <label class="flex items-center gap-3 text-sm" style="color: var(--text-primary);">
                                                    <input
                                                        type="checkbox"
                                                        prop:checked=move || allow_zip.get()
                                                        on:change=move |ev| allow_zip.set(event_target_checked(&ev))
                                                        class="form-checkbox h-4 w-4"
                                                        style="color: var(--accent-primary); background-color: var(--bg-secondary); border-color: var(--border-primary);"
                                                    />
                                                    "Allow ZIP compression"
                                                </label>
                                                <label class="flex items-center gap-3 text-sm" style="color: var(--text-primary);">
                                                    <input
                                                        type="checkbox"
                                                        prop:checked=move || as_queued.get()
                                                        on:change=move |ev| as_queued.set(event_target_checked(&ev))
                                                        class="form-checkbox h-4 w-4"
                                                        style="color: var(--accent-primary); background-color: var(--bg-secondary); border-color: var(--border-primary);"
                                                    />
                                                    "Add as queued"
                                                </label>
                                                <label class="flex items-center gap-3 text-sm" style="color: var(--text-primary);">
                                                    <input
                                                        type="checkbox"
                                                        prop:checked=move || add_only_if_cached.get()
                                                        on:change=move |ev| add_only_if_cached.set(event_target_checked(&ev))
                                                        class="form-checkbox h-4 w-4"
                                                        style="color: var(--accent-primary); background-color: var(--bg-secondary); border-color: var(--border-primary);"
                                                    />
                                                    "Add only if cached"
                                                </label>
                                                <div class="flex items-center gap-2 text-sm" style="color: var(--text-primary);">
                                                    <label for="seed-duration">"Seed Duration:"</label>
                                                    <select
                                                        id="seed-duration"
                                                        class="px-2 py-1 rounded-md text-sm border"
                                                        style="background-color: var(--bg-secondary); border-color: var(--border-primary); color: var(--text-primary);"
                                                        prop:value=move || seed_duration.get().to_string()
                                                        on:change=move |ev| {
                                                            if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                                                seed_duration.set(value);
                                                            }
                                                        }
                                                    >
                                                        <option value="0">"No Seeding"</option>
                                                        <option value="1">"Short"</option>
                                                        <option value="2">"Medium"</option>
                                                        <option value="3">"Long"</option>
                                                    </select>
                                                </div>
                                            </div>
                                        </div>
                                }.into_any(),
                                UploadType::NZB => view! {
                                    <div class="flex flex-col gap-4">
                                        <div 
                                            class="border-2 border-dashed rounded-lg p-6 transition-all cursor-pointer"
                                            style=move || {
                                                if is_dragging_val {
                                                    "border-color: var(--accent-primary); background-color: rgba(var(--accent-primary-rgb), 0.1);"
                                                } else {
                                                    "border-color: var(--border-primary); background-color: var(--bg-secondary);"
                                                }
                                            }
                                            on:dragover=move |ev| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    if let Some(drag_ev) = ev.dyn_ref::<web_sys::DragEvent>() {
                                                        drag_ev.prevent_default();
                                                        is_dragging.set(true);
                                                    }
                                                }
                                            }
                                            on:dragleave=move |_| is_dragging.set(false)
                                            on:drop=move |ev| {
                                                #[cfg(target_arch = "wasm32")]
                                                {
                                                    if let Some(drag_ev) = ev.dyn_ref::<web_sys::DragEvent>() {
                                                        drag_ev.prevent_default();
                                                        is_dragging.set(false);
                                                        if let Some(data_transfer) = drag_ev.data_transfer() {
                                                            if let Some(files) = data_transfer.files() {
                                                                let mut file_names = Vec::new();
                                                                for i in 0..files.length() {
                                                                    if let Some(file) = files.get(i) {
                                                                        let file: web_sys::File = file.dyn_into().unwrap();
                                                                        let file_name = file.name();
                                                                        if file_name.ends_with(".nzb") {
                                                                            file_names.push(file_name);
                                                                        }
                                                                    }
                                                                }
                                                                if !file_names.is_empty() {
                                                                    nzb_links.set(file_names.join("\n"));
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            on:click=move |_| handle_file_select(UploadType::NZB)
                                        >
                                            <div class="flex flex-col items-center justify-center gap-3 text-center">
                                                <svg class="w-8 h-8 sm:w-10 sm:h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--text-secondary);">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                                                </svg>
                                                <div class="flex flex-col gap-1">
                                                    <span class="text-sm font-medium" style="color: var(--text-primary);">
                                                        "Drop .nzb files here or click to browse"
                                                    </span>
                                                    <span class="text-xs" style="color: var(--text-secondary);">
                                                        "Supports multiple files"
                                                    </span>
                                                </div>
                                                <button
                                                    class="px-4 py-2 text-sm font-medium rounded-lg transition-colors"
                                                    style="background-color: var(--accent-primary); color: white;"
                                                    on:click=move |ev| {
                                                        ev.stop_propagation();
                                                        handle_file_select(UploadType::NZB);
                                                    }
                                                >
                                                    "Browse Files"
                                                </button>
                                            </div>
                                        </div>
                                        
                                        <div class="flex flex-col gap-2">
                                            <label class="text-sm font-medium" style="color: var(--text-primary);">"Or paste NZB links (one per line):"</label>
                                            <textarea
                                                class="w-full px-4 py-3 rounded-lg border transition-all text-sm resize-y min-h-[100px]"
                                                style="background-color: var(--bg-secondary); border-color: var(--border-primary); color: var(--text-primary);"
                                                prop:value=move || nzb_links.get()
                                                on:input=move |ev| nzb_links.set(event_target_value(&ev))
                                                placeholder="https://example.com/file.nzb"
                                            ></textarea>
                                        </div>
                                        
                                        <div class="flex flex-col gap-2 mt-4">
                                            <label class="flex items-center gap-3 text-sm" style="color: var(--text-primary);">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || as_queued.get()
                                                    on:change=move |ev| as_queued.set(event_target_checked(&ev))
                                                    class="form-checkbox h-4 w-4"
                                                    style="color: var(--accent-primary); background-color: var(--bg-secondary); border-color: var(--border-primary);"
                                                />
                                                "Add as queued"
                                            </label>
                                            <label class="flex items-center gap-3 text-sm" style="color: var(--text-primary);">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || add_only_if_cached.get()
                                                    on:change=move |ev| add_only_if_cached.set(event_target_checked(&ev))
                                                    class="form-checkbox h-4 w-4"
                                                    style="color: var(--accent-primary); background-color: var(--bg-secondary); border-color: var(--border-primary);"
                                                />
                                                "Add only if cached"
                                            </label>
                                        </div>
                                    </div>
                                }.into_any(),
                                UploadType::WebLinks => view! {
                                    <div class="flex flex-col gap-4">
                                        <div class="flex flex-col gap-2">
                                            <label class="text-sm font-medium" style="color: var(--text-primary);">"Web Links (one per line)"</label>
                                            <textarea
                                                class="w-full px-4 py-3 rounded-lg border transition-all text-sm resize-y min-h-[100px]"
                                                style="background-color: var(--bg-secondary); border-color: var(--border-primary); color: var(--text-primary);"
                                                prop:value=move || web_links.get()
                                                on:input=move |ev| web_links.set(event_target_value(&ev))
                                                placeholder="https://example.com/file.zip"
                                            ></textarea>
                                        </div>
                                        
                                        <div class="flex flex-col gap-2 mt-4">
                                            <label class="flex items-center gap-3 text-sm" style="color: var(--text-primary);">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || as_queued.get()
                                                    on:change=move |ev| as_queued.set(event_target_checked(&ev))
                                                    class="form-checkbox h-4 w-4"
                                                    style="color: var(--accent-primary); background-color: var(--bg-secondary); border-color: var(--border-primary);"
                                                />
                                                "Add as queued"
                                            </label>
                                            <label class="flex items-center gap-3 text-sm" style="color: var(--text-primary);">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || add_only_if_cached.get()
                                                    on:change=move |ev| add_only_if_cached.set(event_target_checked(&ev))
                                                    class="form-checkbox h-4 w-4"
                                                    style="color: var(--accent-primary); background-color: var(--bg-secondary); border-color: var(--border-primary);"
                                                />
                                                "Add only if cached"
                                            </label>
                                        </div>
                                    </div>
                                }.into_any()
                                }
                            }}
                            
                            <Show when=move || !upload_errors.get().is_empty()>
                                <div class="rounded-lg border p-3 mb-4" style="background-color: rgba(248, 113, 113, 0.1); border-color: rgba(248, 113, 113, 0.3);">
                                    <div class="flex items-center gap-2 mb-2">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: #f87171;">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <span class="text-sm font-semibold" style="color: #f87171;">"Upload Errors"</span>
                                    </div>
                                    <div class="flex flex-col gap-2">
                                        <For
                                            each=move || upload_errors.get()
                                            key=|(link, _)| link.clone()
                                            children=move |(link, error)| {
                                                view! {
                                                    <div class="text-xs" style="color: var(--text-secondary);">
                                                        <span class="font-medium" style="color: var(--text-primary);">{link}</span>
                                                        <span class="ml-2">{error}</span>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            </Show>
                            
                            <div class="flex items-center justify-between gap-4 pt-2">
                                <Show when=move || is_uploading.get()>
                                    <div class="flex items-center gap-2 text-sm" style="color: var(--text-secondary);">
                                        <LoadingSpinner size=SpinnerSize::Small variant=SpinnerVariant::Default/>
                                        <span>{move || format!("Processing {} of {}", upload_progress.get(), upload_total.get())}</span>
                                    </div>
                                </Show>
                                <div class="flex-1"></div>
                                <button
                                    class="px-5 sm:px-6 py-2.5 sm:py-3 rounded-lg transition-all font-medium flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed shrink-0"
                                    style="background-color: var(--accent-primary); color: white; min-width: 100px;"
                                    on:click=move |_| process_uploads()
                                    disabled=move || is_uploading.get() || (upload_type.get() == UploadType::Torrents && torrent_links.get().is_empty()) || (upload_type.get() == UploadType::NZB && nzb_links.get().is_empty()) || (upload_type.get() == UploadType::WebLinks && web_links.get().is_empty())
                                >
                                    <Show when=move || is_uploading.get()>
                                        <LoadingSpinner size=SpinnerSize::Small variant=SpinnerVariant::Default/>
                                    </Show>
                                    {move || if is_uploading.get() { "Uploading..." } else { "Upload" }.to_string()}
                                </button>
                            </div>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}
