import { createApp } from 'vue';
import App from './App.vue';
import router from './router';

import 'element-plus/theme-chalk/el-message.css';
import 'remixicon/fonts/remixicon.css';
import './assets/styles/styles.css';
import './assets/styles/global.scss';

createApp(App).use(router).mount('#app');
