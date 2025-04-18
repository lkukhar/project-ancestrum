import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './index.css'
import tauri from './plugins/tauri'

const app = createApp(App)

app.use(createPinia())
app.use(tauri)

app.mount('#app') 