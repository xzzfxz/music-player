import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { createPinia } from 'pinia';
import { createPersistedState } from 'pinia-plugin-persistedstate';

import 'element-plus/theme-chalk/el-message.css';
import 'remixicon/fonts/remixicon.css';
import './assets/styles/styles.css';
import './assets/styles/global.scss';

const pinia = createPinia();
pinia.use(
  createPersistedState({
    key: id => `__persisted__music__${id}`
  })
);

createApp(App).use(router).use(pinia).mount('#app');
