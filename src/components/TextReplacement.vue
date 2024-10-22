<template>
  <div class="text-replacement">
    <h2>Text Replacement</h2>
    <textarea v-model="inputText" placeholder="Enter text here..."></textarea>
    <div class="button-container">
      <button @click="replaceText">Replace</button>
      <button @click="reverseReplaceText">Reverse Replace</button>
      <button @click="copyToClipboard">Copy Result to Clipboard</button>
    </div>
    <textarea v-model="outputText" placeholder="Replaced text will appear here..." readonly></textarea>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";

export default {
  data() {
    return {
      inputText: '',
      outputText: ''
    };
  },
  methods: {
    async replaceText() {
      try {
        this.outputText = await invoke("replace_text_with_db", { input: this.inputText, reverse: false });
      } catch (error) {
        console.error("Error replacing text:", error);
      }
    },
    async reverseReplaceText() {
      try {
        this.outputText = await invoke("replace_text_with_db", { input: this.inputText, reverse: true });
      } catch (error) {
        console.error("Error reversing text:", error);
      }
    },
    copyToClipboard() {
      navigator.clipboard.writeText(this.outputText).then(() => {
        alert('Text copied to clipboard!');
      }).catch(err => {
        console.error('Failed to copy text: ', err);
      });
    }
  }
};
</script>

<style scoped>
.text-replacement {
  padding: 20px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--background-color);
}

textarea {
  width: 100%;
  height: 150px;
  margin-bottom: 20px;
  padding: 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  resize: none;
}

.button-container {
  display: flex;
  justify-content: space-between;
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
</style>
