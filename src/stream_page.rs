use leptos::prelude::*;
use leptos_meta::{Style, Title};

#[component]
pub fn StreamPage() -> impl IntoView {
    view! {
        <Title text="TorBox Stream Player"/>
        <Style>
            {r#"
            :root {
                --bg-primary: #0a0a0a;
                --bg-secondary: #111111;
                --bg-tertiary: #1a1a1a;
                --bg-card: rgba(17, 17, 17, 0.98);
                --text-primary: #ffffff;
                --text-secondary: #aaaaaa;
                --text-muted: #717171;
                --border-primary: rgba(255, 255, 255, 0.1);
                --border-secondary: rgba(255, 255, 255, 0.05);
                --accent-primary: #ff0000;
                --accent-hover: #cc0000;
                --success: #10b981;
                --error: #ef4444;
                --transition-base: 200ms cubic-bezier(0.4, 0, 0.2, 1);
            }

            .stream-page {
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                z-index: 9999;
                background: var(--bg-primary);
                color: var(--text-primary);
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
                -webkit-font-smoothing: antialiased;
                -moz-osx-font-smoothing: grayscale;
                overflow: hidden;
            }

            .player-container {
                flex: 1;
                display: flex;
                flex-direction: column;
                width: 100%;
                height: 100%;
                max-width: 100%;
                max-height: 100%;
                background: #000;
                position: relative;
            }

            .player-header {
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                z-index: 20;
                padding: 12px 16px;
                background: linear-gradient(to bottom, rgba(0, 0, 0, 0.7) 0%, transparent 100%);
                display: flex;
                justify-content: space-between;
                align-items: center;
                pointer-events: none;
            }

            .player-header > * {
                pointer-events: all;
            }

            .player-header h1 {
                font-size: 14px;
                font-weight: 500;
                color: var(--text-primary);
            }

            .close-btn {
                background: rgba(0, 0, 0, 0.5);
                border: none;
                color: var(--text-primary);
                padding: 8px;
                border-radius: 50%;
                cursor: pointer;
                width: 40px;
                height: 40px;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: all var(--transition-base);
            }

            .close-btn:hover {
                background: rgba(0, 0, 0, 0.7);
            }

            .close-btn svg {
                width: 24px;
                height: 24px;
                stroke: currentColor;
            }

            .video-wrapper {
                position: relative;
                width: 100%;
                height: 100%;
                flex: 1;
                display: block;
                background: #000;
                overflow: hidden;
                padding: 8px;
                box-sizing: border-box;
            }

            video {
                width: calc(100% - 16px);
                height: calc(100% - 16px);
                object-fit: contain;
                display: block;
                outline: none;
                position: absolute;
                top: 8px;
                left: 8px;
            }

            .controls-overlay {
                position: absolute;
                bottom: 0;
                left: 0;
                right: 0;
                background: linear-gradient(to top, rgba(0, 0, 0, 0.75) 0%, rgba(0, 0, 0, 0) 100%);
                padding: 0;
                opacity: 0;
                transition: opacity 200ms ease;
                pointer-events: none;
                z-index: 10;
            }

            .player-container:hover .controls-overlay,
            .player-container.controls-visible .controls-overlay {
                opacity: 1;
                pointer-events: all;
            }

            .progress-wrapper {
                padding: 0 16px 8px;
                cursor: pointer;
            }

            .progress-container {
                position: relative;
                height: 5px;
                background: rgba(255, 255, 255, 0.2);
                border-radius: 2px;
                cursor: pointer;
                transition: height 100ms;
            }

            .progress-container:hover {
                height: 6px;
            }

            .progress-buffer {
                position: absolute;
                top: 0;
                left: 0;
                height: 100%;
                background: rgba(255, 255, 255, 0.3);
                border-radius: 2px;
                width: 0%;
            }

            .progress-bar {
                position: absolute;
                top: 0;
                left: 0;
                height: 100%;
                background: var(--accent-primary);
                border-radius: 2px;
                width: 0%;
                transition: width 0.1s linear;
            }

            .progress-hover {
                position: absolute;
                top: 0;
                left: 0;
                height: 100%;
                width: 0%;
                background: rgba(255, 255, 255, 0.5);
                border-radius: 2px;
                opacity: 0;
                transition: opacity 100ms;
            }

            .progress-container:hover .progress-hover {
                opacity: 1;
            }

            .controls-bar {
                display: flex;
                align-items: center;
                padding: 8px 12px 12px;
                gap: 8px;
            }

            .controls-left {
                display: flex;
                align-items: center;
                gap: 8px;
                flex: 0 0 auto;
            }

            .controls-center {
                flex: 1;
                display: flex;
                align-items: center;
                gap: 8px;
                justify-content: center;
                min-width: 0;
            }

            .controls-right {
                display: flex;
                align-items: center;
                gap: 8px;
                flex: 0 0 auto;
            }

            .control-btn {
                background: transparent;
                border: none;
                color: var(--text-primary);
                cursor: pointer;
                padding: 8px;
                border-radius: 50%;
                transition: background var(--transition-base);
                display: flex;
                align-items: center;
                justify-content: center;
                width: 48px;
                height: 48px;
                position: relative;
            }

            .control-btn:hover {
                background: rgba(255, 255, 255, 0.1);
            }

            .control-btn:active {
                background: rgba(255, 255, 255, 0.15);
            }

            .control-btn svg {
                width: 24px;
                height: 24px;
                stroke: currentColor;
                stroke-width: 2;
                fill: none;
                display: block;
            }

            .control-btn.play-pause svg {
                width: 28px;
                height: 28px;
            }

            .time-display {
                font-size: 14px;
                color: var(--text-primary);
                font-variant-numeric: tabular-nums;
                white-space: nowrap;
                padding: 0 4px;
                user-select: none;
            }

            .volume-container {
                display: flex;
                align-items: center;
                gap: 8px;
                position: relative;
            }

            .volume-slider-wrapper {
                width: 0;
                overflow: hidden;
                transition: width 150ms;
                display: flex;
                align-items: center;
            }

            .volume-container:hover .volume-slider-wrapper,
            .volume-container.show-slider .volume-slider-wrapper {
                width: 80px;
            }

            .volume-slider {
                width: 80px;
                height: 4px;
                background: rgba(255, 255, 255, 0.2);
                border-radius: 2px;
                cursor: pointer;
                position: relative;
                flex-shrink: 0;
            }

            .volume-slider-fill {
                height: 100%;
                background: var(--text-primary);
                border-radius: 2px;
                width: 100%;
                transition: width 0.1s;
            }

            .volume-percentage {
                position: absolute;
                bottom: 100%;
                left: 50%;
                transform: translateX(-50%);
                margin-bottom: 8px;
                background: rgba(0, 0, 0, 0.8);
                color: var(--text-primary);
                padding: 6px 12px;
                border-radius: 4px;
                font-size: 13px;
                font-variant-numeric: tabular-nums;
                white-space: nowrap;
                pointer-events: none;
                opacity: 0;
                transition: opacity 200ms;
                z-index: 30;
            }

            .volume-container.show-percentage .volume-percentage {
                opacity: 1;
            }

            .settings-menu {
                position: relative;
            }

            .settings-dropdown {
                position: absolute;
                bottom: 100%;
                right: 0;
                margin-bottom: 8px;
                background: rgba(28, 28, 28, 0.95);
                border: 1px solid rgba(255, 255, 255, 0.1);
                border-radius: 8px;
                min-width: 240px;
                padding: 8px 0;
                display: none;
                box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
                backdrop-filter: blur(12px);
            }

            .settings-menu.active .settings-dropdown {
                display: block;
            }

            .settings-item {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 12px 16px;
                color: var(--text-primary);
                cursor: pointer;
                font-size: 14px;
                transition: background var(--transition-base);
            }

            .settings-item:hover {
                background: rgba(255, 255, 255, 0.05);
            }

            .settings-item select {
                background: transparent;
                border: none;
                color: var(--text-primary);
                font-size: 14px;
                cursor: pointer;
                outline: none;
                padding: 4px 8px;
                border-radius: 4px;
            }

            .settings-item select:hover {
                background: rgba(255, 255, 255, 0.1);
            }

            .loading, .error {
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                text-align: center;
                color: var(--text-secondary);
                font-size: 15px;
                z-index: 5;
            }

            .error {
                color: var(--error);
            }

            .loading-overlay {
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                background: rgba(0, 0, 0, 0.7);
                padding: 20px 40px;
                border-radius: 8px;
                color: var(--text-primary);
                font-size: 14px;
                z-index: 15;
                backdrop-filter: blur(8px);
                display: none;
            }

            .loading-overlay.active {
                display: block;
            }

            @media (max-width: 768px) {
                .controls-row {
                    flex-direction: column;
                    gap: 16px;
                }

                .control-group {
                    min-width: 100%;
                }

                .volume-slider-wrapper {
                    display: none;
                }

                .time-display {
                    font-size: 12px;
                    min-width: 70px;
                }

                .control-btn {
                    width: 40px;
                    height: 40px;
                    padding: 6px;
                }

                .control-btn svg {
                    width: 20px;
                    height: 20px;
                }
            }

            video::-webkit-media-controls {
                display: none !important;
            }

            video::-webkit-media-controls-enclosure {
                display: none !important;
            }
            "#}
        </Style>
        <div class="stream-page">
            <div class="player-container" id="player-container">
                <div class="player-header">
                        <h1>"TorBox Stream Player"</h1>
                        <button class="close-btn" onclick="window.close()" title="Close (Esc)">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                    
                    <div class="video-wrapper" id="video-wrapper">
                        <div id="player-wrapper">
                            <div class="loading">"Loading stream..."</div>
                        </div>
                        
                        <div class="loading-overlay" id="loading-overlay">"Switching track..."</div>
                        
                        <div class="controls-overlay">
                            <div class="progress-wrapper">
                                <div class="progress-container" id="progress-container">
                                    <div class="progress-buffer" id="progress-buffer"></div>
                                    <div class="progress-hover" id="progress-hover"></div>
                                    <div class="progress-bar" id="progress-bar"></div>
                                </div>
                            </div>
                            
                            <div class="controls-bar">
                                <div class="controls-left">
                                    <button class="control-btn play-pause" id="play-pause-btn" title="Play/Pause (Space)">
                                        <svg id="play-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <svg id="pause-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" style="display: none;">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                    </button>
                                    
                                    <div class="volume-container" id="volume-container">
                                        <button class="control-btn" id="mute-btn" title="Mute (M)">
                                            <svg id="volume-high-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
                                            </svg>
                                            <svg id="volume-low-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" style="display: none;">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2" />
                                            </svg>
                                            <svg id="volume-muted-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" style="display: none;">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2M17 10l2-2m0 0l2-2" />
                                            </svg>
                                        </button>
                                        <div class="volume-percentage" id="volume-percentage">"100%"</div>
                                        <div class="volume-slider-wrapper">
                                            <div class="volume-slider" id="volume-slider">
                                                <div class="volume-slider-fill" id="volume-fill"></div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="controls-center">
                                    <div class="time-display" id="time-display">"0:00 / 0:00"</div>
                                </div>
                                
                                <div class="controls-right">
                                    <div class="settings-menu" id="settings-menu">
                                        <button class="control-btn" id="settings-btn" title="Settings">
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                            </svg>
                                        </button>
                                        <div class="settings-dropdown">
                                            <div class="settings-item">
                                                <span>"Audio Track"</span>
                                                <select id="audio-track-settings">
                                                    <option value="">"Loading..."</option>
                                                </select>
                                            </div>
                                            <div class="settings-item">
                                                <span>"Subtitles"</span>
                                                <select id="subtitle-track-settings">
                                                    <option value="">"None"</option>
                                                </select>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <button class="control-btn" id="fullscreen-btn" title="Fullscreen (F)">
                                        <svg id="fullscreen-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" />
                                        </svg>
                                        <svg id="fullscreen-exit-icon" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" style="display: none;">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

            <script src="https://cdn.jsdelivr.net/npm/hls.js@latest"></script>
            <script>
                {r#"
                    const urlParams = new URLSearchParams(window.location.search);
                    const streamUrl = urlParams.get('url');
                    const presignedToken = urlParams.get('presigned_token');
                    const userToken = urlParams.get('user_token');
                    const metadataJson = urlParams.get('metadata');
                    
                    let metadata = null;
                    let hlsInstance = null;
                    let currentVideo = null;
                    let currentAudioIndex = null;
                    let currentSubtitleIndex = null;
                    let controlsTimeout = null;
                    let isDragging = false;
                    let hoverTimeout = null;

                    if (metadataJson) {
                        try {
                            metadata = JSON.parse(decodeURIComponent(metadataJson));
                        } catch (e) {
                            console.error('Failed to parse metadata:', e);
                        }
                    }

                    function formatTime(seconds) {
                        if (isNaN(seconds)) return '0:00';
                        const hrs = Math.floor(seconds / 3600);
                        const mins = Math.floor((seconds % 3600) / 60);
                        const secs = Math.floor(seconds % 60);
                        if (hrs > 0) {
                            return `${hrs}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
                        }
                        return `${mins}:${secs.toString().padStart(2, '0')}`;
                    }

                    function updateProgress() {
                        if (!currentVideo) return;
                        const progress = (currentVideo.currentTime / currentVideo.duration) * 100;
                        document.getElementById('progress-bar').style.width = progress + '%';
                        
                        if (currentVideo.buffered.length > 0) {
                            const buffered = (currentVideo.buffered.end(0) / currentVideo.duration) * 100;
                            document.getElementById('progress-buffer').style.width = buffered + '%';
                        }
                        
                        document.getElementById('time-display').textContent = 
                            `${formatTime(currentVideo.currentTime)} / ${formatTime(currentVideo.duration)}`;
                    }

                    function updatePlayPauseIcon() {
                        const playIcon = document.getElementById('play-icon');
                        const pauseIcon = document.getElementById('pause-icon');
                        if (currentVideo && !currentVideo.paused) {
                            playIcon.style.display = 'none';
                            pauseIcon.style.display = 'block';
                        } else {
                            playIcon.style.display = 'block';
                            pauseIcon.style.display = 'none';
                        }
                    }

                    function updateVolumeIcon() {
                        const highIcon = document.getElementById('volume-high-icon');
                        const lowIcon = document.getElementById('volume-low-icon');
                        const mutedIcon = document.getElementById('volume-muted-icon');
                        const volumeFill = document.getElementById('volume-fill');
                        
                        if (!currentVideo) return;
                        
                        volumeFill.style.width = (currentVideo.volume * 100) + '%';
                        
                        if (currentVideo.muted || currentVideo.volume === 0) {
                            highIcon.style.display = 'none';
                            lowIcon.style.display = 'none';
                            mutedIcon.style.display = 'block';
                        } else if (currentVideo.volume < 0.5) {
                            highIcon.style.display = 'none';
                            lowIcon.style.display = 'block';
                            mutedIcon.style.display = 'none';
                        } else {
                            highIcon.style.display = 'block';
                            lowIcon.style.display = 'none';
                            mutedIcon.style.display = 'none';
                        }
                    }

                    function showControls() {
                        const container = document.getElementById('player-container');
                        container.classList.add('controls-visible');
                        clearTimeout(controlsTimeout);
                        controlsTimeout = setTimeout(() => {
                            if (!isDragging && currentVideo && !currentVideo.paused) {
                                container.classList.remove('controls-visible');
                            }
                        }, 3000);
                    }

                    function hideControls() {
                        if (!isDragging && currentVideo && !currentVideo.paused) {
                            clearTimeout(hoverTimeout);
                            hoverTimeout = setTimeout(() => {
                                document.getElementById('player-container').classList.remove('controls-visible');
                            }, 2000);
                        }
                    }

                    function updateStreamUrl(newUrl) {
                        if (!currentVideo || !hlsInstance) return;
                        
                        document.getElementById('loading-overlay').classList.add('active');
                        
                        hlsInstance.destroy();
                        hlsInstance = new Hls({
                            enableWorker: true,
                            lowLatencyMode: false,
                        });
                        hlsInstance.loadSource(newUrl);
                        hlsInstance.attachMedia(currentVideo);
                        
                        hlsInstance.on(Hls.Events.MANIFEST_PARSED, () => {
                            document.getElementById('loading-overlay').classList.remove('active');
                            enableSubtitlesFromStream();
                            currentVideo.play().catch(e => console.error('Play error:', e));
                        });
                        
                        hlsInstance.on(Hls.Events.ERROR, (event, data) => {
                            if (data.fatal) {
                                document.getElementById('loading-overlay').classList.remove('active');
                                alert('Failed to switch track. Please try again.');
                            }
                        });
                    }

                    function populateAudioTracks() {
                        const audioSelectSettings = document.getElementById('audio-track-settings');
                        
                        if (!audioSelectSettings) return;
                        
                        audioSelectSettings.innerHTML = '';
                        if (metadata && metadata.audios) {
                            metadata.audios.forEach((audio, idx) => {
                                const option = document.createElement('option');
                                option.value = idx;
                                option.textContent = `${audio.language_full || audio.language}${audio.title ? ' - ' + audio.title : ''} (${audio.channel_layout || audio.channels + 'ch'})`;
                                if (audio.default) {
                                    option.selected = true;
                                    currentAudioIndex = idx;
                                }
                                audioSelectSettings.appendChild(option);
                            });
                        }
                        
                        audioSelectSettings.addEventListener('change', async (e) => {
                            const newAudioIndex = parseInt(e.target.value);
                            if (newAudioIndex === currentAudioIndex || !presignedToken || !userToken) return;
                            
                            currentAudioIndex = newAudioIndex;
                            
                            const url = `https://api.torbox.app/v1/api/stream/getstreamdata?token=${encodeURIComponent(userToken)}&presigned_token=${encodeURIComponent(presignedToken)}&chosen_audio_index=${newAudioIndex}${currentSubtitleIndex !== null ? '&chosen_subtitle_index=' + currentSubtitleIndex : ''}`;
                            
                            try {
                                const response = await fetch(url, {
                                    headers: { 'Authorization': `Bearer ${userToken}` }
                                });
                                const data = await response.json();
                                if (data.success && data.data && data.data.hls_url) {
                                    updateStreamUrl(data.data.hls_url);
                                }
                            } catch (error) {
                                console.error('Failed to switch audio track:', error);
                                alert('Failed to switch audio track');
                            }
                        });
                    }

                    function enableSubtitlesFromStream() {
                        if (!currentVideo) return;
                        
                        if (currentVideo.textTracks && currentVideo.textTracks.length > 0) {
                            for (let i = 0; i < currentVideo.textTracks.length; i++) {
                                const track = currentVideo.textTracks[i];
                                if (currentSubtitleIndex !== null) {
                                    if (track.mode === 'disabled' || track.mode === 'hidden') {
                                        track.mode = 'showing';
                                    }
                                } else {
                                    track.mode = 'hidden';
                                }
                            }
                        }
                    }

                    function loadSubtitleFromUrl(subtitleUrl, language, label) {
                        if (!currentVideo) return;
                        
                        const existingTracks = Array.from(currentVideo.textTracks);
                        existingTracks.forEach(track => {
                            if (track.label === label) {
                                track.mode = 'hidden';
                            }
                        });
                        
                        const track = document.createElement('track');
                        track.kind = 'subtitles';
                        track.src = subtitleUrl;
                        track.srclang = language;
                        track.label = label;
                        track.default = false;
                        
                        currentVideo.appendChild(track);
                        
                        track.addEventListener('load', () => {
                            track.track.mode = 'showing';
                        });
                    }

                    function populateSubtitleTracks() {
                        const subtitleSelectSettings = document.getElementById('subtitle-track-settings');
                        
                        if (!subtitleSelectSettings) return;
                        
                        subtitleSelectSettings.innerHTML = '<option value="">None</option>';
                        
                        const subtitleUrls = window.subtitleUrls || (metadata && metadata.subtitles);
                        
                        if (subtitleUrls) {
                            subtitleUrls.forEach((subtitle, idx) => {
                                const option = document.createElement('option');
                                option.value = idx;
                                const language = subtitle.language_full || subtitle.language || '';
                                const title = subtitle.title || subtitle.name || '';
                                option.textContent = `${language}${title ? ' - ' + title : ''}`;
                                if (subtitle.default || (idx === 0 && currentSubtitleIndex === null)) {
                                    option.selected = true;
                                    currentSubtitleIndex = idx;
                                    if (subtitle.url && currentVideo) {
                                        loadSubtitleFromUrl(subtitle.url, language, option.textContent);
                                    }
                                }
                                subtitleSelectSettings.appendChild(option);
                            });
                        }
                        
                        subtitleSelectSettings.addEventListener('change', async (e) => {
                            const newSubtitleIndex = e.target.value === '' ? null : parseInt(e.target.value);
                            if (newSubtitleIndex === currentSubtitleIndex) {
                                enableSubtitlesFromStream();
                                return;
                            }
                            
                            if (!presignedToken || !userToken) {
                                if (metadata && metadata.subtitles && newSubtitleIndex !== null) {
                                    const subtitle = metadata.subtitles[newSubtitleIndex];
                                    if (subtitle && subtitle.url) {
                                        currentSubtitleIndex = newSubtitleIndex;
                                        loadSubtitleFromUrl(
                                            subtitle.url,
                                            subtitle.language,
                                            `${subtitle.language_full || subtitle.language}${subtitle.title ? ' - ' + subtitle.title : ''}`
                                        );
                                        return;
                                    }
                                }
                                return;
                            }
                            
                            currentSubtitleIndex = newSubtitleIndex;
                            const subtitleParam = newSubtitleIndex !== null ? `&chosen_subtitle_index=${newSubtitleIndex}` : '';
                            const url = `https://api.torbox.app/v1/api/stream/getstreamdata?token=${encodeURIComponent(userToken)}&presigned_token=${encodeURIComponent(presignedToken)}${subtitleParam}&chosen_audio_index=${currentAudioIndex !== null ? currentAudioIndex : 0}`;
                            
                            try {
                                const response = await fetch(url, {
                                    headers: { 'Authorization': `Bearer ${userToken}` }
                                });
                                const data = await response.json();
                                if (data.success && data.data) {
                                    if (data.data.hls_url) {
                                        updateStreamUrl(data.data.hls_url);
                                    }
                                    if (data.data.subtitles && newSubtitleIndex !== null && data.data.subtitles[newSubtitleIndex]) {
                                        const subtitle = data.data.subtitles[newSubtitleIndex];
                                        if (subtitle.url) {
                                            loadSubtitleFromUrl(
                                                subtitle.url,
                                                subtitle.language,
                                                `${subtitle.language_full || subtitle.language}${subtitle.title ? ' - ' + subtitle.title : ''}`
                                            );
                                        }
                                    }
                                }
                            } catch (error) {
                                console.error('Failed to switch subtitle track:', error);
                                alert('Failed to switch subtitle track');
                            }
                        });
                    }

                    if (!streamUrl) {
                        document.getElementById('player-wrapper').innerHTML = 
                            '<div class="error">No stream URL provided</div>';
                    } else {
                        const video = document.createElement('video');
                        video.id = 'video-player';
                        video.autoplay = true;
                        video.playsInline = true;
                        currentVideo = video;
                        
                        const wrapper = document.getElementById('player-wrapper');
                        wrapper.innerHTML = '';
                        wrapper.appendChild(video);
                        
                        if (metadata) {
                            populateAudioTracks();
                            populateSubtitleTracks();
                        }
                        
                        const subtitleUrlsJson = urlParams.get('subtitle_urls');
                        if (subtitleUrlsJson) {
                            try {
                                const subtitleUrls = JSON.parse(decodeURIComponent(subtitleUrlsJson));
                                window.subtitleUrls = subtitleUrls;
                                if (!metadata && subtitleUrls && subtitleUrls.length > 0) {
                                    populateSubtitleTracks();
                                }
                            } catch (e) {
                                console.error('Failed to parse subtitle URLs:', e);
                            }
                        }

                        const playPauseBtn = document.getElementById('play-pause-btn');
                        const muteBtn = document.getElementById('mute-btn');
                        const fullscreenBtn = document.getElementById('fullscreen-btn');
                        const progressContainer = document.getElementById('progress-container');
                        const volumeSlider = document.getElementById('volume-slider');
                        const videoWrapper = document.getElementById('video-wrapper');
                        const playerContainer = document.getElementById('player-container');
                        const settingsMenu = document.getElementById('settings-menu');
                        const settingsBtn = document.getElementById('settings-btn');

                        playPauseBtn.addEventListener('click', () => {
                            if (currentVideo.paused) {
                                currentVideo.play();
                            } else {
                                currentVideo.pause();
                            }
                            showControls();
                        });

                        muteBtn.addEventListener('click', () => {
                            currentVideo.muted = !currentVideo.muted;
                            showControls();
                        });

                        fullscreenBtn.addEventListener('click', () => {
                            if (!document.fullscreenElement) {
                                playerContainer.requestFullscreen().catch(err => {
                                    console.error('Error attempting to enable fullscreen:', err);
                                });
                            } else {
                                document.exitFullscreen();
                            }
                            showControls();
                        });

                        settingsBtn.addEventListener('click', (e) => {
                            e.stopPropagation();
                            settingsMenu.classList.toggle('active');
                        });

                        document.addEventListener('click', (e) => {
                            if (!settingsMenu.contains(e.target)) {
                                settingsMenu.classList.remove('active');
                            }
                        });

                        document.addEventListener('fullscreenchange', () => {
                            const fullscreenIcon = document.getElementById('fullscreen-icon');
                            const exitIcon = document.getElementById('fullscreen-exit-icon');
                            if (document.fullscreenElement) {
                                fullscreenIcon.style.display = 'none';
                                exitIcon.style.display = 'block';
                            } else {
                                fullscreenIcon.style.display = 'block';
                                exitIcon.style.display = 'none';
                            }
                        });

                        progressContainer.addEventListener('click', (e) => {
                            if (!currentVideo) return;
                            const rect = progressContainer.getBoundingClientRect();
                            const percent = (e.clientX - rect.left) / rect.width;
                            currentVideo.currentTime = percent * currentVideo.duration;
                            showControls();
                        });

                        progressContainer.addEventListener('mousemove', (e) => {
                            if (!currentVideo) return;
                            const rect = progressContainer.getBoundingClientRect();
                            const percent = (e.clientX - rect.left) / rect.width;
                            const hoverBar = document.getElementById('progress-hover');
                            hoverBar.style.width = (percent * 100) + '%';
                            
                            if (isDragging) {
                                currentVideo.currentTime = percent * currentVideo.duration;
                            }
                        });

                        progressContainer.addEventListener('mousedown', (e) => {
                            isDragging = true;
                            if (!currentVideo) return;
                            const rect = progressContainer.getBoundingClientRect();
                            const percent = (e.clientX - rect.left) / rect.width;
                            currentVideo.currentTime = percent * currentVideo.duration;
                        });

                        let isDraggingVolume = false;

                        document.addEventListener('mouseup', () => {
                            isDragging = false;
                            isDraggingVolume = false;
                        });

                        volumeSlider.addEventListener('click', (e) => {
                            if (!currentVideo) return;
                            const rect = volumeSlider.getBoundingClientRect();
                            const percent = (e.clientX - rect.left) / rect.width;
                            currentVideo.volume = Math.max(0, Math.min(1, percent));
                            currentVideo.muted = false;
                            showControls();
                        });

                        volumeSlider.addEventListener('mousedown', (e) => {
                            if (!currentVideo) return;
                            isDraggingVolume = true;
                            e.preventDefault();
                            const rect = volumeSlider.getBoundingClientRect();
                            const percent = (e.clientX - rect.left) / rect.width;
                            currentVideo.volume = Math.max(0, Math.min(1, percent));
                            currentVideo.muted = false;
                            showControls();
                        });

                        document.addEventListener('mousemove', (e) => {
                            if (isDraggingVolume && currentVideo) {
                                const rect = volumeSlider.getBoundingClientRect();
                                const percent = (e.clientX - rect.left) / rect.width;
                                currentVideo.volume = Math.max(0, Math.min(1, percent));
                                currentVideo.muted = false;
                            }
                        });

                        video.addEventListener('loadedmetadata', () => {
                            updateProgress();
                            updateVolumeIcon();
                        });

                        video.addEventListener('timeupdate', updateProgress);
                        video.addEventListener('play', updatePlayPauseIcon);
                        video.addEventListener('pause', updatePlayPauseIcon);
                        video.addEventListener('volumechange', updateVolumeIcon);
                        video.addEventListener('click', showControls);

                        videoWrapper.addEventListener('mousemove', showControls);
                        videoWrapper.addEventListener('mouseleave', hideControls);

                        const volumeContainer = document.getElementById('volume-container');
                        const volumePercentage = document.getElementById('volume-percentage');
                        let volumeTimeout = null;

                        videoWrapper.addEventListener('wheel', (e) => {
                            if (!currentVideo) return;
                            
                            e.preventDefault();
                            
                            const delta = e.deltaY > 0 ? -0.05 : 0.05;
                            const newVolume = Math.max(0, Math.min(1, currentVideo.volume + delta));
                            
                            currentVideo.volume = newVolume;
                            currentVideo.muted = newVolume === 0;
                            
                            if (volumeContainer) {
                                volumeContainer.classList.add('show-slider', 'show-percentage');
                                const percentage = Math.round(newVolume * 100);
                                if (volumePercentage) {
                                    volumePercentage.textContent = `${percentage}%`;
                                }
                            }
                            
                            clearTimeout(volumeTimeout);
                            volumeTimeout = setTimeout(() => {
                                if (volumeContainer) {
                                    volumeContainer.classList.remove('show-slider', 'show-percentage');
                                }
                            }, 1500);
                            
                            showControls();
                        }, { passive: false });

                        if (Hls.isSupported()) {
                            hlsInstance = new Hls({
                                enableWorker: true,
                                lowLatencyMode: false,
                            });
                            hlsInstance.loadSource(streamUrl);
                            hlsInstance.attachMedia(video);
                            
                            hlsInstance.on(Hls.Events.MANIFEST_PARSED, () => {
                                console.log('HLS manifest parsed, starting playback');
                                enableSubtitlesFromStream();
                            });
                            
                            hlsInstance.on(Hls.Events.SUBTITLE_TRACKS_UPDATED, () => {
                                enableSubtitlesFromStream();
                            });
                            
                            hlsInstance.on(Hls.Events.ERROR, (event, data) => {
                                console.error('HLS error:', data);
                                if (data.fatal) {
                                    switch (data.type) {
                                        case Hls.ErrorTypes.NETWORK_ERROR:
                                            wrapper.innerHTML = '<div class="error">Network error. Please check your connection and try again.</div>';
                                            break;
                                        case Hls.ErrorTypes.MEDIA_ERROR:
                                            wrapper.innerHTML = '<div class="error">Media error. The stream may be unavailable.</div>';
                                            break;
                                        default:
                                            wrapper.innerHTML = '<div class="error">Stream error. Please try again later.</div>';
                                            break;
                                    }
                                }
                            });
                        } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
                            video.src = streamUrl;
                            video.addEventListener('error', (e) => {
                                console.error('Video error:', e);
                                wrapper.innerHTML = '<div class="error">Error loading stream. Please check the URL.</div>';
                            });
                        } else {
                            wrapper.innerHTML = '<div class="error">Your browser does not support HLS streaming. Please use a modern browser.</div>';
                        }
                    }

                    document.addEventListener('keydown', (e) => {
                        const video = document.getElementById('video-player');
                        if (!video) return;
                        
                        switch(e.key) {
                            case ' ':
                                e.preventDefault();
                                if (video.paused) {
                                    video.play();
                                } else {
                                    video.pause();
                                }
                                showControls();
                                break;
                            case 'f':
                            case 'F':
                                e.preventDefault();
                                if (!document.fullscreenElement) {
                                    document.getElementById('player-container').requestFullscreen();
                                } else {
                                    document.exitFullscreen();
                                }
                                showControls();
                                break;
                            case 'm':
                            case 'M':
                                e.preventDefault();
                                video.muted = !video.muted;
                                showControls();
                                break;
                            case 'ArrowLeft':
                                e.preventDefault();
                                video.currentTime -= 10;
                                showControls();
                                break;
                            case 'ArrowRight':
                                e.preventDefault();
                                video.currentTime += 10;
                                showControls();
                                break;
                            case 'ArrowUp':
                                e.preventDefault();
                                video.volume = Math.min(1, video.volume + 0.1);
                                video.muted = false;
                                showControls();
                                break;
                            case 'ArrowDown':
                                e.preventDefault();
                                video.volume = Math.max(0, video.volume - 0.1);
                                showControls();
                                break;
                            case 'Escape':
                                if (document.fullscreenElement) {
                                    document.exitFullscreen();
                                }
                                break;
                        }
                    });
                    "#}
            </script>
        </div>
    }
}

