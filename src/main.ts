import { createApp } from 'vue';
import App from './App.vue';
import router from './router';

import 'remixicon/fonts/remixicon.css';
import './assets/styles/styles.css';
import './assets/styles/global.scss';

createApp(App).use(router).mount('#app');
