<template>
  <div class="app-container">
    <div class="sidebar">
      <select v-model="currentLanguage" @change="changeLanguage" class="language-selector">
        <option value="zh">中文</option>
        <option value="en">English</option>
      </select>
      <ul>
        <li><router-link to="/dictionary">{{ $t('dictionary') }}</router-link></li>
        <li><router-link to="/file">{{ $t('file') }}</router-link></li>
        <li><router-link to="/text-replacement">{{ $t('textReplacement') }}</router-link></li>
      </ul>
    </div>
    <div style="flex: 1;padding: 0px; display: flex;">
      <router-view></router-view>
    </div>
  </div>
</template>

<script>
import './styles/theme.css';
import { useI18n } from 'vue-i18n';

export default {
  name: 'App',
  setup() {
    const { t, locale } = useI18n();
    return { t, locale };
  },
  data() {
    return {
      currentTheme: 'default',
      currentLanguage: 'zh'
    };
  },
  methods: {
    toggleTheme() {
      this.currentTheme = this.currentTheme === 'default' ? 'modern' : 'default';
      document.documentElement.setAttribute('data-theme', this.currentTheme);

      // 更新CSS变量
      const root = document.documentElement;
      if (this.currentTheme === 'modern') {
        root.style.setProperty('--background-color', '#f0f0f0');
        root.style.setProperty('--text-color', '#000'); // 将文字颜色设置为黑色
        root.style.setProperty('--border-color', '#ccc');
        root.style.setProperty('--button-hover-color', '#004d40');
        root.style.setProperty('--select-file-btn-bg', '#00796b');
        root.style.setProperty('--select-file-btn-hover-bg', '#004d40');
        root.style.setProperty('--process-btn-bg', '#4db6ac');
        root.style.setProperty('--process-btn-hover-bg', '#00897b');
        root.style.setProperty('--reverse-process-btn-bg', '#80cbc4');
        root.style.setProperty('--reverse-process-btn-hover-bg', '#26a69a');
      } else {
        root.style.setProperty('--background-color', '#ffffff');
        root.style.setProperty('--text-color', '#000');
        root.style.setProperty('--border-color', '#ccc');
        root.style.setProperty('--button-hover-color', '#004d40');
        root.style.setProperty('--select-file-btn-bg', '#00796b');
        root.style.setProperty('--select-file-btn-hover-bg', '#004d40');
        root.style.setProperty('--process-btn-bg', '#4db6ac');
        root.style.setProperty('--process-btn-hover-bg', '#00897b');
        root.style.setProperty('--reverse-process-btn-bg', '#80cbc4');
        root.style.setProperty('--reverse-process-btn-hover-bg', '#26a69a');
      }
    },
    changeLanguage() {
      this.locale = this.currentLanguage;
    }
  }
};
</script>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
}

.sidebar {
  width: 150px;
  padding: 15px;
  box-shadow: 2px 0 5px rgba(0, 0, 0, 0.1);
}

.sidebar ul {
  list-style-type: none;
  padding: 0;
}

.sidebar li {
  margin-bottom: 8px;
}

.sidebar a {
  text-decoration: none;
  color: var(--text-color);
  font-weight: bold;
  display: block;
  padding: 8px;
  border-radius: 4px;
  transition: background-color 0.3s;
}

.theme-switcher {
  margin-top: 20px;
}

button {
  padding: 10px 20px;
  border: none;
  color: var(--text-color);
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.language-selector {
  width: 100%;
  padding: 8px;
  margin-bottom: 15px;
  border-radius: 4px;
  border: 1px solid var(--border-color);
  background-color: var(--background-color);
  color: var(--text-color);
}
</style>
