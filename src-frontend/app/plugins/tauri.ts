import { invoke } from '@tauri-apps/api/tauri';

export default {
  install: (app: any) => {
    app.config.globalProperties.$tauri = {
      invoke,
    };
  },
}; 