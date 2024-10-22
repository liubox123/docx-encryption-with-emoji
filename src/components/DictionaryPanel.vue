<template>
  <div class="left-panel">
    <h2>Replacement Dictionary</h2>
    <div class="file-selection">
      <button @click="selectDictionaryFile">Select Dictionary File</button>
    </div>
    <div class="dictionary-table-container">
      <table class="dictionary-table">
        <thead>
          <tr>
            <th>Find</th>
            <th>Replace With</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="replacement in replacements" :key="replacement.id">
            <td>{{ replacement.find }}</td>
            <td>{{ replacement.replace }}</td>
            <td>
              <button class="delete-btn" @click="deleteReplacement(replacement.id)">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="add-replacement">
      <input v-model="newFind" placeholder="Find" @input="autoFillReplace" />
      <input v-model="newReplace" placeholder="Replace With" />
      <button @click="addReplacement">Add</button>
    </div>
  </div>
</template>
  
<script>
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from 'element-plus';
import allEmojis from '../assets/emojis.json';
import { open } from "@tauri-apps/plugin-dialog";

export default {
  name:'DictionaryPanel',
  data() {
    return {
      replacements: [],
      newFind: "",
      newReplace: ""
    };
  },
  mounted() {
    this.getReplacements();
  },
  methods: {
    async getReplacements() {
      this.replacements = await invoke("get_replacements");
    },
    async addReplacement() {
      if (!this.newFind || !this.newReplace) return;

      try {
        const result = await invoke("add_replacement", { find: this.newFind, replace: this.newReplace });
        if (result.startsWith("替换项添加成功")) {
          this.getReplacements();
        } else {
          ElMessage.error(result);
        }
      } catch (e) {
        ElMessage.error(e);
      }
    },
    async deleteReplacement(id) {
      await invoke("delete_replacement", { id });
      this.getReplacements();
    },
    generateUniqueEmojiString(length) {
      let result = "";
      const usedEmojis = new Set(this.replacements.map(r => r.replace));

      while (Array.from(result).length < length) {
        const randomEmoji = allEmojis[Math.floor(Math.random() * allEmojis.length)];
        if (!usedEmojis.has(randomEmoji)) {
          result += randomEmoji;
        }
      }

      if (Array.from(result).length !== length) {
        return this.generateUniqueEmojiString(length);
      }

      return result;
    },
    logUnicodeCodePoints(str) {
      const codePoints = Array.from(str).map(char => char.codePointAt(0).toString(16));
      console.log(`Unicode code points: ${codePoints.join(' ')}`);
    },
    autoFillReplace() {
      const length = Array.from(this.newFind).length;
      this.newReplace = this.generateUniqueEmojiString(length);
      this.logUnicodeCodePoints(this.newFind);
      this.logUnicodeCodePoints(this.newReplace);
    },
    async selectDictionaryFile() {
      try {
        const selectedFile = await open({
          filters: [{ name: 'SQLite Files', extensions: ['db', 'sqlite'] }]
        });
        if (selectedFile) {
          const result = await invoke('handle_dictionary_file', { filePath: selectedFile });
          console.log(result);
          this.getReplacements();
        }
      } catch (error) {
        console.error('Error selecting file:', error);
      }
    }
  }
};
</script>
  
<style scoped>
.left-panel {
  /* padding: 20px; */
  border-radius: 8px;
  background-color: var(--background-color);
  height: 100vh; /* 使面板填充窗口高度 */
  display: flex;
  flex-direction: column;
  box-sizing: border-box; /* 确保 padding 不影响总高度 */
  overflow: hidden; /* 防止面板本身出现滚动条 */
}

h2 {
  color: var(--text-color);
  margin-bottom: 10px;
}

.file-selection {
  margin-bottom: 20px;
}

.dictionary-table-container {
  flex: 1;
  overflow-y: auto; /* 允许垂直滚动 */
  overflow-x: auto; /* 允许水平滚动 */
}

.dictionary-table {
  width: 100%;
  border-collapse: collapse;
  table-layout: fixed; /* 固定表格布局 */
}

.dictionary-table th {
  position: sticky;
  top: 0;
  background-color: var(--background-color);
  z-index: 1;
  color: var(--text-color);
}

.dictionary-table th, .dictionary-table td {
  border: 1px solid var(--border-color);
  padding: 8px;
  text-align: center;
}

.add-replacement {
  margin-top: 20px;
  display: flex;
  gap: 10px;
  flex-shrink: 0; /* 防止输入区域被压缩 */
}

.add-replacement input {
  flex: 1;
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
}

button {
  padding: 10px 20px;
  border: none;
  color: var(--text-color);
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover {
  background-color: var(--button-hover-color);
}

.delete-btn {
  background-color: #dc3545;
}

.delete-btn:hover {
  background-color: #c82333;
}
</style>
