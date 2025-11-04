use leptos::prelude::*;
use crate::dashboard::components::downloads_table::DownloadItem;

fn format_speed(bytes_per_sec: i64) -> String {
    if bytes_per_sec < 0 {
        return "0 B/s".to_string();
    }
    if bytes_per_sec >= 1_099_511_627_776 {
        format!("{:.2} TB/s", bytes_per_sec as f64 / 1_099_511_627_776.0)
    } else if bytes_per_sec >= 1_073_741_824 {
        format!("{:.2} GB/s", bytes_per_sec as f64 / 1_073_741_824.0)
    } else if bytes_per_sec >= 1_048_576 {
        format!("{:.2} MB/s", bytes_per_sec as f64 / 1_048_576.0)
    } else if bytes_per_sec >= 1024 {
        format!("{:.2} KB/s", bytes_per_sec as f64 / 1024.0)
    } else {
        format!("{} B/s", bytes_per_sec)
    }
}

fn calculate_total_speeds(downloads: &[DownloadItem]) -> (i64, i64) {
    downloads
        .iter()
        .filter(|item| item.active)
        .fold((0i64, 0i64), |(dl, ul), item| {
            (dl + item.download_speed, ul + item.upload_speed)
        })
}

#[component]
pub fn NetworkActivityChart() -> impl IntoView {
    let downloads = use_context::<ReadSignal<Vec<DownloadItem>>>()
        .expect("Downloads signal should be provided by OverviewTab");

    let current_speeds = Memo::new(move |_| calculate_total_speeds(&downloads.get()));

    view! {
        <div class="mt-4 px-4 py-4 mb-4 rounded-xl border" 
             style="border-color: var(--border-secondary); background-color: var(--bg-card); box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);">
            <div class="flex items-center gap-4">
                <h3 class="text-base lg:text-lg font-semibold" style="color: var(--text-primary);">
                    "Network Activity"
                </h3>
                
                <div class="flex items-center gap-4">
                    <div class="flex items-center gap-2">
                        <div class="w-2.5 h-2.5 rounded-full" style="background-color: #34D399; box-shadow: 0 0 8px rgba(52, 211, 153, 0.4);"></div>
                        <span class="text-sm font-medium" style="color: #34D399;">
                            {move || format!("↓ {}", format_speed(current_speeds.get().0))}
                        </span>
                    </div>
                    <div class="flex items-center gap-2">
                        <div class="w-2.5 h-2.5 rounded-full" style="background-color: #F87171; box-shadow: 0 0 8px rgba(248, 113, 113, 0.4);"></div>
                        <span class="text-sm font-medium" style="color: #F87171;">
                            {move || format!("↑ {}", format_speed(current_speeds.get().1))}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}
