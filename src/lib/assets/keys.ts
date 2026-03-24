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
} as const;

export const FORMATS = {
    VIDEO: "video",
    AUDIO: "audio",
} as const;
