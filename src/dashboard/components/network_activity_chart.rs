use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::dashboard::components::downloads_table::DownloadItem;
use std::collections::VecDeque;
#[cfg(target_arch = "wasm32")]
use js_sys;

#[derive(Clone, Debug)]
struct DataPoint {
    timestamp: f64,
    download_speed: i64,
    upload_speed: i64,
}

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
    
    let data_points = RwSignal::new(VecDeque::<DataPoint>::new());
    let is_collapsed = RwSignal::new(true);
    let hovered_point = RwSignal::new(None::<(f64, f64, i64, i64)>);
    
    let has_activity = Memo::new(move |_| {
        let (dl, ul) = current_speeds.get();
        dl > 0 || ul > 0
    });
    
    Effect::new(move |_| {
        let activity = has_activity.get();
        if activity {
            is_collapsed.set(false);
        } else {
            is_collapsed.set(true);
        }
    });
    
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    
    const TEN_MINUTES_MS: f64 = 10.0 * 60.0 * 1000.0;
    const REDRAW_THROTTLE_MS: f64 = 100.0; 
    
    #[cfg(target_arch = "wasm32")]
    let last_redraw_time = RwSignal::new(0.0f64);
    
    Effect::new(move |_| {
        let (dl_speed, ul_speed) = current_speeds.get();
        
        #[cfg(target_arch = "wasm32")]
        let now = js_sys::Date::now();
        #[cfg(not(target_arch = "wasm32"))]
        let now = 0.0;
        
        if dl_speed > 0 || ul_speed > 0 {
            data_points.update(|points| {
                points.push_back(DataPoint {
                    timestamp: now,
                    download_speed: dl_speed,
                    upload_speed: ul_speed,
                });
                
                while let Some(front) = points.front() {
                    if now - front.timestamp > TEN_MINUTES_MS {
                        points.pop_front();
                    } else {
                        break;
                    }
                }
            });
            
            #[cfg(target_arch = "wasm32")]
            {
                let last_redraw = last_redraw_time.get();
                let time_since_last_redraw = now - last_redraw;
                
                if time_since_last_redraw >= REDRAW_THROTTLE_MS {
                    last_redraw_time.set(now);
                    use wasm_bindgen_futures::JsFuture;
                    use web_sys::js_sys::Promise;
                    
                    let canvas_ref_clone = canvas_ref.clone();
                    let data_points_clone = data_points.get();
                    let hovered_point_clone = hovered_point.clone();
                    spawn_local(async move {
                        let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                        let _ = JsFuture::from(promise).await;
                        draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_clone);
                    });
                }
            }
        } else {
            #[cfg(target_arch = "wasm32")]
            {
                // Redraw once when activity stops
                let canvas_ref_clone = canvas_ref.clone();
                let data_points_clone = data_points.get();
                let hovered_point_clone = hovered_point.clone();
                use wasm_bindgen_futures::JsFuture;
                use web_sys::js_sys::Promise;
                spawn_local(async move {
                    let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                    let _ = JsFuture::from(promise).await;
                    draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_clone);
                });
            }
        }
    });
    
    let toggle_collapse = move |_| {
        is_collapsed.update(|c| *c = !*c);
        #[cfg(target_arch = "wasm32")]
        {
                let canvas_ref_clone = canvas_ref.clone();
                let data_points_clone = data_points.get();
                let hovered_point_clone = hovered_point.clone();
                spawn_local(async move {
                    use wasm_bindgen_futures::JsFuture;
                    use web_sys::js_sys::Promise;
                    let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                    let _ = JsFuture::from(promise).await;
                    draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_clone);
                });
        }
    };
    
    #[cfg(target_arch = "wasm32")]
    {
        let canvas_ref_resize = canvas_ref.clone();
        let data_points_resize = data_points.clone();
        spawn_local(async move {
            use wasm_bindgen_futures::JsFuture;
            use web_sys::js_sys::Promise;
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;
            
            let closure = Closure::wrap(Box::new(move || {
                let canvas_ref_clone = canvas_ref_resize.clone();
                let data_points_clone = data_points_resize.get();
                let hovered_point_clone = hovered_point.clone();
                spawn_local(async move {
                    let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                    let _ = JsFuture::from(promise).await;
                    draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_clone);
                });
            }) as Box<dyn FnMut()>);
            
            if let Some(window) = web_sys::window() {
                window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).ok();
                closure.forget();
            }
        });
    }

    view! {
        <div class="mt-4 mb-4 rounded-xl border" 
             style="border-color: var(--border-secondary); background-color: var(--bg-card); box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);">
            <div class="px-4 py-4">
                <div class="flex items-center justify-between gap-4 cursor-pointer" on:click=toggle_collapse style="user-select: none;">
                    <div class="flex items-center gap-4 flex-1" style="pointer-events: none;">
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
                    <div class="text-lg" style="color: var(--text-secondary); pointer-events: none;">
                        {move || if is_collapsed.get() { "▼" } else { "▲" }}
                    </div>
                </div>
            </div>
            
            <Show when=move || !is_collapsed.get()>
                <div class="px-4 pb-4" style="position: relative;">
                    <canvas 
                        node_ref=canvas_ref
                        width="800"
                        height="300"
                        style="width: 100%; height: 300px; max-width: 100%; display: block; cursor: crosshair; touch-action: none;"
                        on:mousemove=move |ev| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(canvas) = canvas_ref.get() {
                                    let rect = canvas.get_bounding_client_rect();
                                    let x = ev.client_x() as f64 - rect.left();
                                    let y = ev.client_y() as f64 - rect.top();
                                    let data_points_clone = data_points.get();
                                    find_hovered_point(&canvas_ref, x, y, &data_points_clone, &hovered_point);
                                }
                            }
                        }
                        on:mouseleave=move |_| {
                            hovered_point.set(None);
                            let canvas_ref_clone = canvas_ref.clone();
                            let data_points_clone = data_points.get();
                            let hovered_point_leave = hovered_point.clone();
                            #[cfg(target_arch = "wasm32")]
                            {
                                use wasm_bindgen_futures::JsFuture;
                                use web_sys::js_sys::Promise;
                                spawn_local(async move {
                                    let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                                    let _ = JsFuture::from(promise).await;
                                    draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_leave);
                                });
                            }
                        }
                        on:touchmove=move |ev| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                ev.prevent_default();
                                if let Some(canvas) = canvas_ref.get() {
                                    if let Some(touch) = ev.changed_touches().get(0) {
                                        let rect = canvas.get_bounding_client_rect();
                                        let x = touch.client_x() as f64 - rect.left();
                                        let y = touch.client_y() as f64 - rect.top();
                                        let data_points_clone = data_points.get();
                                        find_hovered_point(&canvas_ref, x, y, &data_points_clone, &hovered_point);
                                    }
                                }
                            }
                        }
                        on:touchstart=move |ev| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                ev.prevent_default();
                                if let Some(canvas) = canvas_ref.get() {
                                    if let Some(touch) = ev.changed_touches().get(0) {
                                        let rect = canvas.get_bounding_client_rect();
                                        let x = touch.client_x() as f64 - rect.left();
                                        let y = touch.client_y() as f64 - rect.top();
                                        let data_points_clone = data_points.get();
                                        find_hovered_point(&canvas_ref, x, y, &data_points_clone, &hovered_point);
                                    }
                                }
                            }
                        }
                        on:touchend=move |_| {
                            hovered_point.set(None);
                            let canvas_ref_clone = canvas_ref.clone();
                            let data_points_clone = data_points.get();
                            let hovered_point_leave = hovered_point.clone();
                            #[cfg(target_arch = "wasm32")]
                            {
                                use wasm_bindgen_futures::JsFuture;
                                use web_sys::js_sys::Promise;
                                spawn_local(async move {
                                    let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                                    let _ = JsFuture::from(promise).await;
                                    draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_leave);
                                });
                            }
                        }
                    ></canvas>
                    {move || {
                        hovered_point.get_untracked().map(|(x, y, dl, ul)| {
                            view! {
                                <div 
                                    class="absolute pointer-events-none rounded px-2 py-1 text-xs"
                                    style=format!(
                                        "left: {}px; top: {}px; background-color: var(--bg-card); border: 1px solid var(--border-secondary); color: var(--text-primary); transform: translate(-50%, -100%); margin-top: -8px; z-index: 10;",
                                        x, y
                                    )
                                >
                                    <div style="color: #34D399;">"↓ " {format_speed(dl)}</div>
                                    <div style="color: #F87171;">"↑ " {format_speed(ul)}</div>
                                </div>
                            }
                        })
                    }}
                </div>
            </Show>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn get_css_variable(name: &str) -> String {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|doc| {
            doc.document_element()
                .and_then(|elem| {
                    web_sys::window()
                        .and_then(|w| {
                            w.get_computed_style(&elem)
                                .ok()
                                .and_then(|result| result)
                        })
                        .and_then(|computed| computed.get_property_value(name).ok())
                })
        })
        .unwrap_or_else(|| {
            match name {
                "--bg-card" => "#1a1a1a".to_string(),
                "--text-primary" => "#ffffff".to_string(),
                "--text-secondary" => "#a3a3a3".to_string(),
                "--border-secondary" => "rgba(255, 255, 255, 0.05)".to_string(),
                _ => "#000000".to_string(),
            }
        })
}

fn round_to_nice_number(value: f64) -> f64 {
    if value <= 0.0 {
        return 1.0;
    }
    let magnitude = 10_f64.powf(value.log10().floor());
    let normalized = value / magnitude;
    let nice = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * magnitude
}

fn format_timestamp(timestamp: f64) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp));
        let hours = date.get_hours();
        let minutes = date.get_minutes();
        let seconds = date.get_seconds();
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "00:00:00".to_string()
    }
}

#[cfg(target_arch = "wasm32")]
fn draw_chart(canvas_ref: &NodeRef<leptos::html::Canvas>, data_points: &VecDeque<DataPoint>, hovered_point: &RwSignal<Option<(f64, f64, i64, i64)>>) {
    use wasm_bindgen::JsCast;
    use web_sys::CanvasRenderingContext2d;
    
    if let Some(canvas) = canvas_ref.get_untracked() {
        if let Ok(Some(ctx)) = canvas.get_context("2d") {
            if let Some(ctx) = ctx.dyn_ref::<CanvasRenderingContext2d>() {
                let rect = canvas.get_bounding_client_rect();
                let dpr = web_sys::window()
                    .map(|w| w.device_pixel_ratio())
                    .unwrap_or(1.0);
                
                let display_width = rect.width() as u32;
                let display_height = 300u32;
                let canvas_width = (display_width as f64 * dpr) as u32;
                let canvas_height = (display_height as f64 * dpr) as u32;
                
                if canvas.width() != canvas_width || canvas.height() != canvas_height {
                    canvas.set_width(canvas_width);
                    canvas.set_height(canvas_height);
                    if let Some(html_element) = canvas.dyn_ref::<web_sys::HtmlElement>() {
                        let style = html_element.style();
                        style.set_property("width", &format!("{}px", display_width)).ok();
                        style.set_property("height", &format!("{}px", display_height)).ok();
                    }
                }
                
                let width = display_width as f64;
                let height = display_height as f64;
                
                ctx.save();
                ctx.scale(dpr, dpr).ok();
                
                ctx.clear_rect(0.0, 0.0, width, height);
                
                let bg_color = get_css_variable("--bg-card");
                let _text_primary = get_css_variable("--text-primary");
                let text_secondary = get_css_variable("--text-secondary");
                let border_color = get_css_variable("--border-secondary");
                
                ctx.set_fill_style_str(&bg_color);
                ctx.fill_rect(0.0, 0.0, width, height);
                
                if data_points.is_empty() {
                    ctx.set_fill_style_str(&text_secondary);
                    ctx.set_font("14px system-ui");
                    ctx.set_text_align("center");
                    ctx.fill_text("No data available", width / 2.0, height / 2.0)
                        .ok();
                    ctx.restore();
                    return;
                }
                
                let has_network_activity = data_points.iter()
                    .any(|p| p.download_speed > 0 || p.upload_speed > 0);
                
                if !has_network_activity {
                    ctx.set_fill_style_str(&text_secondary);
                    ctx.set_font("14px system-ui");
                    ctx.set_text_align("center");
                    ctx.fill_text("No network activity", width / 2.0, height / 2.0)
                        .ok();
                    ctx.restore();
                    return;
                }
                
                let max_dl = data_points.iter()
                    .map(|p| p.download_speed)
                    .max()
                    .unwrap_or(0) as f64;
                let max_ul = data_points.iter()
                    .map(|p| p.upload_speed)
                    .max()
                    .unwrap_or(0) as f64;
                let max_speed_raw = max_dl.max(max_ul);
                let max_speed = if max_speed_raw > 0.0 {
                    round_to_nice_number(max_speed_raw * 1.15)
                } else {
                    1024.0
                };
                
                let y_ticks = 5;
                let max_label = format_speed(max_speed as i64);
                let estimated_char_width = 7.0;
                let max_label_width = max_label.len() as f64 * estimated_char_width;
                let left_padding = (max_label_width + 20.0).max(70.0); 
                let right_padding = 40.0;
                let top_padding = 20.0;
                let bottom_padding = 40.0;
                
                let chart_width = (width - left_padding - right_padding).max(0.0);
                let chart_height = (height - top_padding - bottom_padding).max(0.0);
                
                let now = js_sys::Date::now();
                let oldest_time = data_points.front().map(|p| p.timestamp).unwrap_or(now - (10.0 * 60.0 * 1000.0));
                let newest_time = data_points.back().map(|p| p.timestamp).unwrap_or(now);
                let actual_time_range = (newest_time - oldest_time).max(1000.0);
                
                ctx.set_stroke_style_str(&border_color);
                ctx.set_line_width(1.0);
                
                ctx.begin_path();
                ctx.move_to(left_padding, top_padding);
                ctx.line_to(left_padding, height - bottom_padding);
                ctx.line_to(width - right_padding, height - bottom_padding);
                ctx.stroke();
                
                ctx.set_fill_style_str(&text_secondary);
                ctx.set_font("11px system-ui");
                ctx.set_text_align("right");
                ctx.set_text_baseline("middle");
                
                for i in 0..=y_ticks {
                    let y = top_padding + (chart_height / y_ticks as f64) * (y_ticks - i) as f64;
                    let value = (max_speed / y_ticks as f64) * i as f64;
                    let label = format_speed(value as i64);
                    
                    ctx.fill_text(&label, left_padding - 10.0, y).ok();
                    
                    ctx.set_stroke_style_str(&border_color);
                    ctx.set_line_width(0.5);
                    ctx.begin_path();
                    ctx.move_to(left_padding, y);
                    ctx.line_to(width - right_padding, y);
                    ctx.stroke();
                }
                
                if data_points.len() > 0 {
                    ctx.set_text_align("center");
                    ctx.set_text_baseline("top");
                    ctx.set_fill_style_str(&text_secondary);
                    ctx.set_font("10px system-ui");
                    
                    let max_labels = 8;
                    let step = if data_points.len() <= max_labels {
                        1
                    } else {
                        (data_points.len() / max_labels).max(1)
                    };
                    
                    for (idx, point) in data_points.iter().enumerate() {
                        if idx % step == 0 || idx == data_points.len() - 1 {
                            let x = if actual_time_range > 0.0 {
                                left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
                            } else {
                                left_padding
                            };
                            let label = format_timestamp(point.timestamp);
                            ctx.fill_text(&label, x, height - bottom_padding + 8.0).ok();
                        }
                    }
                }
                
                if data_points.len() > 1 {
                    let download_color = "#34D399";
                    let upload_color = "#F87171";
                    let download_fill = "rgba(52, 211, 153, 0.2)";
                    let upload_fill = "rgba(248, 113, 113, 0.2)";
                    
                    let bottom_y = height - bottom_padding;
                    
                    ctx.set_line_width(2.0);
                    ctx.set_line_cap("round");
                    ctx.set_line_join("round");
                    
                    ctx.begin_path();
                    ctx.set_fill_style_str(download_fill);
                    let mut first = true;
                    for point in data_points.iter() {
                        let x = if actual_time_range > 0.0 {
                            left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        let y = height - bottom_padding - ((point.download_speed as f64 / max_speed) * chart_height);
                        
                        if first {
                            ctx.move_to(x, bottom_y);
                            first = false;
                        }
                        ctx.line_to(x, y);
                    }
                    if let Some(last_point) = data_points.back() {
                        let last_x = if actual_time_range > 0.0 {
                            left_padding + ((last_point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        ctx.line_to(last_x, bottom_y);
                        ctx.close_path();
                    }
                    ctx.fill();
                    
                    ctx.begin_path();
                    ctx.set_stroke_style_str(download_color);
                    let mut first = true;
                    for point in data_points.iter() {
                        let x = if actual_time_range > 0.0 {
                            left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        let y = height - bottom_padding - ((point.download_speed as f64 / max_speed) * chart_height);
                        
                        if first {
                            ctx.move_to(x, y);
                            first = false;
                        } else {
                            ctx.line_to(x, y);
                        }
                    }
                    ctx.stroke();
                    
                    ctx.begin_path();
                    ctx.set_fill_style_str(upload_fill);
                    let mut first = true;
                    for point in data_points.iter() {
                        let x = if actual_time_range > 0.0 {
                            left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        let y = height - bottom_padding - ((point.upload_speed as f64 / max_speed) * chart_height);
                        
                        if first {
                            ctx.move_to(x, bottom_y);
                            first = false;
                        }
                        ctx.line_to(x, y);
                    }
                    if let Some(last_point) = data_points.back() {
                        let last_x = if actual_time_range > 0.0 {
                            left_padding + ((last_point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        ctx.line_to(last_x, bottom_y);
                        ctx.close_path();
                    }
                    ctx.fill();
                    
                    ctx.begin_path();
                    ctx.set_stroke_style_str(upload_color);
                    let mut first = true;
                    for point in data_points.iter() {
                        let x = if actual_time_range > 0.0 {
                            left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        let y = height - bottom_padding - ((point.upload_speed as f64 / max_speed) * chart_height);
                        
                        if first {
                            ctx.move_to(x, y);
                            first = false;
                        } else {
                            ctx.line_to(x, y);
                        }
                    }
                    ctx.stroke();
                    
                    ctx.set_line_width(1.0);
                    
                    let hovered = hovered_point.get_untracked();
                    let hovered_x = hovered.map(|(x, _, _, _)| x);
                    
                    ctx.set_fill_style_str(download_color);
                    for point in data_points.iter() {
                        let x = if actual_time_range > 0.0 {
                            left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        let y = height - bottom_padding - ((point.download_speed as f64 / max_speed) * chart_height);
                        
                        let dot_size = if hovered_x.map(|hx| (hx - x).abs() < 5.0).unwrap_or(false) {
                            5.0
                        } else {
                            3.5
                        };
                        
                        ctx.begin_path();
                        ctx.arc(x, y, dot_size, 0.0, 2.0 * std::f64::consts::PI).ok();
                        ctx.fill();
                    }
                    
                    ctx.set_fill_style_str(upload_color);
                    for point in data_points.iter() {
                        let x = if actual_time_range > 0.0 {
                            left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
                        } else {
                            left_padding
                        };
                        let y = height - bottom_padding - ((point.upload_speed as f64 / max_speed) * chart_height);
                        
                        let dot_size = if hovered_x.map(|hx| (hx - x).abs() < 5.0).unwrap_or(false) {
                            5.0
                        } else {
                            3.5
                        };
                        
                        ctx.begin_path();
                        ctx.arc(x, y, dot_size, 0.0, 2.0 * std::f64::consts::PI).ok();
                        ctx.fill();
                    }
                }
                
                ctx.restore();
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn find_hovered_point(
    canvas_ref: &NodeRef<leptos::html::Canvas>,
    mouse_x: f64,
    mouse_y: f64,
    data_points: &VecDeque<DataPoint>,
    hovered_point: &RwSignal<Option<(f64, f64, i64, i64)>>,
) {
    use wasm_bindgen::JsCast;
    
    if let Some(canvas) = canvas_ref.get_untracked() {
        let rect = canvas.get_bounding_client_rect();
        let width = rect.width();
        let height = rect.height();
        
        let now = js_sys::Date::now();
        let oldest_time = data_points.front().map(|p| p.timestamp).unwrap_or(now - (10.0 * 60.0 * 1000.0));
        let newest_time = data_points.back().map(|p| p.timestamp).unwrap_or(now);
        let actual_time_range = (newest_time - oldest_time).max(1000.0);
        
        let max_dl = data_points.iter().map(|p| p.download_speed).max().unwrap_or(0) as f64;
        let max_ul = data_points.iter().map(|p| p.upload_speed).max().unwrap_or(0) as f64;
        let max_speed_raw = max_dl.max(max_ul);
        let max_speed = if max_speed_raw > 0.0 {
            round_to_nice_number(max_speed_raw * 1.15)
        } else {
            1024.0
        };
        
        let max_label = format_speed(max_speed as i64);
        let estimated_char_width = 7.0;
        let max_label_width = max_label.len() as f64 * estimated_char_width;
        let left_padding = (max_label_width + 20.0).max(70.0);
        let right_padding = 40.0;
        let top_padding = 20.0;
        let bottom_padding = 40.0;
        
        let chart_width = (width - left_padding - right_padding).max(0.0f64);
        let chart_height = (height - top_padding - bottom_padding).max(0.0f64);
        
        let hover_threshold = 8.0;
        
        for point in data_points.iter() {
            let x = if actual_time_range > 0.0 {
                left_padding + ((point.timestamp - oldest_time) / actual_time_range) * chart_width
            } else {
                left_padding
            };
            
            let dl_y = height - bottom_padding - ((point.download_speed as f64 / max_speed) * chart_height);
            let ul_y = height - bottom_padding - ((point.upload_speed as f64 / max_speed) * chart_height);
            
            let dist_dl = ((mouse_x - x).powi(2) + (mouse_y - dl_y).powi(2)).sqrt();
            let dist_ul = ((mouse_x - x).powi(2) + (mouse_y - ul_y).powi(2)).sqrt();
            
            if dist_dl < hover_threshold || dist_ul < hover_threshold {
                hovered_point.set(Some((mouse_x, mouse_y, point.download_speed, point.upload_speed)));
                
                let canvas_ref_clone = canvas_ref.clone();
                let data_points_clone = data_points.clone();
                let hovered_point_clone = hovered_point.clone();
                spawn_local(async move {
                    use wasm_bindgen_futures::JsFuture;
                    use web_sys::js_sys::Promise;
                    let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
                    let _ = JsFuture::from(promise).await;
                    draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_clone);
                });
                return;
            }
        }
        
        hovered_point.set(None);
        
        let canvas_ref_clone = canvas_ref.clone();
        let data_points_clone = data_points.clone();
        let hovered_point_clone = hovered_point.clone();
        spawn_local(async move {
            use wasm_bindgen_futures::JsFuture;
            use web_sys::js_sys::Promise;
            let promise = Promise::resolve(&wasm_bindgen::JsValue::UNDEFINED);
            let _ = JsFuture::from(promise).await;
            draw_chart(&canvas_ref_clone, &data_points_clone, &hovered_point_clone);
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn find_hovered_point(
    _canvas_ref: &NodeRef<leptos::html::Canvas>,
    _mouse_x: f64,
    _mouse_y: f64,
    _data_points: &VecDeque<DataPoint>,
    _hovered_point: &RwSignal<Option<(f64, f64, i64, i64)>>,
) {
}

#[cfg(not(target_arch = "wasm32"))]
fn get_css_variable(_name: &str) -> String {
    "#1a1a1a".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn draw_chart(_canvas_ref: &NodeRef<leptos::html::Canvas>, _data_points: &VecDeque<DataPoint>, _hovered_point: &RwSignal<Option<(f64, f64, i64, i64)>>) {
}
