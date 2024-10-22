import { createRouter, createWebHistory } from 'vue-router';
import DictionaryPanel from '../components/DictionaryPanel.vue';
import FilePanel from '../components/FilePanel.vue';
import TextReplacement from '../components/TextReplacement.vue';
const routes = [
  { path: '/dictionary', component: DictionaryPanel },
  { path: '/file', component: FilePanel },
  { path: '/text-replacement', component: TextReplacement }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;