<template>
  <div class="app-container">
    <div class="sidebar">
      <ul>
        <li><router-link to="/dictionary">Dictionary</router-link></li>
        <li><router-link to="/file">File</router-link></li>
        <li><router-link to="/text-replacement">Text Replacement</router-link></li>
      </ul>
    </div>
    <div class="content">
      <router-view></router-view>
    </div>
  </div>
</template>

<script>
import './styles/theme.css';

export default {
  name: 'App',
  data() {
    return {
      currentTheme: 'default'
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
</style>
