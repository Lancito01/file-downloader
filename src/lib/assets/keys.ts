export const SETTINGS_KEYS = {
    ACTIVE_TAB: {
        id: "activeTab",
        text: "Active Tab",
    },
    MUSIC_FOLDER_PATH: {
        id: "musicFolderPath",
        text: "Music Folder Path",
    },
    DEFAULT_FORMAT: {
        id: "defaultFormat",
        text: "Default Download Format",
    },
    EMBED_METADATA: {
        id: "embedMetadata",
        text: "Embed artwork and metadata",
    },
    DEFAULT_AUDIO_EXTENSION: {
        id: "defaultAudioExtension",
        text: "Default Audio Extension",
    },
    DEFAULT_VIDEO_EXTENSION: {
        id: "defaultVideoExtension",
        text: "Default Video Extension",
    },
} as const;

export const FORMATS = {
    VIDEO: "video",
    AUDIO: "audio",
} as const;
