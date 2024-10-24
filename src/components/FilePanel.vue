<template>
  <div class="right-panel">
    <h2>File Processing</h2>
    <div class="button-container">
      <button class="select-file-btn" @click="selectFile">Select File</button>
      <button class="process-btn" @click="processFile(false)" :disabled="!selectedFile">Execute Replace and Save</button>
      <button class="reverse-process-btn" @click="processFile(true)" :disabled="!selectedFile">Reverse Replace and Save</button>
    </div>
    <div v-if="selectedFile" class="file-preview">
      <p>Selected File: {{ selectedFile }}</p>
      <div ref="docxContainer" class="docx-container"></div>
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import { renderAsync } from 'docx-preview';

export default {
  data() {
    return {
      selectedFile: null,
      fileContent: null
    };
  },
  mounted() {
    window.addEventListener('resize', this.handleResize);
  },
  beforeDestroy() {
    window.removeEventListener('resize', this.handleResize);
  },
  methods: {
    async selectFile() {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'DOCX', extensions: ['docx'] }]
      });
      if (selected) {
        this.selectedFile = selected;
        this.loadFileContent();
      }
    },
    async loadFileContent() {
      try {
        const binaryData = await readFile(this.selectedFile);
        this.fileContent = binaryData;
        await this.renderDocx(binaryData);
      } catch (error) {
        console.error("Error loading file content:", error);
      }
    },
    async processFile(reverse) {
      try {
        const newFilePath = await invoke("process_docx", { filePath: this.selectedFile, reverse });
        alert(`文件已保存为: ${newFilePath}`);
        this.selectedFile = newFilePath;
        await this.loadFileContent();
      } catch (error) {
        console.error("Error processing file:", error);
      }
    },
    async renderDocx(binaryData) {
      try {
        const docxContainer = this.$refs.docxContainer;
        if (docxContainer) {
          docxContainer.innerHTML = "";
          await renderAsync(binaryData, docxContainer, null, {
            className: 'docx',
            inWrapper: false,
            ignoreWidth: true,
            ignoreHeight: true
          });
          this.adjustDocxSize();
        } else {
          console.error("docxContainer ref is not available.");
        }
      } catch (error) {
        console.error("Error rendering DOCX:", error);
      }
    },
    handleResize() {
      if (this.fileContent) {
        this.adjustDocxSize();
      }
    },
    adjustDocxSize() {
      const docxContainer = this.$refs.docxContainer;
      if (docxContainer) {
        const scale = docxContainer.clientWidth / docxContainer.scrollWidth;
        docxContainer.style.transform = `scale(${scale})`;
        docxContainer.style.transformOrigin = 'top left';
      }
    }
  }
};
</script>

<style scoped>
.right-panel {
  flex: 1;
  /* flex-grow: 1; */
  border: 0px solid var(--border-color);
  padding: 0px;
  border-radius: 8px;
  background-color: var(--background-color);
  height: 100%;
  /* 移除 overflow: hidden; */
}

.button-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
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

.select-file-btn {
  background-color: var(--select-file-btn-bg);
}

.select-file-btn:hover {
  background-color: var(--select-file-btn-hover-bg);
}

.process-btn {
  background-color: var(--process-btn-bg);
}

.process-btn:hover {
  background-color: var(--process-btn-hover-bg);
}

.reverse-process-btn {
  background-color: var(--reverse-process-btn-bg);
}

.reverse-process-btn:hover {
  background-color: var(--reverse-process-btn-hover-bg);
}

.file-preview {
  margin-top: 20px;
  text-align: center;
  overflow: hidden;
}

.docx-container {
  max-width: 100%;
  height: 90%; /* 设置一个固定高度 */
  overflow: auto; /* 允许滚动 */
}
</style>
