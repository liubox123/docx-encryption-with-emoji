<template>
  <div class="left-panel">
    <h2>{{ $t('replacementDictionary') }}</h2>
    <div class="file-selection">
      <button @click="selectDictionaryFile">{{ $t('selectDictionaryFile') }}</button>
      <button @click="showBatchAddDialog = true">{{ $t('batchAdd') }}</button>
    </div>
    <div class="dictionary-table-container">
      <table class="dictionary-table">
        <thead>
          <tr>
            <th>{{ $t('find') }}</th>
            <th>{{ $t('replaceWith') }}</th>
            <th>{{ $t('action') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="replacement in replacements" :key="replacement.id">
            <td>{{ replacement.find }}</td>
            <td>{{ replacement.replace }}</td>
            <td>
              <button class="delete-btn" @click="deleteReplacement(replacement.id)">{{ $t('delete') }}</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="add-replacement">
      <input v-model="newFind" :placeholder="$t('find')" @input="autoFillReplace" />
      <input v-model="newReplace" :placeholder="$t('replaceWith')" />
      <button @click="addReplacement">{{ $t('add') }}</button>
    </div>
    <div v-if="showBatchAddDialog" class="overlay">
      <div class="batch-add-dialog">
        <div class="input-container" contenteditable="true" @input="handleInput" ref="inputContainer">
        </div>
        <textarea v-model="batchReplaceText" :placeholder="$t('generatedEmojis')" readonly></textarea>
        <button @click="batchAddReplacements">{{ $t('submit') }}</button>
        <button @click="showBatchAddDialog = false">{{ $t('cancel') }}</button>
      </div>
    </div>
  </div>
</template>
  
<script>
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from 'element-plus';
import allEmojis from '../assets/emojis.json';
import { open } from "@tauri-apps/plugin-dialog";
import { useI18n } from 'vue-i18n';

export default {
  name:'DictionaryPanel',
  setup() {
    const { t } = useI18n();
    return { t };
  },
  data() {
    return {
      replacements: [],
      newFind: "",
      newReplace: "",
      showBatchAddDialog: false,
      batchAddText: "",
      batchAddArray: [],
      batchReplaceText: "",
      hasError: false
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
        if (result.startsWith(this.t('replacementAddedSuccessfully'))) {
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
    },
    handleInput() {
      const inputText = this.$refs.inputContainer.innerText.trim();
      const words = inputText.split(' ');
      this.batchAddArray = words.map(word => ({
        text: word,
        error: false
      }));
      this.autoFillBatchReplace();
    },
    autoFillBatchReplace() {
      this.batchReplaceText = this.batchAddArray.map(word => this.generateUniqueEmojiString(Array.from(word.text).length)).join(' ');
    },
    async batchAddReplacements() {
      let hasError = false;
      for (let i = 0; i < this.batchAddArray.length; i++) {
        const word = this.batchAddArray[i];
        if (word.text && this.batchReplaceText.split(' ')[i]) {
          try {
            const result = await this.addReplacement(word.text, this.batchReplaceText.split(' ')[i]);
            if (!result.startsWith(this.t('replacementAddedSuccessfully'))) {
              word.error = true;
              hasError = true;
            } else {
              word.text = ''; // 清空成功的项
            }
          } catch (e) {
            word.error = true;
            hasError = true;
          }
        }
      }
      this.batchAddArray = this.batchAddArray.filter(word => word.text !== ''); // 过滤掉成功的项
      this.updateInputContainer();
      if (!hasError) {
        this.batchReplaceText = ''; // 清空 emoji 框
      }
      this.getReplacements();
    },
    updateInputContainer() {
      this.$refs.inputContainer.innerHTML = this.batchAddArray.map(word => 
        `<span style="background-color: ${word.error ? 'red' : 'transparent'};">${word.text}</span>`
      ).join(' ');
    },
    async addReplacement(find, replace) {
      try {
        const result = await invoke("add_replacement", { find, replace });
        return result;
      } catch (e) {
        ElMessage.error(e);
        throw e;
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

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000; /* 确保浮动窗体在最上层 */
}

.batch-add-dialog {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  width: 600px; /* 增加宽度 */
  height: 400px; /* 增加高度 */
  max-width: 90%;
  display: flex;
  flex-direction: column;
  gap: 10px;
  z-index: 1001; /* 确保对话框在 overlay 之上 */
}

.input-container {
  flex: 1;
  width: 96%;
  padding: 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  min-height: 50px;
  overflow-y: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
}

textarea {
  flex: 1;
  width: 96%;
  padding: 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  resize: none; /* 禁止调整大小 */
}

.batch-replace-container {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
}
</style>
