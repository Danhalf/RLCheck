import { createApp } from 'vue';
import './styles.css';
import App from './App.vue';
import PrimeVue from 'primevue/config';

import 'primevue/resources/themes/tailwind-light/theme.css';
import 'primevue/resources/primevue.min.css';
import 'primeicons/primeicons.css';

const app = createApp(App);
app.use(PrimeVue);
app.mount('#app');
