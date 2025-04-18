import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './index.css'

// Initialize Tauri API
if (window.__TAURI__) {
  console.log('Tauri API initialized')
}

const app = createApp(App)
app.use(createPinia())
app.mount('#app') 