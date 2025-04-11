/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_TITLE: string
  readonly TAURI_DEBUG: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
} 