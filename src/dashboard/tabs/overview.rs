use leptos::prelude::*;
use crate::dashboard::components::downloads_table::{DownloadsTable, DownloadItem};
use crate::dashboard::components::search::SearchComponent;
use crate::dashboard::components::network_activity_chart::NetworkActivityChart;
use crate::dashboard::components::upload::UploadComponent;

#[component]
pub fn OverviewTab() -> impl IntoView {
    let downloads = RwSignal::new(Vec::<DownloadItem>::new());
    provide_context(downloads.read_only());
    
    view! {
        <div class="flex flex-col w-full mt-10 sm:mt-12">
            <div class="mb-4 sm:mb-6" style="position: relative; z-index: 10; pointer-events: auto;">
                <SearchComponent/>
            </div>
            
            <div class="mb-4 sm:mb-6" style="position: relative; z-index: 8; pointer-events: auto;">
                <UploadComponent/>
            </div>
            
            <div class="mb-4 sm:mb-6" style="position: relative; z-index: 5;">
                <NetworkActivityChart/>
            </div>
            
            <div style="position: relative; z-index: 1;">
                <DownloadsTable downloads_signal=downloads/>
            </div>
        </div>
    }
}
