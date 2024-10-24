<template>
  <div class="text-replacement">
    <h2>{{ $t('textReplacement') }}</h2>
    <textarea v-model="inputText" :placeholder="$t('enterTextHere')"></textarea>
    <div class="button-container">
      <button @click="replaceText">{{ $t('replace') }}</button>
      <button @click="reverseReplaceText">{{ $t('reverseReplace') }}</button>
      <button @click="convertToUnicode">{{ $t('convertToUnicode') }}</button>
      <button @click="unicodeToTextAndReverseReplace">{{ $t('unicodeToTextAndReverseReplace') }}</button>
      <button @click="copyToClipboard">{{ $t('copyResultToClipboard') }}</button>
    </div>
    <textarea v-model="outputText" :placeholder="$t('replacedTextWillAppearHere')" readonly></textarea>
  </div>
</template>

<script>
import { useI18n } from 'vue-i18n';
import { invoke } from "@tauri-apps/api/core";

export default {
  setup() {
    const { t } = useI18n();
    return { t };
  },
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
    async convertToUnicode() {
      this.outputText = Array.from(this.inputText).map(char => `\\u${char.charCodeAt(0).toString(16).padStart(4, '0')}`).join(' ');
    },
    async unicodeToTextAndReverseReplace() {
      try {
        this.outputText = await invoke("unicode_to_text_and_reverse", { input: this.inputText });
      } catch (error) {
        console.error("Error converting unicode to text and reversing:", error);
      }
    },
    copyToClipboard() {
      navigator.clipboard.writeText(this.outputText).then(() => {
        alert(this.t('textCopiedToClipboard'));
      }).catch(err => {
        console.error(this.t('failedToCopyText'), err);
      });
    }
  }
};
</script>

<style scoped>
.text-replacement {
  padding: 0px;
  border: 0px solid var(--border-color);
  border-radius: 8px;
  background-color: var(--background-color);
  width: 100%;
  height: 100%;
  box-sizing: border-box;
}

textarea {
  width: 100%;
  height: 30%;
  margin-bottom: 20px;
  padding: 0px;
  border: 0px solid var(--border-color);
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
